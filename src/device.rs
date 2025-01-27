use crate::battery::BatteryStatus;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct CorsairVoidDevice {
    pub id: String,
    fw_version_headset: Option<String>,
    fw_version_receiver: String,
    pub max_sidetone: u8,
    microphone_up: Option<bool>,
    pub battery_status: Option<BatteryStatus>,
}

impl CorsairVoidDevice {
    pub fn from_device_path(dev_path: &Path) -> Option<Self> {
        let id = dev_path.file_name()?.to_str()?.to_string();
        let fw_version_receiver = Self::read_sysfs_file(dev_path.join("fw_version_receiver"))?;

        let fw_version_headset =
            Self::read_sysfs_file(dev_path.join("fw_version_headset")).filter(|s| !s.is_empty());

        let max_sidetone = Self::read_sysfs_file(dev_path.join("sidetone_max"))?
            .parse::<u8>()
            .ok()?;

        let microphone_up = Self::read_sysfs_file(dev_path.join("microphone_up"))
            .filter(|s| !s.is_empty())
            .and_then(|s| Some(s.parse::<u8>().ok()? == 1));

        let battery_action_status =
            Self::read_sysfs_file(dev_path.join("power_supply/corsair-void-1-battery/status"));
        let battery_level =
            Self::read_sysfs_file(dev_path.join("power_supply/corsair-void-1-battery/capacity"))
                .and_then(|s| s.parse::<u8>().ok());

        let battery_status = BatteryStatus::parse(battery_action_status, battery_level);

        Some(CorsairVoidDevice {
            id,
            fw_version_headset,
            fw_version_receiver,
            max_sidetone,
            microphone_up,
            battery_status,
        })
    }

    pub fn read_sysfs_file<P: AsRef<Path>>(path: P) -> Option<String> {
        fn inner(path: &Path) -> Option<String> {
            if path.exists() {
                read_to_string(path).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        }
        inner(path.as_ref())
    }
}

impl Display for CorsairVoidDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Corsair Void device ID: {}", self.id)?;
        writeln!(f, "\tMax sidetone: {}", self.max_sidetone)?;

        writeln!(
            f,
            "\tReceiver firmware version: {}",
            self.fw_version_receiver
        )?;
        writeln!(
            f,
            "\tHeadset firmware version: {}",
            self.fw_version_headset.as_deref().unwrap_or("Disconnected")
        )?;

        writeln!(f, "\tMicrophone is {}", match self.microphone_up {
            Some(true) => "up",
            Some(false) => "down",
            None => "Disconnected",
        })?;

        let battery_status = self.battery_status.as_ref();
        writeln!(f, "\tBattery {}", match battery_status {
            Some(bs) => bs.status().to_string(),
            None => "Unknown".to_string(),
        })?;

        writeln!(f, "\tBattery level: {}", match battery_status {
            Some(bs) => format!("{}%", bs.level()),
            None => "Disconnected".to_string(),
        })
    }
}
