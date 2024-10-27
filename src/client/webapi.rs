use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use sha2::Sha256;
use base64::{self, Engine};
use rand::Rng;
use ureq::{json, Agent, Request, Response};

use crate::data::bots::{ColorBulb, Device};

// This is all coming from: https://github.com/OpenWonderLabs/SwitchBotAPI
// Specifically: https://github.com/OpenWonderLabs/SwitchBotAPI?tab=readme-ov-file#color-bulb-3 
// Type alias for HMAC SHA256
type HmacSha256 = Hmac<Sha256>;
pub const NET_CONNECT_TIMEOUT: Duration = Duration::from_millis(8 * 1000);

pub const NET_IO_TIMEOUT: Duration = Duration::from_millis(16 * 1000);

#[derive(Clone)]
pub struct SwitchBotClient {
    token: String,
    secret: String,
    agent: Agent,
}

impl SwitchBotClient {
    pub fn new(token: &str, secret: &str) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(NET_CONNECT_TIMEOUT)
            .timeout_read(NET_IO_TIMEOUT)
            .timeout_write(NET_IO_TIMEOUT)
            .build();
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
            agent,
        }
    }

    // Function to generate HMAC signature
    fn generate_signature(&self, current_time: u64, nonce: &str) -> String {
        let data = format!("{}{}{}", self.token, current_time, nonce);
        
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        let signature = mac.finalize().into_bytes();
        
        base64::engine::general_purpose::STANDARD.encode(signature)
    }

    fn build_request(&self, method: &str, base_url: &str, url: &str) -> Result<Request, Box<ureq::Error>> {
        // Get current time
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap().as_millis() as u64;

        // Generate a nonce
        let nonce: String = rand::thread_rng().gen::<u128>().to_string();

        // Generate signature
        let signature = self.generate_signature(current_time, &nonce);

        // Create request based on method
        let request = self.agent
        .request(method, &format!("https://{}/{}", base_url, url))
            .set("Authorization", &self.token)
            .set("sign", &signature)
            .set("nonce", &nonce)
            .set("t", &current_time.to_string());
        
        Ok(request)
    }
    // Function to create an HTTP request
    fn request(&self, method: &str, path: &str) -> Result<Request, Box<ureq::Error>> {
        self.build_request(method, "api.switch-bot.com/v1.1", path)
    }

    fn get(&self, path: &str) -> Result<Request, Box<ureq::Error>> {
        self.request("GET", path)
    }

    fn post(&self, path: &str) -> Result<Request, Box<ureq::Error>> {
        self.request("POST", path)
    }

    fn with_retry(f: impl Fn() -> Result<Response, Box<ureq::Error>>) -> Result<Response, Box<ureq::Error>> {
        loop {
            let response = f()?;
            match response.status() {
                429 => {
                    let retry_after_secs = response
                        .header("Retry-After")
                        .and_then(|secs| secs.parse().ok())
                        .unwrap_or(2);
                    thread::sleep(Duration::from_secs(retry_after_secs));
                }
                _ => {
                    break Ok(response);
                }
            }
        }
    }

    /// Send a request and return the deserialized JSON body.  Use for GET
    /// requests.
    fn load<T: DeserializeOwned>(&self, request: Request) -> Result<T, Box<ureq::Error>> {
        let response = Self::with_retry(|| Ok(request.clone().call()?))?;
        let result = response.into_json().unwrap();
        Ok(result)
    }
}

impl SwitchBotClient {
    pub async fn get_devices(&self) -> Result<Vec<Device>, Box<ureq::Error>> {
        #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub body: Body,
            pub message: String,
            pub status_code: i64,
        }

        #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Body {
            pub device_list: Vec<Device>,
        }

        let request = self.get("devices")?;

        let result: Root = self.load(request)?;
        log::info!("Got {} devices", result.body.device_list.len());
        // This needs to be async
        Ok(result.body.device_list)
    }
    
    pub async fn get_device_status(&self, device_id: &str) -> Result<ColorBulb, Box<ureq::Error>> {
        #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub body: ColorBulb,
            pub message: String,
            pub status_code: i64,
        }

        let request = self.get(&format!("devices/{}/status", device_id))?;
        // This needs to be async
        let result: Root = self.load(request)?;

        log::info!("Getting device status for {}", device_id);
        Ok(result.body)
    }
    
    pub fn device_on(&self, device_id: &str) -> Result<(), Box<ureq::Error>> {
        self.post(&format!("devices/{}/commands", device_id))?
        .send_json(json!({  
            "command": "turnOn",
            "parameter": "default",
            "commandType": "command" }))?;
        
        log::info!("Turning on device {}", device_id);
        Ok(())
    }

    pub fn device_off(&self, device_id: &str) -> Result<(), Box<ureq::Error>> {
        self.post(&format!("devices/{}/commands", device_id))?
        .send_json(json!({  
            "command": "turnOff",
            "parameter": "default",
            "commandType": "command" }))?;

        log::info!("Turning off device {}", device_id);
        Ok(())
    }

    pub fn set_brightness(&self, device_id: &str, brightness: i32) -> Result<(), Box<ureq::Error>> {
        self.post(&format!("devices/{}/commands", device_id))?
        .send_json(json!({  
            "command": "setBrightness",
            "parameter": brightness,
            "commandType": "command" }))?;

        log::info!("Setting brightness to {} for device {}", brightness, device_id);
        Ok(())
    }

    pub fn set_color(&self, device_id: &str, color: String) -> Result<(), Box<ureq::Error>> {
        self.post(&format!("devices/{}/commands", device_id))?
        .send_json(json!({  
            "command": "setColor",
            "parameter": color,
            "commandType": "command" }))?;
        
        log::info!("Setting color to {} for device {}", color, device_id);
        Ok(())
    }

    pub fn set_color_temperature(&self, device_id: &str, temp: i32) -> Result<(), Box<ureq::Error>> {
        self.post(&format!("devices/{}/commands", device_id))?
        .send_json(json!({  
            "command": "setColorTemperature",
            "parameter": temp,
            "commandType": "command" }))?;

        log::info!("Setting temp to {} for device {}", temp, device_id);
        Ok(())
    }
}
