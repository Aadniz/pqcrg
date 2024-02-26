use super::crypter;
use base64::{engine, Engine as _};
use oqs::*;
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
type Connections = HashMap<IpAddr, kem::SharedSecret>;

/// Starts the server and listens for incoming connections.
pub fn listen() {
    println!(
        "Server listening on TCP 127.0.0.1:{} and UDP 127.0.0.1:{}",
        super::TCP_PORT,
        super::UDP_PORT
    );

    let connections : Arc<Mutex<Connections>> = Arc::new(Mutex::new(HashMap::new()));
    let connections_clone : Arc<Mutex<Connections>> = Arc::clone(&connections);

    thread::spawn(move || listen_tcp(connections_clone));
    listen_udp(connections);
}

/// Listens for incoming TCP connections.
///
/// # Arguments
///
/// * `connections` - A shared, mutable reference to the map of connections.
fn listen_tcp(connections: Arc<Mutex<Connections>>) {
    let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", super::TCP_PORT))
        .expect(&format!("Could not bind port {}", super::TCP_PORT));
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                let connections = Arc::clone(&connections);
                handshake(stream, connections).unwrap_or_else(|error| eprintln!("{:?}", error));
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
fn listen_udp(connections: Arc<Mutex<Connections>>) {
    let udp_socket = match UdpSocket::bind(format!("127.0.0.1:{}", super::UDP_PORT)) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Could not bind port {}: {}", super::UDP_PORT, e);
            return;
        }
    };

    let mut buf = [0; 1024];

    loop {
        let (amt, peer_addr) = match udp_socket.recv_from(&mut buf) {
            Ok((amt, addr)) => (amt, addr),
            Err(e) => {
                eprintln!("Failed to receive from UDP socket: {}", e);
                continue;
            }
        };

        // Get the shared secret for this peer
        let connections = match connections.lock() {
            Ok(connections) => connections,
            Err(e) => {
                eprintln!("Failed to lock connections: {}", e);
                continue;
            }
        };

        if let Some(shared_secret) = connections.get(&peer_addr.ip()) {
            let nonce = &buf[..12];
            let ciphertext = &buf[12..amt];
            match crypter::decrypt(shared_secret.clone(), nonce, ciphertext) {
                Ok(data) => match String::from_utf8(data) {
                    Ok(string) => println!("Received data: {}", string),
                    Err(e) => eprintln!("Failed to convert data to string: {}", e),
                },
                Err(e) => eprintln!("Failed to decrypt data: {}", e),
            }
        } else {
            eprintln!("No shared secret for peer: {}", peer_addr.ip());
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
fn handshake(
    mut stream: TcpStream,
    connections: Arc<Mutex<Connections>>,
) -> Result<(), Box<dyn Error>> {
    let peer_addr = stream.peer_addr()?;
    println!("Incoming TCP connection from: {}", peer_addr);

    let mut buf = vec![0; 1184];
    stream.read_exact(&mut buf)?;
    let data = buf;
    println!("Received: {}", base64_vec(&data));

    let kemalg = kem::Kem::new(kem::Algorithm::Kyber768)?;
    let kem_pk = kemalg
        .public_key_from_bytes(&data)
        .ok_or("Failed to create public key from bytes")?;
    let (kem_ct, kem_ss) = kemalg.encapsulate(&kem_pk)?;

    let mut connections = connections.lock()?;
    connections.insert(peer_addr.ip(), kem_ss.clone());
    println!("Shared key is: {}", base64_vec(&kem_ss.into_vec()));

    let data2 = kem_ct.into_vec();
    println!("Size of {}", data2.len());
    println!("Sent: {}", base64_vec(&data2));

    stream.write_all(&data2)?;

    Ok(())
}

/// Encodes a vector of bytes to a base64 string.
///
/// # Arguments
///
/// * `data` - The data to encode.
fn base64_vec(data: &Vec<u8>) -> String {
    return engine::general_purpose::STANDARD.encode(data);
}
