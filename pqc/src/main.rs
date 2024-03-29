use clap::Parser;
use oqs::*;
use std::net::IpAddr;

mod client;
mod crypter;
mod server;

pub const BRIDGE_PORT: u16 = 3522;

/// Client-Server PQC
///
/// Client-Server communication application for PQC
#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    /// What type of service are you running, server or client?
    #[clap(subcommand)]
    service: Service,
}

#[derive(Parser)]
enum Service {
    Server(Server),
    Client(Client),
}

#[derive(Parser)]
struct Server {
    /// The port to listen on
    #[clap(short, long)]
    port: u16,
}

#[derive(Parser)]
struct Client {
    /// The IP address to connect to
    #[clap(short, long)]
    ip: IpAddr,

    /// The port to connect to
    #[clap(short, long)]
    port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.service {
        Service::Server(server) => {
            let port = server.port;
            server::listen(port);
        }
        Service::Client(client) => {
            let ip = client.ip;
            let port = client.port;
            client::Client::new().pass(ip, port);
        }
    }

    Ok(())
}
