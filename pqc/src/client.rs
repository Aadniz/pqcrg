use super::crypter;
use base64::{engine::general_purpose, Engine as _};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    io::{self, ErrorKind, Read},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    result::Result,
    sync::{Arc, Mutex},
    thread,
};

use oqs::*;

type Connections = HashMap<IpAddr, kem::SharedSecret>;

/// The `Client` struct represents a client in a client-server model.
pub struct Client {
    source_addr: Arc<Mutex<Option<SocketAddr>>>,
    socket: UdpSocket,
    passer: UdpSocket,
    connections: Arc<Mutex<Connections>>,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new() -> Client {
        // Binding to 0.0.0.0:0 means it is random
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let socket2 = socket.try_clone().unwrap();
        let passer = UdpSocket::bind(format!("0.0.0.0:{}", super::BRIDGE_PORT)).unwrap();
        let passer2 = passer.try_clone().unwrap();
        let connections: Arc<Mutex<Connections>> = Arc::new(Mutex::new(HashMap::new()));
        let connections_clone = Arc::clone(&connections);
        let source_addr: Arc<Mutex<Option<SocketAddr>>> = Arc::new(Mutex::new(None));
        let source_addr_clone = Arc::clone(&source_addr);

        thread::spawn(move || {
            let mut buf = [0; 1024];
            loop {
                let (amt, addr) = match socket2.recv_from(&mut buf) {
                    Ok((amt, addr)) => (amt, addr),
                    Err(e) => {
                        eprintln!("Failed to receive data: {}", e);
                        continue;
                    }
                };

                let shared_secret = {
                    let connections_lock = connections_clone.lock().unwrap();

                    match connections_lock.get(&addr.ip()) {
                        Some(shared_secret) => shared_secret.clone(),
                        None => {
                            eprintln!("No shared secret for peer: {}", &addr);
                            continue;
                        }
                    }
                };

                let nonce = &buf[..12];
                let ciphertext = &buf[12..amt];
                match crypter::decrypt(shared_secret.clone(), nonce, ciphertext) {
                    Ok(data) => {
                        // Convert the bytes to a string
                        let data = match String::from_utf8(data.to_vec()) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed to convert data to string: {}", e);
                                continue;
                            }
                        };

                        // Forward the received data to the destination
                        let addr: &Option<SocketAddr> =
                            { &source_addr_clone.lock().unwrap().clone() };
                        if let Some(addr) = addr {
                            if let Err(e) = passer2.send_to(&data.as_bytes(), addr) {
                                eprintln!("Failed to send data: {}", e);
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to decrypt data: {}", e),
                }
            }
        });

        Client {
            source_addr,
            socket,
            passer,
            connections,
        }
    }

    pub fn pass(&mut self, ip: IpAddr, port: u16) {
        let mut buf = [0; 1024];

        loop {
            let (amt, peer_addr) = match self.passer.recv_from(&mut buf) {
                Ok((amt, addr)) => (amt, addr),
                Err(e) => {
                    eprintln!("Failed to receive from UDP socket: {}", e);
                    continue;
                }
            };

            {
                let mut source_addr = self.source_addr.lock().unwrap();
                if source_addr.is_none() {
                    if peer_addr.port() != 0 {
                        *source_addr = Some(peer_addr);
                    }
                }
            }

            // Foorwards the data
            if let Err(e) = self.send(ip, buf[..amt].to_vec()) {
                eprintln!("{e}");
            };
        }
    }

    /// Sends a message to a specified IP address.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address to send the message to.
    /// * `msg` - The message to send.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    fn send(&mut self, ip: IpAddr, msg: Vec<u8>) -> Result<(), Box<dyn Error + '_>> {
        let addr = SocketAddr::new(ip, super::BRIDGE_PORT);
        let mut shared_secret = None;

        {
            let connections_lock = self.connections.lock().unwrap();
            if let Some(secret) = connections_lock.get(&ip) {
                shared_secret = Some(secret.clone());
            }
        }

        if shared_secret.is_none() {
            shared_secret = Some(self.handshake(ip)?);
        }

        let shared_secret = shared_secret.unwrap();

        let (mut nonce, ciphertext) = crypter::encrypt(shared_secret.clone(), &msg)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
        nonce.extend(ciphertext);
        self.socket.send_to(&nonce, addr)?;

        Ok(())
    }

    /// Performs a handshake with a specified IP address.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP address to perform the handshake with.
    ///
    /// # Errors
    ///
    /// Returns an error if the handshake cannot be performed.
    fn handshake(&mut self, ip: IpAddr) -> Result<kem::SharedSecret, Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::BRIDGE_PORT);
        println!("New connection to {}, exchanging keys", addr);

        let kemalg = kem::Kem::new(kem::Algorithm::Kyber768).map_err(to_io_error)?;
        let (kem_pk, kem_sk) = kemalg.keypair().map_err(to_io_error)?;

        let mut stream = TcpStream::connect(addr)?;
        let data = kem_pk.into_vec();
        println!("Size of {}", data.len());
        println!("Sent: {}", base64_vec(&data));
        stream.write_all(&data)?;

        let mut buf = [0; 1088];
        stream.read_exact(&mut buf)?;
        let data2 = buf;
        println!("Received: {}", base64_vec(&data2.to_vec()));
        let kem_ct = kemalg
            .ciphertext_from_bytes(&data2)
            .ok_or("No ciphered text was generated")?;

        let kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct).map_err(to_io_error)?;
        {
            self.connections.lock().unwrap().insert(ip, kem_ss.clone());
        }

        println!("Shared key is: {}", base64_vec(&kem_ss.clone().into_vec()));

        Ok(kem_ss)
    }
}

/// Converts an `oqs::Error` to an `io::Error`.
///
/// # Arguments
///
/// * `e` - The `oqs::Error` to convert.
fn to_io_error(e: oqs::Error) -> io::Error {
    io::Error::new(ErrorKind::Other, e.to_string())
}

/// Encodes a vector of bytes to a base64 string.
///
/// # Arguments
///
/// * `data` - The data to encode.
fn base64_vec(data: &Vec<u8>) -> String {
    return general_purpose::STANDARD.encode(data);
}
