mod battery;
mod device;
use crate::device::CorsairVoidDevice;
use battery::BatteryStatus;
use clap::{Parser, Subcommand};
use std::fs::{read_dir, write};
use std::path::Path;

#[derive(Parser)]
#[command(about = "Cli app to manage Corsair Void headsets", version)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Prints out information about the headset")]
    Info {
        #[arg(short, help = "Output as json", default_value = "false")]
        json: bool,
    },
    #[command(about = "Play a built-in notification from the headset")]
    SendAlert {
        #[arg(index = 1, help = "0 or 1")]
        alert: u8,
    },
    #[command(about = "Sets the sidetone")]
    SetSidetone {
        #[arg(index = 1, help = "a number from 0 to max-sidetone)")]
        sidetone: u8,
    },
    #[command(about = "Prints out information about the battery")]
    Battery {
        #[arg(short, help = "Output as json", default_value = "false")]
        json: bool,
    },
}
struct CorsairVoidInfo {
    devices: Vec<CorsairVoidDevice>,
}
impl CorsairVoidInfo {
    fn get_available_devices() -> Self {
        let devices_path = Path::new("/sys/bus/hid/drivers/hid-corsair-void/");
        let mut devices = Vec::new();

        if devices_path.exists() {
            if let Ok(entries) = read_dir(devices_path) {
                for entry in entries.flatten() {
                    if let Some(device) = CorsairVoidDevice::from_device_path(&entry.path()) {
                        devices.push(device);
                    }
                }
            }
        } else {
            eprintln!("Couldn't find any Corsair Void devices.");
        }

        CorsairVoidInfo { devices }
    }
}

fn main() {
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("This tool only works on linux");
        std::process::exit(1);
    }
    let args = Args::parse();
    let info = CorsairVoidInfo::get_available_devices();

    if info.devices.is_empty() {
        println!("No Corsair Void devices found.");
        return;
    }

    match args.commands {
        Commands::Info { json } => {
            if json {
                println!("{}", serde_json::to_string_pretty(&info.devices).unwrap());
                return;
            }
            for device in &info.devices {
                println!("{}", device);
            }
        }
        Commands::SendAlert { alert } => {
            if !(0..=1).contains(&alert) {
                eprintln!("Invalid alert value. Use 0 or 1.");
                return;
            }
            for device in &info.devices {
                let dev = format!(
                    "/sys/bus/hid/drivers/hid-corsair-void/{}/send_alert",
                    device.id
                );
                let alert_path = Path::new(&dev);
                if write(alert_path, alert.to_string()).is_ok() {
                    println!("Sent alert {} to device {}", alert, device.id);
                } else {
                    eprintln!("Failed to send alert to device {}", device.id);
                }
            }
        }
        Commands::SetSidetone { sidetone } => {
            for device in &info.devices {
                if sidetone > device.max_sidetone {
                    eprintln!(
                        "Sidetone value exceeds the maximum allowed ({}).",
                        device.max_sidetone
                    );
                    return;
                }
                let dev = format!("/sys/bus/hid/drivers/hid-corsair-void/{}", device.id);
                let sidetone_path = Path::new(&dev).join("set_sidetone");
                if write(sidetone_path, sidetone.to_string()).is_ok() {
                    println!("Set sidetone to {} for device {}", sidetone, device.id);
                } else {
                    eprintln!("Failed to set sidetone for device {}", device.id);
                }
            }
        }
        Commands::Battery { json } => {
            if json {
                let battery_info: Vec<BatteryStatus> = info
                    .devices
                    .iter()
                    .filter_map(|device| device.battery_status)
                    .collect();
                println!("{}", serde_json::to_string_pretty(&battery_info).unwrap());
                return;
            }
            for device in &info.devices {
                if let Some(battery_status) = &device.battery_status {
                    println!("Battery status: {}", battery_status.status());
                }
                if let Some(battery_level) = device.battery_status {
                    println!("Battery level: {}%", battery_level.level());
                }
            }
        }
    }
}
