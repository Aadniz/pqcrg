use base64::{engine, Engine as _};
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs8::LineEnding;
use rsa::{pkcs1::EncodeRsaPublicKey, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::error::Error;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream, UdpSocket},
};

/// A type alias for a map of IP addresses to shared secrets.
type Connections = HashMap<IpAddr, RsaPublicKey>;

pub struct Server {
    secret_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    connections: Mutex<Connections>,
}

impl Server {
    pub fn new() -> Arc<Self> {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let secret_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&secret_key);
        Arc::new(Server {
            secret_key,
            public_key,
            connections: Mutex::new(HashMap::new()),
        })
    }

    /// Starts the server and listens for incoming connections.
    pub fn listen(self: &Arc<Self>) {
        println!(
            "Server listening on TCP 127.0.0.1:{} and UDP 127.0.0.1:{}",
            super::TCP_PORT,
            super::UDP_PORT
        );

        let self_arc = Arc::new(self.clone());
        let self_arc_clone = Arc::clone(&self_arc);

        thread::spawn(move || self_arc_clone.listen_tcp());
        self_arc.listen_udp();
    }

    /// Listens for incoming TCP connections.
    ///
    /// # Arguments
    ///
    /// * `connections` - A shared, mutable reference to the map of connections.
    fn listen_tcp(self: &Arc<Self>) {
        let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", super::TCP_PORT))
            .expect(&format!("Could not bind port {}", super::TCP_PORT));
        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handshake(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                }
                Err(e) => {
                    eprintln!("Unable to connect: {}", e);
                }
            }
        }
    }

    /// Listens for incoming UDP packets.
    ///
    /// # Arguments
    ///
    /// * `connections` - A shared, mutable reference to the map of connections.
    fn listen_udp(self: &Arc<Self>) {
        let udp_socket = match UdpSocket::bind(format!("127.0.0.1:{}", super::UDP_PORT)) {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Could not bind port {}: {}", super::UDP_PORT, e);
                return;
            }
        };

        let mut buf = [0; 1024];

        loop {
            let (amt, _) = match udp_socket.recv_from(&mut buf) {
                Ok((amt, addr)) => (amt, addr),
                Err(e) => {
                    eprintln!("Failed to receive from UDP socket: {}", e);
                    continue;
                }
            };

            let encrypted_text = &buf[..amt];
            match self.secret_key.decrypt(Pkcs1v15Encrypt, encrypted_text) {
                Ok(data) => match String::from_utf8(data) {
                    Ok(string) => println!("Received data: {}", string),
                    Err(e) => eprintln!("Failed to convert data to string: {}", e),
                },
                Err(e) => eprintln!("Failed to decrypt data: {}", e),
            }
        }
    }

    /// Performs a handshake with a client.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream for the client.
    /// * `connections` - A shared, mutable reference to the map of connections.
    ///
    /// # Errors
    ///
    /// Returns an error if the handshake cannot be performed.
    fn handshake(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let peer_addr = stream.peer_addr()?;
        println!("Incoming TCP connection from: {}", peer_addr);

        let mut buf = vec![0; 426];
        stream.read_exact(&mut buf)?;
        let data = buf;
        println!("Received: {}", base64_vec(&data));

        let peer_public_key = RsaPublicKey::from_pkcs1_pem(std::str::from_utf8(&data).unwrap())?;

        let mut connections = self
            .connections
            .lock()
            .map_err(|e| format!("Failed to lock connection: {}", e))?;
        connections.insert(peer_addr.ip(), peer_public_key.clone());

        println!(
            "Peer public key is: {}",
            &peer_public_key.clone().to_pkcs1_pem(LineEnding::LF)?
        );

        let data2 = self
            .public_key
            .to_pkcs1_pem(LineEnding::LF)
            .unwrap()
            .into_bytes();
        println!("Size of {}", data2.len());
        println!("Sent: {}", base64_vec(&data2));

        stream.write_all(&data2)?;

        Ok(())
    }
}

/// Encodes a vector of bytes to a base64 string.
///
/// # Arguments
///
/// * `data` - The data to encode.
fn base64_vec(data: &Vec<u8>) -> String {
    return engine::general_purpose::STANDARD.encode(data);
}
