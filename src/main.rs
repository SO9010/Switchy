// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use client::webapi::SwitchBotClient;

slint::include_modules!();

mod client;
mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let ui = AppWindow::new()?;
    // https://github.com/OpenWonderLabs/SwitchBotAPI?tab=readme-ov-file#getting-started
    let token = "YOUR TOKEN";
    let secret = "YOUR SECRET";

    // Create the SwitchBotClient instance
    let client = std::sync::Arc::new(SwitchBotClient::new(token, secret));

    // Perform a GET request
    let devices = client.get_devices().await?;
    
    // Change the index if you have more than one device, currently it only works with the first device, this is becuase I only have one device.
    let first_device_id = std::sync::Arc::new(devices.first().unwrap().device_id.clone());
    let status = client.get_device_status(&first_device_id).await?;

    // Set the initial state of the UI
    ui.set_brightness(i32::try_from(status.brightness)?);
    ui.set_light_on(status.power == "on");
    ui.set_color_warmth(i32::try_from(status.color_temperature)? - 2700);

    ui.on_request_on_off({
        let ui_handle = ui.as_weak();
        let client = client.clone();
        let first_device_id = first_device_id.clone();
        move || {
            let ui = ui_handle.unwrap();
            if ui.get_light_on() {
                ui.set_light_on(false);
                let _ = client.device_off(&first_device_id);
            } else {
                ui.set_light_on(true);
                let _ = client.device_on(&first_device_id);
            }
        }
    });

    ui.on_request_change_brightness_value({
        let ui_handle = ui.as_weak();
        let client = client.clone();
        let first_device_id = first_device_id.clone();
        move || {
            let ui = ui_handle.unwrap();
            let _ = client.set_brightness(&first_device_id, ui.get_brightness());
        }
    });

    ui.on_request_change_color_value({
        let ui_handle = ui.as_weak();
        let client = client.clone();
        let first_device_id = first_device_id.clone();
        move || {
            let ui = ui_handle.unwrap();
            let _ = client.set_color(&first_device_id, format!("{}:{}:{}", ui.get_color_change().to_argb_u8().red, ui.get_color_change().to_argb_u8().green, ui.get_color_change().to_argb_u8().blue));
        }
    });

    ui.on_update_color({
        let ui_handle = ui.as_weak();
        let client = client.clone();
        let first_device_id = first_device_id.clone();
        move || {
            let ui = ui_handle.unwrap();
            let _ = client.set_color(&first_device_id, format!("{}:{}:{}", ui.get_color_change().to_argb_u8().red, ui.get_color_change().to_argb_u8().green, ui.get_color_change().to_argb_u8().blue));
        }
    });

    ui.on_request_change_warmth_value({
        let ui_handle = ui.as_weak();
        let client = client.clone();
        let first_device_id = first_device_id.clone();
        move || {
            let ui = ui_handle.unwrap();
            let new_warmth = ui.get_color_warmth() + 2700;
            let _ = client.set_color_temperature(&first_device_id, new_warmth);
        }
    });

    ui.run()?;

    Ok(())
}