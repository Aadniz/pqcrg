use oqs::kem::SharedSecret;
use oqs::*;
use std::error::Error;
use std::io::Read;
use std::net::IpAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream, UdpSocket},
};

pub struct Server {
    tcp_listener: TcpListener,
    udp_socket: UdpSocket,
    connections: Arc<Mutex<HashMap<IpAddr, SharedSecret>>>,
}

impl Server {
    pub fn new() -> Server {
        let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", super::TCP_PORT))
            .expect(&format!("Could not bind port {}", super::TCP_PORT));
        let udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", super::UDP_PORT))
            .expect(&format!("Could not bind port {}", super::UDP_PORT));
        let connections = Arc::new(Mutex::new(HashMap::new()));
        Server {
            tcp_listener,
            udp_socket,
            connections,
        }
    }

    pub fn listen(&mut self) {
        println!(
            "Server listening on TCP 127.0.0.1:{} and UDP 127.0.0.1:{}",
            super::TCP_PORT,
            super::UDP_PORT
        );

        // Accept all incoming connections
        let mut incoming_streams: Vec<TcpStream> = Vec::new();
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => incoming_streams.push(stream),
                Err(e) => eprintln!("Unable to connect: {}", e),
            }
        }

        // Handle all accepted connections
        for stream in incoming_streams {
            if let Err(error) = self.handle_tcp_client(stream) {
                eprintln!("{:?}", error);
            }
        }

        loop {
            if let Err(error) = self.handle_udp_client() {
                eprintln!("{:?}", error);
            }
        }
    }

    fn handle_tcp_client(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let peer_addr = stream.peer_addr()?;
        println!("Incoming TCP connection from: {}", peer_addr);

        let mut buf = [0; 1024]; // Adjust buffer size as needed
        stream.read(&mut buf)?;

        let kemalg = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
        let kem_pk = kemalg
            .public_key_from_bytes(&buf)
            .ok_or("Failed to create public key from bytes")?;
        let (kem_ct, kem_ss) = kemalg.encapsulate(&kem_pk).unwrap();

        self.connections.insert(peer_addr.ip(), kem_ss);

        Ok(())
    }

    fn handle_udp_client(&self) -> std::io::Result<()> {
        let mut buf = [0; 1024];

        loop {
            let (amt, src) = self.udp_socket.recv_from(&mut buf)?;

            // Echo the data back to the client
            self.udp_socket.send_to(&buf[..amt], &src)?;
        }
    }
}
