use godot::prelude::*;
use std::net::IpAddr;
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
            "Host bridge started on port 0.0.0.0:{BRIDGE_PORT}, forwarded to port 127.0.0.1:{port}"
        );
        thread::spawn(move || server::listen(port.clone()));
    }

    #[func]
    fn start_client_bridge(&mut self, ip: String, port: u16) {
        godot_print!(
            "Client bridge started on port 0.0.0.0:{BRIDGE_PORT}, forwarded to port {ip}:{port}"
        );
        match ip.parse::<IpAddr>() {
            Ok(ip) => {
                thread::spawn(move || {
                    let mut client = client::Client::new();
                    client.pass(ip, port);
                });
            }
            Err(e) => {
                godot_error!("{e}");
            }
        };
    }
}
