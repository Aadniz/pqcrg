use clap::Parser;
use lazy_static::lazy_static;
use openssl::pkey::{Private, Public};
use openssl::rsa::Rsa;
use oqs::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use std::{thread, time::Duration};

mod client;
mod crypter;
mod handshake;
mod ip;
mod server;

lazy_static! {
    static ref RSA: Rsa<Private> = Rsa::generate(2048).unwrap();
    static ref PUB_KEY: Vec<u8> = RSA.public_key_to_pem().unwrap();
}

#[derive(Clone)]
struct RsaKeypair {
    private_key: Rsa<Private>,
    public_key: Vec<u8>,
}
impl RsaKeypair {
    pub fn new() -> RsaKeypair {
        let private_key = Rsa::generate(2048).unwrap();
        let public_key = private_key.public_key_to_pem().unwrap();
        RsaKeypair {
            private_key,
            public_key,
        }
    }
}

#[derive(Clone)]
pub enum KEM {
    Kyber(kem::SharedSecret),
    Rsa(Rsa<Public>),
    None,
}
pub type Connections = HashMap<u64, KEM>;

pub const TCP_PORT: u16 = 2522;
pub const UDP_PORT: u16 = 2525;

#[derive(clap::ValueEnum, Clone)]
enum HandshakeMethod {
    Kyber,
    Rsa,
    None,
}

impl std::fmt::Display for HandshakeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = match self {
            HandshakeMethod::Kyber => "Kyber",
            HandshakeMethod::Rsa => "Rsa",
            HandshakeMethod::None => "None",
        };
        write!(f, "{}", method)
    }
}

/// Client-Server PQC
///
/// Client-Server communication application for PQC
#[derive(Parser)]
#[clap(author, about, long_about = None)]
enum Cli {
    /// Client service
    #[clap(name = "client")]
    Client {
        /// Number of packets to send per second
        #[clap(long, default_value = "1")]
        packets_per_second: u32,

        /// Set max stress testing, will send as many packets as it can within the set duration. This ignores packets_per_second
        #[clap(long, default_value = "false")]
        max_stress: bool,

        /// Chose the amount of threads to send from
        #[clap(long, default_value = "1")]
        threads: u8,

        /// The duration of the test
        #[clap(long, default_value = "60")]
        duration: u64,

        /// Gradually increase the stress till it has reached --packets-per-second
        #[clap(long, default_value = "false")]
        gradual: bool,

        /// The algorithm to use for handshake
        #[clap(long, default_value = "kyber")]
        handshake_method: HandshakeMethod,
    },

    /// Server service
    #[clap(name = "server")]
    Server {
        /// The algorithm to use for handshake
        #[clap(long, default_value = "kyber")]
        handshake_method: HandshakeMethod,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli {
        Cli::Server { handshake_method } => server::listen(handshake_method),
        Cli::Client {
            packets_per_second,
            max_stress,
            threads,
            duration,
            gradual,
            handshake_method,
        } => stress_loop(
            packets_per_second,
            max_stress,
            threads,
            duration,
            gradual,
            handshake_method,
        ),
    }

    Ok(())
}

fn stress_loop(
    packets_per_second: u32,
    max_stress: bool,
    threads: u8,
    duration: u64,
    gradual: bool,
    method: HandshakeMethod,
) {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut instance_counter = 0;

    print!("Starting in ");
    for i in 10..0 {
        print!("{} ", 9 - i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    if gradual {
        for i in 1..=packets_per_second {
            let sleep_duration = Duration::from_micros(1_000_000 / (i as u64));
            println!("Sending {} handshakes per second", i);
            for _ in 0..i {
                instance_counter += 1;
                let mut client = client::Client::new(method.clone());
                send_message(
                    &mut client,
                    ip,
                    format!("Message from instance {}", instance_counter).as_str(),
                );
                thread::sleep(sleep_duration);
            }
        }
    } else {
        let start_time = Instant::now();
        let sleep_duration = if max_stress {
            Duration::from_micros(0)
        } else {
            Duration::from_micros(1_000_000 / ((packets_per_second / threads as u32) as u64))
        };

        let mut handles = vec![];

        for _ in 0..threads {
            let method = method.clone();
            handles.push(thread::spawn(move || {
                while start_time.elapsed() < Duration::from_secs(duration) {
                    instance_counter += 1;
                    let mut client = client::Client::new(method.clone());
                    send_message(
                        &mut client,
                        ip,
                        format!("Message from instance {}", instance_counter).as_str(),
                    );
                    thread::sleep(sleep_duration);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

/// Sends a message to the specified IP address using the given client.
///
/// # Arguments
///
/// * `client` - The client to use to send the message.
/// * `ip` - The IP address to send the message to.
/// * `msg` - The message to send.
fn send_message(client: &mut client::Client, ip: IpAddr, msg: &str) {
    if let Err(error) = client.send(ip, msg) {
        eprintln!("{:?}", error);
    }
}
