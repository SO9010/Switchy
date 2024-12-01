# Switchy

This is a simple program that uses Slint for the UI and allows you to turn on and off your Switchbot light.
## About

This project is to allow you to turn on and off your Switchbot light. Currently it only controls the first light, as I can't test for more devices as I only have one. It is resposive and mainly a learning project for me. 

Contributions are more than welcome!

## Goals:
Make a tray item for the program, perhaps have it simplified.


Get the esp32 version running to the same standard as this one, currently it is working, however i need to make it so that it has the off screen and to have the power symbol. Once that is done I will open up the resposity, and add it to this resposity. I did however, have to use c++ for it as the esp32 board i was using didnt have support in rust.
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

## In action:

https://github.com/user-attachments/assets/0a4c38b5-f469-460a-9109-29b8cac86e7d
