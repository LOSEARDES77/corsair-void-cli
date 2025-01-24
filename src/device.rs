use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
enum BatteryStatus {
    Charging,
    Discharging,
}
pub struct CorsairVoidDevice {
    pub id: String,
    fw_version_headset: String,
    fw_version_receiver: String,
    pub max_sidetone: u8,
    microphone_up: bool,
    #[allow(dead_code)]
    battery_status: Option<BatteryStatus>,
}

impl CorsairVoidDevice {
    pub fn from_device_path(dev_path: &Path) -> Option<Self> {
        let id = dev_path.file_name()?.to_str()?.to_string();
        let fw_version_headset = Self::read_sysfs_file(dev_path.join("fw_version_headset"))?;
        let fw_version_receiver = Self::read_sysfs_file(dev_path.join("fw_version_receiver"))?;
        let max_sidetone = Self::read_sysfs_file(dev_path.join("sidetone_max"))?.parse::<u8>().ok()?;
        let microphone_up = Self::read_sysfs_file(dev_path.join("microphone_up"))?
            .parse::<u8>()
            .map(|v| v == 1)
            .ok()?;

        Some(CorsairVoidDevice {
            id,
            fw_version_headset,
            fw_version_receiver,
            max_sidetone,
            microphone_up,
            battery_status: None,
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
        write!(f, "Corsair Void device ID: {}", self.id)?;

        write!(f, "Headset firmware version ({})", self.fw_version_headset)?;

        write!(f, "Receiver firmware version ({})", self.fw_version_receiver)?;

        write!(
            f,
            "Microphone is {}",
            if self.microphone_up { "up" } else { "down" }
        )
    }
}
