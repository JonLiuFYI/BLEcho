# BLEcho
Bluetooth Low Energy text messages between a PC and a microcontroller

This repository has two main components:
* a server program that runs on an ESP32
    * source is in the `blecho_server` directory
    * Written in C++
* a client program that runs on a PC
    * source is in the `src` directory
    * Written in Rust

## Setup
For the chat server,
* have an ESP32-WROOM-32 dev module
* install Arduino IDE
* install ESP32 board support for Arduino

For the chat client,
* have a personal computer with Bluetooth
* install Rust

## Usage
Make sure the PC's Bluetooth is enabled.

1. **Start the ESP32 chat server:** Upload the `blecho_server.ino` Arduino sketch to the ESP32. It will start waiting for a client connection.
    * Also, make sure the baud rate of Arduino IDE's serial monitor is 115200.
2. **Start the PC chat client:** The quick way is with the terminal command `cargo run`.

Once the client finds and connects to the server, you can start sending messages. The server will display the received messages in the serial monitor of Arduino IDE.

Press <kbd>Ctrl</kbd><kbd>D</kbd> to disconnect and quit. The server will go back to waiting for a client.

## License
Copyright © 2022 JonLiuFYI.

BLEcho is free software. All of its source is released under the GNU GPL v3 or later. See `LICENSE`.
