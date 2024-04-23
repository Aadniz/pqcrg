use super::crypter;
use base64::{engine, Engine as _};
use oqs::*;
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    net::{TcpListener, TcpStream, UdpSocket},
    result::Result,
    sync::{Arc, Mutex},
    thread,
};

type Connections = HashMap<IpAddr, kem::SharedSecret>;

pub struct Forwarder {
    socket: UdpSocket,
}

impl Forwarder {
    pub fn new(
        destination: SocketAddr,
        shared_secret: kem::SharedSecret,
        socket_clone: UdpSocket,
    ) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let socket2 = socket.try_clone().unwrap();

        // Spawn a new thread to handle incoming data
        thread::spawn(move || {
            Self::handle_incoming_data(socket2, shared_secret, socket_clone, destination)
        });

        Forwarder { socket }
    }

    fn handle_incoming_data(
        listening_socket: UdpSocket,
        shared_secret: kem::SharedSecret,
        sending_socket: UdpSocket,
        destination: SocketAddr,
    ) {
        let mut buf = [0; 1024];
        loop {
            let (amt, _src) = match listening_socket.recv_from(&mut buf) {
                Ok((amt, _src)) => (amt, _src),
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    continue;
                }
            };

            let (mut nonce, ciphertext) =
                match crypter::encrypt(shared_secret.clone(), &buf[..amt].to_vec()) {
                    Ok((nonce, ciphertext)) => (nonce, ciphertext),
                    Err(e) => {
                        eprintln!("Failed to encrypt data: {}", e);
                        continue;
                    }
                };
            nonce.extend(ciphertext);

            // Forward the received data to the destination
            if let Err(e) = sending_socket.send_to(&nonce, destination) {
                eprintln!("Failed to send data: {}", e);
            }
        }
    }

    fn pass(&mut self, data: Vec<u8>, port: u16) -> Result<(), Box<dyn Error>> {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let addr = SocketAddr::new(ip, port);
        self.socket.send_to(&data, addr)?;
        Ok(())
    }
}

/// Starts the server and listens for incoming connections.
pub fn listen(port: u16) {
    println!("Server listening on 0.0.0.0:{}", super::BRIDGE_PORT);

    let connections: Arc<Mutex<Connections>> = Arc::new(Mutex::new(HashMap::new()));
    let connections_clone: Arc<Mutex<Connections>> = Arc::clone(&connections);

    thread::spawn(move || listen_tcp(connections_clone));

    listen_udp(connections, port);
}

/// Listens for incoming TCP connections.
///
/// # Arguments
///
/// * `connections` - A shared, mutable reference to the map of connections.
fn listen_tcp(connections: Arc<Mutex<Connections>>) {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", super::BRIDGE_PORT))
        .expect(&format!("Could not bind port {}", super::BRIDGE_PORT));
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
fn listen_udp(connections: Arc<Mutex<Connections>>, port: u16) {
    let udp_socket = match UdpSocket::bind(format!("0.0.0.0:{}", super::BRIDGE_PORT)) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Could not bind port {}: {}", super::BRIDGE_PORT, e);
            return;
        }
    };

    let mut forwards: HashMap<u16, Forwarder> = HashMap::new();

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
            let peer_port = peer_addr.port();
            let forwarder = match forwards.get_mut(&peer_port) {
                Some(f) => f,
                None => {
                    let socket_clone = match udp_socket.try_clone() {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Failed to clone socket: {}", e);
                            continue;
                        }
                    };
                    let forwarder = Forwarder::new(peer_addr, shared_secret.clone(), socket_clone);
                    forwards.insert(peer_port, forwarder);
                    forwards.get_mut(&peer_port).unwrap()
                }
            };

            let nonce = &buf[..12];
            let ciphertext = &buf[12..amt];
            match crypter::decrypt(shared_secret.clone(), nonce, ciphertext) {
                Ok(data) => match forwarder.pass(data, port) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Failed to decrypt data: {}", e),
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
