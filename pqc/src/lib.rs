use godot::prelude::*;
use std::net::ToSocketAddrs;
use std::thread;

mod client;
mod crypter;
mod server;

pub const BRIDGE_PORT: u16 = 3522;

struct PqcExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PqcExtension {}

#[derive(GodotClass)]
struct Pqc {
    base: Base<RefCounted>,
}
#[godot_api]
impl IRefCounted for Pqc {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Pqc {
    #[func]
    fn start_host_bridge(&mut self, port: u16) {
        godot_print!(
            "Host bridge started on port 0.0.0.0:{BRIDGE_PORT}, forwarded to port 127.0.0.1:{port} blarg"
        );
        thread::spawn(move || server::listen(port.clone()));
    }

    #[func]
    fn start_client_bridge(&mut self, host: String, port: u16) {
        godot_print!(
            "Client bridge started on port 0.0.0.0:{BRIDGE_PORT}, forwarded to port {host}:{port}"
        );
        match (host.as_str(), port).to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    thread::spawn(move || {
                        let mut client = client::Client::new();
                        client.pass(addr.ip(), addr.port());
                    });
                } else {
                    godot_error!("No IP addresses found for the provided hostname.");
                }
            }
            Err(e) => {
                godot_error!("{e}");
            }
        };
    }
}
