use crate::{handshake, ip};

use super::crypter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::{
    collections::HashMap,
    net::{TcpListener, UdpSocket},
};

type Connections = super::Connections;

/// Starts the server and listens for incoming connections.
pub fn listen(handshake_method: super::HandshakeMethod) {
    println!(
        "Server listening on TCP 127.0.0.1:{} and UDP 127.0.0.1:{} using {} encryption",
        super::TCP_PORT,
        super::UDP_PORT,
        handshake_method
    );

    let connections: Arc<Mutex<Connections>> = Arc::new(Mutex::new(HashMap::new()));
    let connections_clone: Arc<Mutex<Connections>> = Arc::clone(&connections);

    thread::spawn(move || listen_tcp(connections_clone, handshake_method));
    listen_udp(connections);
}

/// Listens for incoming TCP connections.
///
/// # Arguments
///
/// * `connections` - A shared, mutable reference to the map of connections.
fn listen_tcp(connections: Arc<Mutex<Connections>>, method: super::HandshakeMethod) {
    let tcp_listener = TcpListener::bind(format!("127.0.0.1:{}", super::TCP_PORT))
        .expect(&format!("Could not bind port {}", super::TCP_PORT));
    let mut start_time = Instant::now();
    let mut handshakes = 0;
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                let connections = Arc::clone(&connections);
                match handshake::handshake(stream, connections, method.clone()) {
                    Ok(_) => handshakes += 1,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        continue;
                    }
                }
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

        let id_bytes = &buf[..4];
        let id = u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]]);
        let ip_num = ip::ip_to_u32(peer_addr.ip());
        let client_id = ((ip_num as u64) << 32) + id as u64;

        if let Some(kem) = connections.get(&client_id) {
            match crypter::decrypt(kem, &buf[4..amt]) {
                Ok(data) => match String::from_utf8(data) {
                    Ok(string) => (), //println!("Got message: {}", string),
                    Err(e) => {
                        eprintln!("Failed to convert data to string: {}", e);
                        error_packets += 1;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to decrypt data: {}", e);
                    error_packets += 1;
                }
            }
        } else {
            //eprintln!("No shared secret for id: {}", id);
            error_packets += 1;
        }

        if start_time.elapsed() >= Duration::from_secs(1) {
            let elapsed = start_time.elapsed().as_secs();
            if elapsed > 0 && 2 >= elapsed && error_packets != 0 {
                let error_rate = error_packets as f64 / total_packets as f64;
                println!("Error rate: {:.2}%", error_rate * 100.0);
            }
            println!(
                "Packets per second: {}, ({})",
                total_packets - error_packets,
                total_packets
            );
            total_packets = 0;
            error_packets = 0;
            start_time = Instant::now();
        }
    }
}
