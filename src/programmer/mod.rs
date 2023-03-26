pub mod sinolink;
// pub mod keyboard;
use std::{thread, time};


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

use hidapi::HidApi;

pub struct Keyboard {
}

const page_size: usize = 2048;
// const flash_size: usize = 65536;
const flash_size: usize = 8192;
const num_pages: usize = flash_size / page_size;

impl Keyboard {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();
        let device = api.open(0x05ac, 0x024f).unwrap();

        println!("Erasing...");
        let mut buf_erase: [u8; 6] = [0x45; 6];
        buf_erase[0] = 0x05;
        device.send_feature_report(&buf_erase).unwrap();
        thread::sleep(time::Duration::from_millis(2000));

        println!("Writing...");
        let mut buf_write_cmd: [u8; 6] = [0; 6];
        buf_write_cmd[0] = 0x05;
        buf_write_cmd[1] = 0x57;
        buf_write_cmd[2] = 0;
        buf_write_cmd[3] = 0;
        buf_write_cmd[4] = (flash_size & 0xff) as u8;
        buf_write_cmd[5] = (flash_size >> 8) as u8;

        device.send_feature_report(&buf_write_cmd).unwrap();

        let mut buf_write: [u8; page_size + 2] = [0; page_size + 2];
        buf_write[0] = 0x06;
        buf_write[1] = 0x77;

        for i in 0..num_pages {
            println!("Sector {}", i);
            device.send_feature_report(&mut buf_write).unwrap();
        }

        println!("Reading...");
        let mut buf_read_cmd: [u8; 6] = [0; 6];
        buf_read_cmd[0] = 0x05;
        buf_read_cmd[1] = 0x52;
        buf_read_cmd[2] = 0;
        buf_read_cmd[3] = 0;
        buf_read_cmd[4] = (flash_size & 0xff) as u8;
        buf_read_cmd[5] = (flash_size >> 8) as u8;

        device.send_feature_report(&buf_read_cmd).unwrap();

        let mut buf_read: [u8; page_size + 2] = [0; page_size + 2];
        buf_read[0] = 0x06;
        buf_read[1] = 0x72;

        for i in 0..num_pages {
            let result = device.get_feature_report(&mut buf_read).unwrap();
            println!("{} {:?}", result, &buf_read[0..3]);
            for chunk in buf_read[2..page_size + 2].chunks(16) {
                for x in &chunk[0..16] {
                    print!("{:02X?}", x);
                }
                println!();
            }
        }

        return Keyboard {}
    }
}
