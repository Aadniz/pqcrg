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
};

use oqs::{kem::SharedSecret, *};

/// The `Client` struct represents a client in a client-server model.
pub struct Client {
    socket: UdpSocket,
    passer: Option<UdpSocket>,
    connections: Arc<Mutex<HashMap<IpAddr, kem::SharedSecret>>>,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new() -> Client {
        // Binding to 0.0.0.0:0 means it is random
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        Client {
            socket,
            passer: None,
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn pass(&mut self, ip: IpAddr, port: u16) {
        if !self.passer.is_none() {
            eprint!("Passer already set!");
            return;
        }

        let udp_socket = match UdpSocket::bind(format!("127.0.0.1:{}", super::UDP_PORT)) {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Could not bind port {}: {}", super::UDP_PORT, e);
                return;
            }
        };

        self.passer = Some(udp_socket);
        let mut buf = [0; 1024];

        loop {
            let (amt, peer_addr) = match self.passer.as_ref().unwrap().recv_from(&mut buf) {
                Ok((amt, addr)) => (amt, addr),
                Err(e) => {
                    eprintln!("Failed to receive from UDP socket: {}", e);
                    continue;
                }
            };

            // Foorwards the data
            match self.send(ip, buf[..amt].to_vec()) {
                Ok(_) => (),
                Err(e) => eprintln!("{e}"),
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
        let addr = SocketAddr::new(ip, super::UDP_PORT);
        let connections = Arc::clone(&self.connections);
        let shared_secret = match { connections.lock()? }.get(&ip) {
            Some(shared_secret) => shared_secret.clone(),
            None => self.handshake(ip)?.clone(),
        };

        let shared_secret = shared_secret.clone();
        let (nonce, ciphertext) = crypter::encrypt(shared_secret, &msg)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
        let mut data = nonce;
        data.extend(ciphertext);
        self.socket.send_to(&data, addr)?;

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
    fn handshake(&mut self, ip: IpAddr) -> Result<SharedSecret, Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::TCP_PORT);
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
            let connetions: Arc<Mutex<HashMap<IpAddr, kem::SharedSecret>>> =
                Arc::clone(self.connections);
            connections.lock()?.insert(ip, kem_ss.clone());
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
