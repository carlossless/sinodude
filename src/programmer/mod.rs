pub mod sinolink;

pub enum PowerSetting {
    Internal3v3,
    Internal5v,
    External,
}

impl PowerSetting {
    pub fn to_byte(&self) -> u8 {
        return match self {
            Internal3v3 => 0x01,
            Internal5v => 0x02,
            External => 0x03,
        };
    }

    pub fn from_option(option: &str) -> PowerSetting {
        return match option {
            "3v3" => Self::Internal3v3,
            "5v" => Self::Internal5v,
            "external" => Self::External,
            _ => unreachable!()
        }
    }
}
