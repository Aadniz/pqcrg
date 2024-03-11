use clap::Parser;
use oqs::*;
use std::net::{IpAddr, Ipv4Addr};

mod client;
mod server;

pub const TCP_PORT: u16 = 2522;
pub const UDP_PORT: u16 = 2525;

/// Client-Server PQC
///
/// Client-Server communication application for PQC
#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    /// What type of service are you running, server or client?
    #[clap(name = "TYPE", default_value = "client")]
    input: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let type_of_service: String = cli.input;
    if type_of_service == "server" {
        server::Server::new().listen();
    } else if type_of_service == "client" {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let mut client = client::Client::new();
        send_message(&mut client, ip, "Hello from client");
        send_message(&mut client, ip, "This is a message");
        send_message(&mut client, ip, "One");
        send_message(&mut client, ip, "Two");
        send_message(&mut client, ip, "Three");
        send_message(&mut client, ip, "Four");
        send_message(&mut client, ip, "Done");
    } else {
        panic!("Invalid service type. Please enter either 'server' or 'client'.")
    }

    Ok(())
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
