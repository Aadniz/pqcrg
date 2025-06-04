# Godot

This folder contains all the required content for Godot. It is means to be opened in the Godot Editor.

It connects with the bridge using the PQC library located at `../pqc/target/*/libpqcrg.*`. Check the `pqc.gdextension` file which links these 2 applications together.

Since Godot does now allow to run the library multithreaded, it is handled from the library itself instead.
