use std::io::{self, ErrorKind};
use std::result::Result;
use std::{
    collections::HashMap,
    error::Error,
    io::Read,
    io::Write,
    net::{SocketAddr, TcpStream, UdpSocket},
};

use oqs::{kem::SharedSecret, *};

pub struct Client {
    socket: UdpSocket,
    connections: HashMap<SocketAddr, kem::SharedSecret>,
}

impl Client {
    pub fn new() -> Client {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        Client {
            socket,
            connections: HashMap::new(),
        }
    }

    pub fn send(&mut self, addr: SocketAddr, msg: String) -> Result<(), Box<dyn Error>> {
        let shared_secret = match self.connections.get(&addr) {
            Some(shared_secret) => shared_secret.clone(),
            None => self.handshake(addr)?,
        };

        Ok(())
    }

    fn handshake(&mut self, addr: SocketAddr) -> Result<SharedSecret, Box<dyn Error>> {
        println!("New connection to {}, exchanging keys", addr);

        let kemalg = kem::Kem::new(kem::Algorithm::Kyber1024).map_err(to_io_error)?;
        let (kem_pk, kem_sk) = kemalg.keypair().map_err(to_io_error)?;

        let mut stream = TcpStream::connect(addr)?;
        stream.write_all(&kem_pk.into_vec());

        let mut buf = [0; 1024];
        stream.read(&mut buf)?;
        let kem_ct = kemalg
            .ciphertext_from_bytes(&buf)
            .ok_or("No ciphered text was generated")?;

        let kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct).map_err(to_io_error)?;

        self.connections.insert(addr, kem_ss.clone());

        Ok(kem_ss)
    }
}

fn to_io_error(e: oqs::Error) -> io::Error {
    io::Error::new(ErrorKind::Other, e.to_string())
}
