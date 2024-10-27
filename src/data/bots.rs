use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub enable_cloud_service: bool,
    pub hub_device_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorBulb {
    // {"body":{"brightness":63,"color":"255:255:255","colorTemperature":0,"deviceId":"84F7033B945A","deviceType":"Color Bulb","hubDeviceId":"84F7033B945A","power":"off","version":null},"message":"success","statusCode":100}
    pub brightness: i64,
    pub color: String,
    pub color_temperature: i64,
    pub device_id: String,
    pub device_type: String,
    pub hub_device_id: String,
    pub power: String,
}