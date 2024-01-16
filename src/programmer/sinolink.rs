use super::super::part::*;
use super::*;
use chrono::*;
use hex_literal::*;
use rusb::*;
use std::{thread, time::Duration};

use log::{debug, info};

/// Internal endpoint representations
#[derive(Debug, PartialEq, Clone)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

pub struct Sinolink<'a> {
    handle: DeviceHandle<GlobalContext>,
    chip_type: &'a Part,
    power_setting: PowerSetting,
}

impl Sinolink<'static> {
    fn find_sinolink() -> Device<GlobalContext> {
        for device in devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() == 0x258a && device_desc.product_id() == 0x5007 {
                return device;
            }
        }

        panic!("nope");
    }

    pub fn new(chip_type: &'static Part, power_setting: PowerSetting) -> Self {
        let device = Self::find_sinolink();

        let device_desc = device.device_descriptor().unwrap();

        debug!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );

        let mut handle = device.open().unwrap();

        // can fail, should retry
        handle.reset().unwrap();

        // Fetch base configuration
        // let languages = handle.read_languages(timeout).unwrap();
        let active_config = handle.active_configuration().unwrap();

        debug!("Active configuration: {}", active_config);

        let config_desc = device.config_descriptor(0).unwrap();

        let (mut write, mut read) = (None, None);

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // Create an endpoint container
                    let e = Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    };

                    debug!("Endpoint: {:?}", e);

                    // Find the relevant endpoints
                    match (endpoint_desc.transfer_type(), endpoint_desc.direction()) {
                        (TransferType::Bulk, Direction::In) => read = Some(e),
                        (TransferType::Bulk, Direction::Out) => write = Some(e),
                        (_, _) => continue,
                    }
                }
            }
        }

        handle.claim_interface(0).unwrap();
        handle.set_active_configuration(1).unwrap();

        let read_addr = read.unwrap().address;
        let write_addr = write.unwrap().address;

        debug!("READ & WRITE: {:02x} {:02x}", read_addr, write_addr);

        return Self {
            handle: handle,
            chip_type: chip_type,
            power_setting: power_setting,
        };
    }

    fn hdtoi(x: u8) -> u8 {
        let s = format!("{:02x}", x);
        return u8::from_str_radix(&s, 10).unwrap();
    }

    // fn hdatoi(src: &[u8], dst: &mut [u8]) {
    //     for i in 0..src.len() {
    //         dst[i] = Self::hdtoi(src[i]);
    //     }
    // }

    fn hdatodt(src: &[u8]) -> NaiveDateTime {
        let s: Vec<String> = src.into_iter().map(|x| format!("{:02x}", x)).collect();
        let datestring = s.join("");
        return NaiveDateTime::parse_from_str(&datestring, "%y%m%d%H%M%S").unwrap();
    }

    fn get_info(&self) {
        // dump example:
        // 220602144418 // version date - 2022-06-02 14:44:18
        // 0250 // version - 2.50
        // 1509230220010201
        // 1c0029000447333230 // programmer device serial number = 1C-00-29-00-04-47-33-32-30
        // 313537c0fc00c00000000000000000000000000000000002002202091456000230000000000000

        let mut buf: [u8; 64] = [0; 64];
        self.read_control(0xc0, 0, 0, 0, &mut buf);
        let firmware_date = Self::hdatodt(&buf[0..6]).and_utc();
        info!("Date: {}", firmware_date.format("%+"));

        let version_major: u8 = Self::hdtoi(buf[6]);
        let version_minor: u8 = Self::hdtoi(buf[7]);
        info!("Version: {}.{}", version_major, version_minor);

        // unknown [8..16]

        let serial_chunks: Vec<String> = buf[16..25]
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect();
        info!("Serial: {}", serial_chunks.join("-"));

        // unknown [25..64]
    }

    pub fn read_init(&self) {
        self.get_info();

        let mut buf2: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04,
        ];
        let mut buf: [u8; 1024] = [0; 1024];

        self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf);

        // seems like external power setting, don't see a difference between v5 and v3.3
        // this command is not sent when external power is used
        let b: [u8; 0] = [];
        self.write_control(0x40, 18, 1, 0, &b);

        self.configure_read();

        self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf);

        let mut buf_1: [u8; 16] = [0; 16];
        self.read_control(0xc0, 24, 1, 0, &mut buf_1);

        let mut buf_2: [u8; 2] = [0; 2];
        self.read_control(0xc0, 21, 1, 0, &mut buf_2);

        let mut buf_3: [u8; 16] = [0; 16];
        self.read_control(0xc0, 64, 0x0101, 0, &mut buf_3);
        // seems like a status call
        // buf == 00 00 00 00 when chip is successfully connected
        // buf == 09 11 22 00 when chip is not connected
        // or 09 00 00 00

        let mut buf_01: [u8; 0x1024] = [0; 0x1024];
        self.read_chip(70, 1, 0, 0, 0x10, &mut buf_01);

        self.read_chip(68, 1, 1, 0x1209, 0x40, &mut buf);

        let mut buf_4: [u8; 5] = [0; 5];
        self.read_control(0xc0, 22, 1, 0, &mut buf_4);

        self.read_chip(68, 1, 1, 0x1200, 0x10, &mut buf2);

        self.read_chip(68, 1, 1, 0x1006, 0x04, &mut buf2);

        let mut buf_5: [u8; 5] = [0; 5];
        self.read_control(0xc0, 22, 1, 0, &mut buf_5);

        self.read_chip(68, 1, 1, 0x1100, 0x04, &mut buf2);

        let mut buf_6: [u8; 5] = [0; 5];
        self.read_control(0xc0, 22, 1, 0, &mut buf_6);

        self.read_chip(68, 1, 1, 0x1000, 0x40, &mut buf);

        self.read_chip(68, 1, 1, 0x1040, 0x40, &mut buf);

        let mut buf_7: [u8; 5] = [0; 5];
        self.read_control(0xc0, 22, 1, 0, &mut buf_7);
    }

    pub fn write_init(&self) {
        self.get_info(); // actully done multiple times (13 times??)

        let mut buf_u1024: [u8; 1024] = [0; 1024];
        self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf_u1024);

        // is actually zero length
        let buf_u0: [u8; 0] = [0; 0];
        self.write_control(0x40, 18, 1, 0, &buf_u0);

        self.configure_write();

        self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf_u1024);

        let mut buf_u2: [u8; 2] = [0; 2];
        self.read_control(0xc0, 21, 1, 0, &mut buf_u2);

        let mut buf_u4: [u8; 4] = [0; 4];
        self.read_control(0xc0, 64, 0x0101, 0, &mut buf_u4);
        // seems like a status call
        // buf == 00 00 00 00 when chip is successfully connected
        // buf == 09 11 22 00 when chip is not connected
        // or 09 00 00 00

        let mut buf_u16: [u8; 16] = [0; 16];
        self.read_chip(70, 1, 0, 0, 0x10, &mut buf_u16);

        let mut buf_u64: [u8; 64] = [0; 64];
        self.read_chip(68, 1, 1, 0x1209, 0x40, &mut buf_u64);

        let mut buf_u5: [u8; 5] = [0; 5];
        self.read_control(0xc0, 22, 1, 0, &mut buf_u5);

        self.read_chip(68, 1, 1, 0x1200, 0x10, &mut buf_u16);

        self.read_chip(68, 1, 1, 0x1100, 0x04, &mut buf_u4);

        self.read_control(0xc0, 22, 1, 0, &mut buf_u5);

        self.read_chip(69, 1, 1, 0x0000, 0x10, &mut buf_u16);

        let buf_w4: [u8; 4] = [0x00, 0x00, 0x00, 0x88];
        self.write_chip(66, 1, 1, 0x1100, 0x04, &buf_w4);

        self.read_control(0xc0, 22, 1, 0, &mut buf_u5);
        if buf_u5[0] != 0x00 {
            panic!("woop!");
        }

        let mut buf_w64: [u8; 64] = [0; 64];
        buf_w64[4] = 0xc0;
        buf_w64[5] = 0x4a;
        buf_w64[6] = 0xa4;
        buf_w64[7] = 0xe0;
        buf_w64[8] = 0x63;
        buf_w64[9] = 0xc0;
        self.write_chip(66, 1, 1, 0x1000, 0x40, &buf_w64);

        self.read_control(0xc0, 22, 1, 0, &mut buf_u5);

        let buf_w4: [u8; 4] = [0x0f, 0x00, 0x00, 0x88];
        self.write_chip(66, 1, 1, 0x1100, 0x04, &buf_w4);

        self.read_control(0xc0, 22, 1, 0, &mut buf_u5);
    }

    pub fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &mut [u8],
    ) -> usize {
        debug!(
            "Read CONTROL: {:02x} {:02} {:04x} {:04x}",
            request_type, request, value, index
        );
        let result = self
            .handle
            .read_control(
                request_type,
                request,
                value,
                index,
                buf,
                Duration::new(2, 0),
            )
            .unwrap();
        debug!("RESULT {:02x?}", &buf[0..result]);
        return result;
    }

    pub fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
    ) -> usize {
        debug!(
            "Write CONTROL: {:02x} {:02} {:04x} {:04x}",
            request_type, request, value, index
        );
        debug!("COMMAND {:02x?}", buf);
        let result = self
            .handle
            .write_control(
                request_type,
                request,
                value,
                index,
                buf,
                Duration::new(2, 0),
            )
            .unwrap();
        return result;
    }

    pub fn read_chip(
        &self,
        request: u8,
        mode1: u8,
        mode2: u8,
        addr: u16,
        length: u16,
        buf: &mut [u8],
    ) {
        debug!(
            "Read CHIP: {:02} {:02x} {:02x} {:04x} {:04x}",
            request, mode1, mode2, addr, length
        );
        let write_buf: [u8; 16] = [
            0x00,
            mode1,
            (addr & 0xff) as u8,
            (addr >> 8) as u8,
            0x00,
            0x00,
            mode2,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            (length & 0xff) as u8,
            (length >> 8) as u8,
        ];

        self.write_control(0x40, request, 0, 0, &write_buf);
        self.handle
            .read_bulk(0x81, buf, Duration::new(2, 0))
            .unwrap();
    }

    pub fn write_chip(
        &self,
        request: u8,
        mode1: u8,
        mode2: u8,
        addr: u16,
        length: u16,
        buf: &[u8],
    ) {
        debug!(
            "Write CHIP: {:02} {:02x} {:02x} {:04x} {:04x}",
            request, mode1, mode2, addr, length
        );
        let write_buf: [u8; 16] = [
            0x00,
            mode1,
            (addr & 0xff) as u8,
            (addr >> 8) as u8,
            0x00,
            0x00,
            mode2,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            (length & 0xff) as u8,
            (length >> 8) as u8,
        ];

        self.write_control(0x40, request, 0, 0, &write_buf);
        debug!("WRITING: {:?}", buf);
        self.handle
            .write_bulk(0x02, buf, Duration::new(2, 0))
            .unwrap();
    }

    pub fn configure_read(&self) {
        let chip_type = self.chip_type;
        debug!("Sending config payload");
        let buf: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04,
        ];
        self.write_control(0x40, 16, 0, 0, &buf);

        let mut config: [u8; 1024] = hex!("
            7887
            bd // checksum???
            07 // Chip Type
            00
            02 // Power setting - 0x02 - 5V, 0x01 - 3.3V, 0x03 - External (3.3V or 5V seems to not matter)
            0402040000050000
            03 // CustomBlock
            01 // ProductBlock
            0620000000000000000800000000000000000000000000000000000000000008
            a4e063c00f000088 // code options
            00000000000000000000010040ff0000fd8f3600000000000000000000000100b36300000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c11
            06080f09000a // looks like chip model
            ff000000000000091200000500
            68f90a0000 // PartNumber
            0000000000040000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000012000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
            230308203607 // current date 2023-03-08 20:36:07
            05500000000000000000
        ");

        config[3] = chip_type.chip_type;

        config[5] = self.power_setting.to_byte();

        config[14] = chip_type.custom_block;
        config[15] = chip_type.product_block;

        config[47..47 + 8].clone_from_slice(&chip_type.default_code_options);

        config[162..162 + 6].clone_from_slice(&chip_type.model);

        config[181..181 + 5].clone_from_slice(&chip_type.part_number);

        let dt = Utc.with_ymd_and_hms(2023, 03, 08, 20, 36, 07).unwrap();
        let dt_string = dt.format("%y%m%d%H%M%S").to_string();
        let date_bytes: Vec<u8> = dt_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|number| u8::from_str_radix(&number, 16).unwrap())
            .collect();
        config[1008..1008 + 6].clone_from_slice(&date_bytes);

        self.handle
            .write_bulk(0x02, &config, Duration::new(2, 0))
            .unwrap();
    }

    pub fn configure_write(&self) {
        let chip_type = self.chip_type;
        debug!("Sending config payload");
        let buf: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04,
        ];
        self.write_control(0x40, 16, 0, 0, &buf);

        let mut config: [u8; 1024] = hex!("
            7887
            7a
            07
            00
            01
            0301040000050000
            03
            01
            06f2000000000000000800000000000000000000000000000000000000000008
            a4e063c00f000088
            00000000000000000000010040ff0000c04a6400000000000000000000000100865800000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c11
            06080f09000a
            ff000000000000091200000500
            68f90a0000
            0000000000040000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000012000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
            240115143216
            05500000000000000000
        ");

        config[3] = chip_type.chip_type;

        config[5] = self.power_setting.to_byte();

        config[14] = chip_type.custom_block;
        config[15] = chip_type.product_block;

        config[47..47 + 8].clone_from_slice(&chip_type.default_code_options);

        config[162..162 + 6].clone_from_slice(&chip_type.model);

        config[181..181 + 5].clone_from_slice(&chip_type.part_number);

        let dt = Utc.with_ymd_and_hms(2023, 03, 08, 20, 36, 07).unwrap();
        let dt_string = dt.format("%y%m%d%H%M%S").to_string();
        let date_bytes: Vec<u8> = dt_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|number| u8::from_str_radix(&number, 16).unwrap())
            .collect();
        config[1008..1008 + 6].clone_from_slice(&date_bytes);

        self.handle
            .write_bulk(0x02, &config, Duration::new(2, 0))
            .unwrap();
    }

    pub fn read_flash(&self) -> Vec<u8> {
        let mut contents = vec![0; self.chip_type.flash_size];
        const PAGE_SIZE: usize = 64;
        for addr in (0..self.chip_type.flash_size).step_by(PAGE_SIZE) {
            let mut buff: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
            self.read_chip(68, 0x01, 0x00, addr as u16, PAGE_SIZE as u16, &mut buff);
            contents[addr..(addr + PAGE_SIZE)].clone_from_slice(&buff[0..PAGE_SIZE]);
        }

        return contents;
    }

    pub fn write_flash(&self, firmware: &[u8]) -> Result<()> {
        const PAGE_SIZE: usize = 1024;
        for addr in (0..self.chip_type.flash_size).step_by(PAGE_SIZE) {
            self.write_chip(
                66,
                0x01,
                0x00,
                addr as u16,
                PAGE_SIZE as u16,
                &firmware[addr..(addr + PAGE_SIZE)],
            );
            let mut buf_5: [u8; 5] = [0; 5];
            self.read_control(0xc0, 22, 0, 0, &mut buf_5);
            assert!(buf_5[0] == 0x00);
        }
        Ok(())
    }
}
