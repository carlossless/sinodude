pub mod serial;
pub mod sinolink;
pub use serial::*;
pub use sinolink::*;

pub enum PowerSetting {
    Internal3v3,
    Internal5v,
    External,
}

impl PowerSetting {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Internal3v3 => 0x01,
            Self::Internal5v => 0x02,
            Self::External => 0x03,
        }
    }

    pub fn from_option(option: &str) -> PowerSetting {
        match option {
            "3v3" => Self::Internal3v3,
            "5v" => Self::Internal5v,
            "external" => Self::External,
            _ => unreachable!(),
        }
    }
}
