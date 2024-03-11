use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs8::LineEnding;
use rsa::{pkcs1::EncodeRsaPublicKey, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    io::{self, ErrorKind, Read},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    result::Result,
};

/// The `Client` struct represents a client in a client-server model.
pub struct Client {
    socket: UdpSocket,
    _secret_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    connections: HashMap<IpAddr, RsaPublicKey>,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new() -> Client {
        // Binding to 0.0.0.0:0 means it is random
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let _secret_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&_secret_key);
        Client {
            socket,
            _secret_key,
            public_key,
            connections: HashMap::new(),
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
    pub fn send(&mut self, ip: IpAddr, msg: &str) -> Result<(), Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::UDP_PORT);
        let peer_public_key: RsaPublicKey = match self.connections.get(&ip) {
            Some(peer_public_key) => peer_public_key.clone(),
            None => self.handshake(ip)?,
        };

        let mut rng = rand::thread_rng();
        let encrypted_msg = peer_public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
        self.socket.send_to(&encrypted_msg, addr)?;

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
    fn handshake(&mut self, ip: IpAddr) -> Result<RsaPublicKey, Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::TCP_PORT);
        println!("New connection to {}, exchanging keys", addr);

        let mut stream = TcpStream::connect(addr)?;
        let data = self.public_key.to_pkcs1_pem(LineEnding::LF)?.into_bytes();
        println!("Size of {}", data.len());
        println!("Sent: {}", base64_vec(&data));
        stream.write_all(&data)?;

        let mut buf = vec![0; 426];
        stream.read_exact(&mut buf)?;
        let data2 = buf;
        println!("Received: {}", base64_vec(&data2.to_vec()));
        let peer_public_key = RsaPublicKey::from_pkcs1_pem(std::str::from_utf8(&data2).unwrap())?;

        self.connections.insert(ip, peer_public_key.clone());

        println!(
            "Peer public key is: {}",
            &peer_public_key.clone().to_pkcs1_pem(LineEnding::LF)?
        );

        Ok(peer_public_key)
    }
}

/// Converts an `oqs::Error` to an `io::Error`.
///
/// # Arguments
///
/// * `e` - The `oqs::Error` to convert.
fn _to_io_error(e: oqs::Error) -> io::Error {
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
