# PQC rust implementation

The implementation uses a client bridge, and a host bridge. These bridges communicate with the application that is connecting to it.

## Testing the implementation

To test the implementation, you first need 2 or more (virtual) machines that are not on the same localhost.

Since we compile as a library, you would also need to tune `Cargo.toml` to use `main.rs` instead of `lib.rs`.
You can do this by commenting out the following:

```toml
#[lib]
#crate-type = ["cdylib"]
```

1. Compile and start the host bridge

```bash
$ cargo build --release
$ ./target/release/pqcrg server --port 2522
```

The `port` specified means the localhost listening port. So packets will be forwarded from `3522` at `0.0.0.0/0` to `2522` at `127.0.0.1`.

2. Compile and start the client bridge

```bash
$ cargo build --release
$ ./target/release/pqcrg client --ip 192.168.111.202 --port 3522
```

On the client side, we tell it to forward packets from localhost, to ip `192.168.111.202` at port `3522`. The ip `192.168.111.202` should be the host reachable ip address. Port `3522` is the built-in bridge port.

3. Run the host application.

This application can be whatever that supports pure UDP communication. For our testing purposes, we will use the python `main.py` example code.

```bash
$ python3 main.py server --port 2522
```

The port `2522` means it will listen on port 2522. Data from the bridge will enter this port.

4. Run the client application.

Now that we have the 2 bridges set up, and the host application ready, we can connect the client application.

Again in our example, we will use the python `main.py` example code.

```bash
$ python3 main.py client --ip 127.0.0.1 --port 3522
```

The ip and port specifies that we are sending the packets to localhost at port `3522`. The bridge listens to port `3522`, so this is exactly what we want.

## Routing explanation

For the complete setup, the routing starts at the client application, forwards packet to the bridge, that encrypts the data with PQC and forwards the packets to the host bridge. Then the host bridge decrypts the messages and forwards it to its localhost application host. If a response happens from the host, it will respond accordingly, encrypted.

An example case for 2 machines might look like this:
```
(client) 192.168.111.111 -> 127.0.0.1:3522 -> 192.168.111.202:3522 -> 127.0.0.1:2522
```

With the nature of sending packets, an available sending port is chosen. During responding, it will follow up on these chosen ports
