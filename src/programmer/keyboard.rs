use hidapi_rusb::*;
use std::{thread, time};
use ihex_ext::*;

pub struct Keyboard {
    device: HidDevice
}

const PAGE_SIZE: usize = 2048;
const FLASH_SIZE: usize = 61440; // 0xF000
// const FLASH_SIZE: usize = 65536; // 0xFFFF
const NUM_PAGES: usize = FLASH_SIZE / PAGE_SIZE;


impl Keyboard {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();
        if let Some(kb_device_info) = api.device_list().find(|d| d.vendor_id() == 0x5ac && d.product_id() == 0x024f) {
            println!("Found KB. Resetting into bootloader...");
            let device = kb_device_info.open_device(&api).unwrap();
            let mut buf_reset: [u8; 6] = [0x00; 6];
            buf_reset[0] = 0x05;
            buf_reset[1] = 0x75;
            device.send_feature_report(&buf_reset).unwrap_or_default(); // ignore errors, a reset will happen immedaatelly
            println!("Waiting for bootloader device...");
            thread::sleep(time::Duration::from_millis(2000));
        } else {
            println!("No KB found. Trying bootloader directly...");
        }

        let device = api.open(0x0603, 0x1020).unwrap();
        println!("Connected!");

        return Self {
            device
        }
    }

    pub fn new_test(&self) {
        let (mut firmware, _) = load_file_vec("private/kb_firmware.hex", FLASH_SIZE, 0).unwrap();
        let lenght = firmware.len();
        firmware[0] = 0x00; // 3rd byte is 0x66
        firmware[1] = 0x00; // 3rd byte is 0x66
        firmware[2] = 0x66; // 3rd byte is 0x66
        firmware[lenght - 5] = 0x00; // 5th last byte is 0x02, erase it
        firmware[lenght - 3] = 0x00; // 3rd last byte is 0x66, erase it

        self.erase();
        self.write(&firmware);
        let written = self.read();
        firmware[0] = 0x02;
        self.verify(&firmware, &written);
        self.finalize();
    }


    pub fn test() {
        let (mut firmware, _) = load_file_vec("private/kb_firmware.hex", FLASH_SIZE, 0).unwrap();

        let lenght = firmware.len();
        firmware[0] = 0x00; // 3rd byte is 0x66
        firmware[1] = 0x00; // 3rd byte is 0x66
        firmware[2] = 0x66; // 3rd byte is 0x66
        firmware[lenght - 5] = 0x00; // 5th last byte is 0x02, erase it
        firmware[lenght - 3] = 0x00; // 3rd last byte is 0x66, erase it

        let api = HidApi::new().unwrap();

        if let Some(kb_device_info) = api.device_list().find(|d| d.vendor_id() == 0x5ac && d.product_id() == 0x024f) {
            println!("Found KB. Resetting into bootloader...");
            let device = kb_device_info.open_device(&api).unwrap();
            let mut buf_reset: [u8; 6] = [0x00; 6];
            buf_reset[0] = 0x05;
            buf_reset[1] = 0x75;
            device.send_feature_report(&buf_reset).unwrap_or_default(); // ignore errors, a reset will happen immedaatelly
            println!("Waiting for bootloader device...");
            thread::sleep(time::Duration::from_millis(2000));
        } else {
            println!("No KB found. Trying bootloader directly...");
        }

        let device = api.open(0x0603, 0x1020).unwrap();
        println!("Connected!");

        // println!("Erasing...");
        // let mut buf_erase: [u8; 6] = [0x45; 6];
        // buf_erase[0] = 0x05;
        // device.send_feature_report(&buf_erase).unwrap();
        // thread::sleep(time::Duration::from_millis(2000));

        // println!("Writing...");
        // let mut buf_write_cmd: [u8; 6] = [0; 6];
        // buf_write_cmd[0] = 0x05;
        // buf_write_cmd[1] = 0x57;
        // buf_write_cmd[2] = 0;
        // buf_write_cmd[3] = 0;
        // buf_write_cmd[4] = (FLASH_SIZE & 0xff) as u8;
        // buf_write_cmd[5] = (FLASH_SIZE >> 8) as u8;

        // device.send_feature_report(&buf_write_cmd).unwrap();

        // let mut buf_write: [u8; PAGE_SIZE + 2] = [0; PAGE_SIZE + 2];
        // buf_write[0] = 0x06;
        // buf_write[1] = 0x77;

        // for i in 0..NUM_PAGES {
        //     buf_write[2..PAGE_SIZE + 2].clone_from_slice(&firmware[(i*PAGE_SIZE)..((i+1)*PAGE_SIZE)]);
        //     device.send_feature_report(&mut buf_write).unwrap();
        // }

        println!("Read Option...");
        let mut buf_write_cmd: [u8; 6] = [0; 6];
        buf_write_cmd[0] = 0x05;
        buf_write_cmd[1] = 0x55;
        buf_write_cmd[2] = 0;
        buf_write_cmd[3] = 0;
        buf_write_cmd[4] = (FLASH_SIZE & 0xff) as u8;
        buf_write_cmd[5] = (FLASH_SIZE >> 8) as u8;

        device.send_feature_report(&buf_write_cmd).unwrap();

        thread::sleep(time::Duration::from_millis(2000));

        println!("Reading...");
        firmware[0] = 0x02;

        let mut buf_read_cmd: [u8; 6] = [0; 6];
        buf_read_cmd[0] = 0x05;
        buf_read_cmd[1] = 0x52;
        buf_read_cmd[2] = 0;
        buf_read_cmd[3] = 0;
        buf_read_cmd[4] = (FLASH_SIZE & 0xff) as u8;
        buf_read_cmd[5] = (FLASH_SIZE >> 8) as u8;

        device.send_feature_report(&buf_read_cmd).unwrap();

        let mut buf_read: [u8; PAGE_SIZE + 2] = [0; PAGE_SIZE + 2];
        buf_read[0] = 0x06;
        buf_read[1] = 0x72;
        for i in 0..NUM_PAGES {
            device.get_feature_report(&mut buf_read).unwrap();

            for y in 0..PAGE_SIZE {
                if buf_read[y+2] != firmware[(i*PAGE_SIZE) + y] {
                    panic!("FIRMWARE MISMATCH @ 0x{:04x} | PAGE {} BYTE {} | == {:02x} != {:02x}", i*PAGE_SIZE+y, i, y, buf_read[y+2], firmware[(i*PAGE_SIZE) + y])
                }
            };

            // if buf_read[2..PAGE_SIZE + 2] != firmware[(i*PAGE_SIZE)..((i+1)*PAGE_SIZE)] {
            //     panic!("FIRMWARE MISMATCH @ PAGE {}", i)
            // }

            for chunk in buf_read[2..PAGE_SIZE + 2].chunks(16) {
                for x in &chunk[0..16] {
                    print!("{:02X?}", x);
                }
                println!();
            }
            buf_read[2..PAGE_SIZE + 2].fill(0);
        }

        println!("Finalizing...");
        let mut finalize_cmd: [u8; 6] = [0x55; 6];
        finalize_cmd[0] = 0x05;
        device.send_feature_report(&finalize_cmd).unwrap();
    }

    fn read(&self) -> Vec<u8> {
        let mut buf_read_cmd: [u8; 6] = [0; 6];
        buf_read_cmd[0] = 0x05;
        buf_read_cmd[1] = 0x52;
        buf_read_cmd[2] = 0;
        buf_read_cmd[3] = 0;
        buf_read_cmd[4] = (FLASH_SIZE & 0xff) as u8;
        buf_read_cmd[5] = (FLASH_SIZE >> 8) as u8;

        self.device.send_feature_report(&buf_read_cmd).unwrap();

        let mut result: Vec<u8> = vec![];
        let mut buf_read: [u8; PAGE_SIZE + 2] = [0; PAGE_SIZE + 2];
        buf_read[0] = 0x06;
        buf_read[1] = 0x72;
        for i in 0..NUM_PAGES {
            self.device.get_feature_report(&mut buf_read).unwrap();
            result.extend_from_slice(&buf_read[2..(PAGE_SIZE+2)]);
            buf_read[2..PAGE_SIZE + 2].fill(0);
        }
        return result;
    }

    fn write(&self, buffer: &Vec<u8>) {
        println!("Writing...");
        let mut buf_write_cmd: [u8; 6] = [0; 6];
        buf_write_cmd[0] = 0x05;
        buf_write_cmd[1] = 0x57;
        buf_write_cmd[2] = 0;
        buf_write_cmd[3] = 0;
        buf_write_cmd[4] = (FLASH_SIZE & 0xff) as u8;
        buf_write_cmd[5] = (FLASH_SIZE >> 8) as u8;

        self.device.send_feature_report(&buf_write_cmd).unwrap();

        let mut buf_write: [u8; PAGE_SIZE + 2] = [0; PAGE_SIZE + 2];
        buf_write[0] = 0x06;
        buf_write[1] = 0x77;

        for i in 0..NUM_PAGES {
            buf_write[2..PAGE_SIZE + 2].clone_from_slice(&buffer[(i*PAGE_SIZE)..((i+1)*PAGE_SIZE)]);
            self.device.send_feature_report(&mut buf_write).unwrap();
        }
    }

    fn erase(&self) {
        println!("Erasing...");
        let mut buf_erase: [u8; 6] = [0x45; 6];
        buf_erase[0] = 0x05;
        self.device.send_feature_report(&buf_erase).unwrap();
        thread::sleep(time::Duration::from_millis(2000));
    }

    fn verify(&self, expected: &Vec<u8>, actual: &Vec<u8>) -> bool {
        if expected.len() != actual.len() {
            panic!("LENGTH MISMATCH {} {}", expected.len(), actual.len());
            return false;
        }

        for i in 0..expected.len() {
            if expected[i] != actual[i] {
                panic!("FIRMWARE MISMATCH @ 0x{:04x} --- {:02x} != {:02x}", i, expected[i], actual[i]);
                return false;
            }
        }
        return true;
    }

    fn finalize(&self) {
        println!("Finalizing...");
        let mut finalize_cmd: [u8; 6] = [0x55; 6];
        finalize_cmd[0] = 0x05;
        self.device.send_feature_report(&finalize_cmd).unwrap();
    }
}
