use clap::Parser;
use oqs::*;

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
        server::listen();
    } else if type_of_service == "client" {
        let _client = client::Client::new();
    } else {
        panic!("Invalid service type. Please enter either 'server' or 'client'.")
    }
    if type_of_service != "server" && type_of_service != "client" {}
    let kemalg = kem::Kem::new(kem::Algorithm::Kyber1024)?;

    // A -> B: kem_pk, signature
    let (kem_pk, kem_sk) = kemalg.keypair()?;

    // B -> A: kem_ct, signature
    let (kem_ct, b_kem_ss) = kemalg.encapsulate(&kem_pk)?;

    // A verifies, decapsulates, now both have kem_ss
    let a_kem_ss = kemalg.decapsulate(&kem_sk, &kem_ct)?;
    assert_eq!(a_kem_ss, b_kem_ss);

    println!("Hei!");

    Ok(())
}
