# Switchy

This is a simple program with uses Slint for the UI and allows you to turn on and off your Switchbot light.
## About

This project is to allow you to turn on and off your Switchbot light. Currently it only controls the first light, as I can't test for more devices as I only have one. It is resposive and mainly a learning project for me. My end goal is to put this onto an ESP32.

Contributions are more than welcome!

## Usage

1. Clone the repository:
    ```
    git clone https://github.com/SO9010/Switchy
    ```
    
2. In the main.rs file, replace the `token` and the`secret` with your Switchbot details. [See Here](https://github.com/OpenWonderLabs/SwitchBotAPI?tab=readme-ov-file#getting-started)
4. Build with `cargo`:
    ```
    cargo build
    // To build for release:
    // cargo build --release
    ```
5. Run the application binary:
    ```
    cargo run
    ```

## Screenshots

![Off state](<assets/Screenshot from 2024-10-27 20-56-29.png>)
![On State](<assets/Screenshot from 2024-10-27 20-56-15.png>)
