use super::crypter;
use base64::{engine, Engine as _};
use oqs::*;
use std::error::Error;
use std::io::{Read, Write};
use std::net::IpAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
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

    let connections: Arc<Mutex<Connections>> = Arc::new(Mutex::new(HashMap::new()));
    let connections_clone: Arc<Mutex<Connections>> = Arc::clone(&connections);

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
    let mut start_time = Instant::now();
    let mut handshakes = 0;
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                let connections = Arc::clone(&connections);
                handshake(stream, connections).unwrap_or_else(|error| eprintln!("{:?}", error));
                handshakes += 1;
                if start_time.elapsed() >= Duration::from_secs(1) {
                    println!("Handshakes per second: {}", handshakes);
                    handshakes = 0;
                    start_time = Instant::now();
                }
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
    let mut start_time = Instant::now();
    let mut total_packets = 0;
    let mut error_packets = 0;

    loop {
        let (amt, peer_addr) = match udp_socket.recv_from(&mut buf) {
            Ok((amt, addr)) => {
                total_packets += 1;
                (amt, addr)
            }
            Err(e) => {
                eprintln!("Failed to receive from UDP socket: {}", e);
                continue;
            }
        };

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
                    Ok(string) => (),
                    Err(e) => {
                        eprintln!("Failed to convert data to string: {}", e);
                        error_packets += 1;
                    }
                },
                Err(e) => {
                    //eprintln!("Failed to decrypt data: {}", e);
                    error_packets += 1;
                }
            }
        } else {
            eprintln!("No shared secret for peer: {}", peer_addr.ip());
            error_packets += 1;
        }

        if start_time.elapsed() >= Duration::from_secs(1) {
            let elapsed = start_time.elapsed().as_secs();
            if elapsed > 0 && 2 >= elapsed && error_packets != 0 {
                let error_rate = error_packets as f64 / total_packets as f64;
                println!("Error rate: {:.2}%", error_rate * 100.0);
            }
            total_packets = 0;
            error_packets = 0;
            start_time = Instant::now();
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
    //println!("Incoming TCP connection from: {}", peer_addr);

    let mut buf = vec![0; 1184];
    stream.read_exact(&mut buf)?;
    let data = buf;
    //println!("Received: {}", base64_vec(&data));

    let kemalg = kem::Kem::new(kem::Algorithm::Kyber768)
        .map_err(|e| format!("Failed to create KEM: {}", e))?;
    let kem_pk = kemalg
        .public_key_from_bytes(&data)
        .ok_or("Failed to create public key from bytes")?;
    let (kem_ct, kem_ss) = kemalg
        .encapsulate(&kem_pk)
        .map_err(|e| format!("Failed to encapsulate: {}", e))?;

    let mut connections = connections
        .lock()
        .map_err(|e| format!("Failed to lock connection: {}", e))?;
    connections.insert(peer_addr.ip(), kem_ss.clone());
    //println!("Shared key is: {}", base64_vec(&kem_ss.into_vec()));

    let data2 = kem_ct.into_vec();
    //println!("Size of {}", data2.len());
    //println!("Sent: {}", base64_vec(&data2));

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
