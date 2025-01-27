use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BatteryActionStatus {
    Charging,
    Discharging,
    Full,
    Empty,
}

impl BatteryActionStatus {
    fn parse(s: Option<String>) -> Option<Self> {
        match s?.as_str() {
            "Charging" => Some(Self::Charging),
            "Discharging" => Some(Self::Discharging),
            "Full" => Some(Self::Full),
            "Empty" => Some(Self::Empty),
            _ => None,
        }
    }
}
impl Display for BatteryActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BatteryActionStatus::Charging => "Charging",
            BatteryActionStatus::Discharging => "Discharging",
            BatteryActionStatus::Full => "Full",
            BatteryActionStatus::Empty => "Empty",
        })
    }
}
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct BatteryStatus {
    status: BatteryActionStatus,
    level: u8,
}
impl BatteryStatus {
    pub fn parse(status: Option<String>, level: Option<u8>) -> Option<Self> {
        status.and_then(|status| {
            let status = BatteryActionStatus::parse(Some(status))?;
            level.map(|level| Self { status, level })
        })
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn status(&self) -> BatteryActionStatus {
        self.status
    }
}
