use clap::Parser;
use oqs::*;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};

mod client;
mod crypter;
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
        server::listen();
    } else if type_of_service == "client" {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let mut client = client::Client::new();
        client.send(ip, "Hello from client").map_err(print_error);
        client.send(ip, "This is a message").map_err(print_error);
        client.send(ip, "One").map_err(print_error);
        client.send(ip, "Two").map_err(print_error);
        client.send(ip, "Three").map_err(print_error);
        client.send(ip, "Four").map_err(print_error);
        client.send(ip, "Done").map_err(print_error);
    } else {
        panic!("Invalid service type. Please enter either 'server' or 'client'.")
    }

    Ok(())
}

fn print_error(e: Box<dyn Error>) {
    eprintln!("{:?}", e);
}
