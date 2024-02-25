use super::crypter;
use base64::{engine::general_purpose, Engine as _};
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    io::{self, ErrorKind, Read},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    result::Result,
};

use oqs::{kem::SharedSecret, *};

pub struct Client {
    socket: UdpSocket,
    connections: HashMap<IpAddr, kem::SharedSecret>,
}

impl Client {
    pub fn new() -> Client {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        Client {
            socket,
            connections: HashMap::new(),
        }
    }

    pub fn send(&mut self, ip: IpAddr, msg: &str) -> Result<(), Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::UDP_PORT);
        let shared_secret = match self.connections.get(&ip) {
            Some(shared_secret) => shared_secret.clone(),
            None => self.handshake(ip)?,
        };

        let (nonce, ciphertext) = crypter::encrypt(shared_secret, msg.as_bytes())
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))?;
        let mut data = nonce;
        data.extend(ciphertext);
        self.socket.send_to(&data, addr)?;

        Ok(())
    }

    fn handshake(&mut self, ip: IpAddr) -> Result<SharedSecret, Box<dyn Error>> {
        let addr = SocketAddr::new(ip, super::TCP_PORT);
        println!("New connection to {}, exchanging keys", addr);

        let kemalg = kem::Kem::new(kem::Algorithm::Kyber768).map_err(to_io_error)?;
        let (kem_pk, kem_sk) = kemalg.keypair().map_err(to_io_error)?;

        let mut stream = TcpStream::connect(addr)?;
        let data = kem_pk.into_vec();
        println!("Size of {}", data.len());
        println!("Sent:     {:?}", base64_vec(&data));
        stream.write_all(&data)?;

        let mut buf = [0; 1088];
        stream.read_exact(&mut buf)?;
        let data2 = buf;
        println!("Received: {:?}", base64_vec(&data2.to_vec()));
        let kem_ct = kemalg
            .ciphertext_from_bytes(&data2)
            .ok_or("No ciphered text was generated")?;

        let kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct).map_err(to_io_error)?;

        self.connections.insert(ip, kem_ss.clone());

        println!(
            "Shared key is: {:?}",
            base64_vec(&kem_ss.clone().into_vec())
        );

        Ok(kem_ss)
    }
}

fn to_io_error(e: oqs::Error) -> io::Error {
    io::Error::new(ErrorKind::Other, e.to_string())
}

fn base64_vec(data: &Vec<u8>) -> String {
    return general_purpose::STANDARD.encode(data);
}
