use base64::{engine, Engine as _};
use openssl::rsa::Rsa;
use oqs::*;
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::result::Result;
use std::sync::{Arc, Mutex};

use crate::{ip, HandshakeMethod};

type Connections = super::Connections;

pub fn handshake(
    stream: TcpStream,
    connections: Arc<Mutex<Connections>>,
    handshake_method: super::HandshakeMethod,
) -> Result<(), Box<dyn Error>> {
    match handshake_method {
        HandshakeMethod::Kyber => kyber(stream, connections),
        HandshakeMethod::Rsa => rsa(stream, connections),
        HandshakeMethod::None => Ok(()),
    }
}

pub fn kyber(
    mut stream: TcpStream,
    connections: Arc<Mutex<Connections>>,
) -> Result<(), Box<dyn Error>> {
    let peer_addr = stream.peer_addr()?;
    //println!("Incoming TCP connection from: {}", peer_addr);

    let mut buf = vec![0; 4 + 1184];
    stream.read_exact(&mut buf)?;
    //println!("Received: {:?}", &buf.to_vec());

    // Split the buffer into the ID and the public key
    let (id_bytes, data) = buf.split_at(4);
    // Convert the ID to a u32
    let id = u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]]);
    let ip_num = ip::ip_to_u32(peer_addr.ip());
    let client_id = ((ip_num as u64) << 32) + id as u64;
    //println!("id is: {}", id);
    //println!("Received: {:?}", &data.to_vec());

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
    connections.insert(client_id, super::KEM::Kyber(kem_ss.clone()));
    //println!("Shared key is: {}", base64_vec(&kem_ss.into_vec()));

    let data2 = kem_ct.into_vec();
    //println!("Size of {}", data2.len());
    //println!("Sent: {}", base64_vec(&data2));

    stream.write_all(&data2)?;

    Ok(())
}

pub fn rsa(
    mut stream: TcpStream,
    connections: Arc<Mutex<Connections>>,
) -> Result<(), Box<dyn Error>> {
    let peer_addr = stream.peer_addr()?;

    // Receive public key from client
    let mut buf = vec![0; 4 + 451]; // 2048-bit = 256 bytes
    stream.read_exact(&mut buf)?;
    //println!("Received: {:?}", &buf.to_vec());

    // Split the buffer into the ID and the public key
    let (id_bytes, data) = buf.split_at(4);
    // Convert the ID to a u32
    let id = u32::from_be_bytes([id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]]);
    let ip_num = ip::ip_to_u32(peer_addr.ip());
    let client_id = ((ip_num as u64) << 32) + id as u64;
    //println!("id is: {}", id);

    let public_key = data.to_vec();
    let public_key = Rsa::public_key_from_pem(&public_key)?;

    let mut connections = connections
        .lock()
        .map_err(|e| format!("Failed to lock connection: {}", e))?;
    connections.insert(client_id, super::KEM::Rsa(public_key));

    //println!("Size of {}", &super::PUB_KEY.len());
    stream.write_all(&super::PUB_KEY)?;

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
