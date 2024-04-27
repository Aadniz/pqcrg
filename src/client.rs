use super::crypter;
use crate::{HandshakeMethod, RsaKeypair};
use base64::{engine::general_purpose, Engine as _};
use openssl::{pkey::Private, rsa::Rsa};
use rand::Rng;
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    io::{self, ErrorKind, Read},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    result::Result,
};

use oqs::kem;

/// The `Client` struct represents a client in a client-server model.
pub struct Client {
    id: u32,
    socket: UdpSocket,
    method: super::HandshakeMethod,
    rsa_keypair: Option<RsaKeypair>,
    connections: HashMap<IpAddr, super::KEM>,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new(method: super::HandshakeMethod) -> Client {
        // Binding to 0.0.0.0:0 means it is random
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let id = rand::thread_rng().gen();
        //println!("\nid is: {}", id);
        Client {
            id,
            socket,
            method,
            rsa_keypair: None,
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
        let kem = match self.connections.get(&ip) {
            Some(kem) => kem.clone(),
            None => self.handshake(ip)?,
        };

        let ciphertext = crypter::encrypt(kem, msg.as_bytes())
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;

        let mut data = self.id.to_be_bytes().to_vec();
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
    fn handshake(&mut self, ip: IpAddr) -> Result<super::KEM, Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::TCP_PORT);
        //println!("New connection to {}, exchanging keys", addr);

        let kem = match self.method {
            HandshakeMethod::Kyber => self.kyber_kem(addr)?,
            HandshakeMethod::Rsa => self.rsa_kem(addr)?,
            HandshakeMethod::None => super::KEM::None,
        };

        self.connections.insert(ip, kem.clone());

        //match kem.clone() {
        //    crate::KEM::Kyber(k) => println!("Key is: {}", base64_vec(&k.clone().into_vec())),
        //    crate::KEM::Rsa(k) => println!("Key is: {:?}", k.clone()),
        //    crate::KEM::None => println!("No key"),
        //}

        Ok(kem)
    }

    fn rsa_kem(&mut self, addr: SocketAddr) -> Result<super::KEM, Box<dyn Error>> {
        let keypair: RsaKeypair = match self.rsa_keypair.clone() {
            Some(r) => r,
            None => {
                let keypair = RsaKeypair::new();
                self.rsa_keypair = Some(keypair.clone());
                keypair
            }
        };

        let mut stream = TcpStream::connect(addr)?;

        // Bake id into data
        let mut data = self.id.to_be_bytes().to_vec();
        data.extend(keypair.public_key);
        //println!("Size of {}", data.len());
        //println!("Sent: {:?}", &data);
        stream.write_all(&data)?;

        let mut buf = [0; 451]; // Assuming the server's public key is also 2048-bit
        stream.read_exact(&mut buf)?;

        let server_public_key = buf.to_vec();
        let rsa_pub_key = Rsa::public_key_from_pem(&server_public_key)?;
        //println!("Received: {}", base64_vec(&server_public_key));

        Ok(super::KEM::Rsa(rsa_pub_key))
    }

    fn kyber_kem(&mut self, addr: SocketAddr) -> Result<super::KEM, Box<dyn Error>> {
        let kemalg = kem::Kem::new(kem::Algorithm::Kyber768).map_err(to_io_error)?;
        let (kem_pk, kem_sk) = kemalg.keypair().map_err(to_io_error)?;

        let mut stream = TcpStream::connect(addr)?;

        // Bake id into data
        let mut data = self.id.to_be_bytes().to_vec();
        data.extend(kem_pk.clone().into_vec());
        //println!("Size of {}", data.len());
        //println!("Sent: {:?}", &data);
        stream.write_all(&data)?;

        let mut buf = [0; 1088];
        stream.read_exact(&mut buf)?;
        let data2 = buf;
        //println!("Received: {}", base64_vec(&data2.to_vec()));
        let kem_ct = kemalg
            .ciphertext_from_bytes(&data2)
            .ok_or("No ciphered text was generated")?;

        Ok(super::KEM::Kyber(
            kemalg.decapsulate(&kem_sk, &kem_ct).map_err(to_io_error)?,
        ))
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
