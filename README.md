# PQCRC

This project demonstrates how one can have quantum-resistant cryptography in a fast-paced environment, like a racing game.

This was the Bachelor thesis of Tønnes ([@Lystrous](https://github.com/Lystrous)) and Kristian ([@Aadniz](https://github.com/Aadniz)) at UiA 2024.

![Screenshot of 4 clients in the game](/figures/game.png)

## How it works

The game uses a local bridge to encrypt its traffic over. The bridge communicates with the server bridge that forwards the packets to its local Godot instance. The entire communication looks like shown below.

<p align="center">
    <img src="/figures/host-client-bridge_diagram.svg" alt="Overview of network flow with 3 clients and 1 host"/>
</p>

Kyber-768 is used for the key exchange handshake, and AES-GCM is used for encrypting the packets.

## Setting up

<!-- ### Development environment -->

### Dependencies

- [Godot](https://godotengine.org/) - Game engine
- [Blender](https://www.blender.org/) - For importing the map
- [Rust](https://github.com/rust-lang/rust) - For building the bridge
- [CMake](https://gitlab.kitware.com/cmake/cmake) - for compiling PQC

### Building

Build the PQC bridge first, then start the Godot Editor

``` bash
$ git clone https://github.com/Aadniz/pqcrg.git
$ cd pqcrg/pqc
$ cargo build  # binds to debug in Godot
$ cargo build --release  # binds to release in Godot
```

Then open Godot and import the Godot folder.

<!-- ### Running the game pre-compiled

 Go to the release section, and download blarg blah blah. -->

