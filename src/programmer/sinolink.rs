use super::super::part::*;
use super::*;
use chrono::*;
use hex_literal::*;
use nusb::{
    self,
    list_devices,
    transfer::{Buffer, Bulk, ControlIn, ControlOut, ControlType, In, Out, Recipient, TransferError},
    ActiveConfigurationError, DeviceInfo, Endpoint, Error, Interface, MaybeFuture,
};
use std::{thread::sleep, time::Duration};
use thiserror::Error;

use log::{debug, info};

pub struct Sinolink<'a> {
    interface: Interface,
    chip_type: &'a Part,
    power_setting: PowerSetting,
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Configuration not found")]
    ConfigurationNotFound,
    #[error("Interface not found")]
    InterfaceNotFound,
    #[error("Active configuration error")]
    ConfigurationError(ActiveConfigurationError),
    #[error("Setup error")]
    SetupError(Error),
}

#[derive(Debug, Error, PartialEq)]
pub enum IOError {
    #[error("Write error")]
    WriteControlError(TransferError),
    #[error("Read error")]
    ReadControlError(TransferError),
    #[error("Write bulk error")]
    WriteBulkError(TransferError),
    #[error("Read bulk error")]
    ReadBulkError(TransferError),
}

const SINOLINK_DEVICE_ID: u16 = 0x258a;
const SINOLINK_VENDOR_ID: u16 = 0x5007;
const SINOLINK_CONFIGURATION_VALUE: u8 = 1;
const SINOLINK_INTERFACE_NUMBER: u8 = 0;

const TIMEOUT: Duration = Duration::from_secs(1);

fn bcdtoi(x: u8) -> u8 {
    let s = format!("{:02x}", x);
    s.parse::<u8>().unwrap()
}

fn bcdtodt(src: &[u8]) -> Option<NaiveDateTime> {
    println!("{:04x?}", src);
    let s: Vec<String> = src.iter().map(|x| format!("{:02x}", x)).collect();
    let datestring = s.join("");
    NaiveDateTime::parse_from_str(&datestring, "%y%m%d%H%M%S")
        .ok()
}

impl Sinolink<'static> {
    fn find_sinolink() -> Result<DeviceInfo, DeviceError> {
        for device in list_devices().wait().unwrap() {
            if device.vendor_id() == SINOLINK_DEVICE_ID && device.product_id() == SINOLINK_VENDOR_ID
            {
                return Ok(device);
            }
        }
        Err(DeviceError::DeviceNotFound)
    }

    pub fn new(chip_type: &'static Part, power_setting: PowerSetting) -> Result<Self, DeviceError> {
        // let device_info_temp = Self::find_sinolink()?;

        // debug!(
        //     "Bus {:03} Device {:03} ID {:04x}:{:04x}",
        //     device_info_temp.bus_id(),
        //     device_info_temp.device_address(),
        //     device_info_temp.vendor_id(),
        //     device_info_temp.product_id()
        // );

        // let device_temp = device_info_temp.open().map_err(DeviceError::SetupError)?;
        // debug!("Resetting device");
        // device_temp.reset().map_err(DeviceError::SetupError)?;

        // // Wait for device to be reeunmerated
        // sleep(Duration::from_secs(3));

        let device_info = Self::find_sinolink()?;
        let device = device_info.open().wait().map_err(DeviceError::SetupError)?;

        device.reset().wait().map_err(DeviceError::SetupError)?;

        sleep(Duration::from_secs(3));

        let device_info = Self::find_sinolink()?;
        let device = device_info.open().wait().map_err(DeviceError::SetupError)?;

        for interface in device.configurations() {
            debug!("{:?}", interface);
        }

        let _ = device
            .configurations()
            .find(|c: &nusb::descriptors::ConfigurationDescriptor| {
                c.configuration_value() == SINOLINK_CONFIGURATION_VALUE
            })
            .ok_or(DeviceError::ConfigurationNotFound)?;

        // let Some(config) = device
        //     .configurations()
        //     .find(|c| c.configuration_value() == SINOLINK_CONFIGURATION_VALUE)
        // else {
        //     return Err(DeviceError::ConfigurationNotFound);
        // };

        // device
        //     .set_configuration(config.configuration_value())
        //     .map_err(DeviceError::SetupError)?;

        let config = device.active_configuration();

        match config {
            Ok(config) => {
                if config.configuration_value() != SINOLINK_CONFIGURATION_VALUE {
                    device
                        .set_configuration(SINOLINK_CONFIGURATION_VALUE)
                        .wait()
                        .map_err(DeviceError::SetupError)?;
                }
            }
            Err(_e) => {
                device
                    .set_configuration(SINOLINK_CONFIGURATION_VALUE)
                    .wait()
                    .map_err(DeviceError::SetupError)?;
            }
        };

        let config = device
            .active_configuration()
            .map_err(DeviceError::ConfigurationError)?;

        let Some(interface) = config
            .interfaces()
            .find(|i: &nusb::descriptors::InterfaceDescriptors| {
                i.interface_number() == SINOLINK_INTERFACE_NUMBER
            })
        else {
            return Err(DeviceError::InterfaceNotFound);
        };

        let interface = device
            .claim_interface(interface.interface_number())
            .wait()
            .map_err(DeviceError::SetupError)?;

        Ok(Self {
            interface,
            chip_type,
            power_setting,
        })
    }

    fn get_info(&self) -> Result<(), IOError> {
        // dump example:
        // 220602144418 // version date - 2022-06-02 14:44:18
        // 0250 // version - 2.50
        // 1509230220010201
        // 1c0029000447333230 // programmer device serial number = 1C-00-29-00-04-47-33-32-30
        // 313537c0fc00c00000000000000000000000000000000002002202091456000230000000000000

        let buf = self.read_control(0, 0, 0, 64)?;
        let firmware_date = bcdtodt(&buf[0..6]).map(|d| d.and_utc());
        if let Some(firmware_date) = firmware_date {
            info!("Date: {}", firmware_date.format("%+"));
        } else {
            info!("Firmware date: Unknown");
        }

        let version_major: u8 = bcdtoi(buf[6]);
        let version_minor: u8 = bcdtoi(buf[7]);
        info!("Version: {}.{}", version_major, version_minor);

        // unknown [8..16]

        let serial_chunks: Vec<String> = buf[16..25].iter().map(|x| format!("{:02x}", x)).collect();
        info!("Serial: {}", serial_chunks.join("-"));

        // unknown [25..64]
        Ok(())
    }

    pub fn read_init(&self) -> Result<(), IOError> {
        self.get_info()?;

        self.read_chip(17, 0, 0, 0x0000, 0x0400)?;

        // seems like external power setting, don't see a difference between v5 and v3.3
        // this command is not sent when external power is used
        self.write_control(18, 1, 0, &[])?;

        self.configure_read()?;

        self.read_chip(17, 0, 0, 0x0000, 0x0400)?;

        self.read_control(24, 1, 0, 16)?;
        self.read_control(21, 1, 0, 2)?;
        self.read_control(64, 0x0101, 0, 16)?;
        // seems like a status call
        // buf == 00 00 00 00 when chip is successfully connected
        // buf == 09 11 22 00 when chip is not connected
        // or 09 00 00 00

        self.read_chip(70, 1, 0, 0, 0x10)?;

        self.read_chip(68, 1, 1, 0x1209, 0x40)?;

        self.read_control(22, 1, 0, 5)?;

        self.read_chip(68, 1, 1, 0x1200, 0x10)?;

        self.read_chip(68, 1, 1, 0x1006, 0x04)?;

        self.read_control(22, 1, 0, 5)?;

        self.read_chip(68, 1, 1, 0x1100, 0x04)?;

        self.read_control(22, 1, 0, 5)?;

        self.read_chip(68, 1, 1, 0x1000, 0x40)?;

        self.read_chip(68, 1, 1, 0x1040, 0x40)?;

        self.read_control(22, 1, 0, 5)?;

        Ok(())
    }

    pub fn write_init(&self) -> Result<(), IOError> {
        self.get_info()?; // actually done multiple times (13 times??)

        self.read_chip(17, 0, 0, 0x0000, 0x0400)?;

        self.write_control(18, 1, 0, &[])?;

        self.configure_write()?;

        self.read_chip(17, 0, 0, 0x0000, 0x0400)?;

        self.read_control(21, 1, 0, 2)?;

        self.read_control(64, 0x0101, 0, 4)?;
        // seems like a status call
        // buf == 00 00 00 00 when chip is successfully connected
        // buf == 09 11 22 00 when chip is not connected
        // or 09 00 00 00

        self.read_chip(70, 1, 0, 0, 0x10)?;

        self.read_chip(68, 1, 1, 0x1209, 0x40)?;

        self.read_control(22, 1, 0, 5)?;

        self.read_chip(68, 1, 1, 0x1200, 0x10)?;

        self.read_chip(68, 1, 1, 0x1100, 0x04)?;

        self.read_control(22, 1, 0, 5)?;

        self.read_chip(69, 1, 1, 0x0000, 0x10)?;

        let buf_w4: Vec<u8> = vec![0x00, 0x00, 0x00, 0x88];
        self.write_chip(66, 1, 1, 0x1100, 0x04, buf_w4)?;

        let buf_u5 = self.read_control(22, 1, 0, 5)?;
        if buf_u5[0] != 0x00 {
            panic!("woop!");
        }

        let mut buf_w64: Vec<u8> = vec![0; 64];
        buf_w64[4] = 0xc0;
        buf_w64[5] = 0x4a;
        buf_w64[6] = 0xa4;
        buf_w64[7] = 0xe0;
        buf_w64[8] = 0x63;
        buf_w64[9] = 0xc0;
        self.write_chip(66, 1, 1, 0x1000, 0x40, buf_w64)?;

        self.read_control(22, 1, 0, 5)?;

        let buf_w4: Vec<u8> = vec![0x0f, 0x00, 0x00, 0x88];
        self.write_chip(66, 1, 1, 0x1100, 0x04, buf_w4)?;

        self.read_control(22, 1, 0, 5)?;

        Ok(())
    }

    pub fn read_control(
        &self,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
    ) -> Result<Vec<u8>, IOError> {
        debug!("Read CONTROL: {:02} {:04x} {:04x}", request, value, index);
        self.interface
            .control_in(
                ControlIn {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Device,
                    request,
                    value,
                    index,
                    length,
                },
                TIMEOUT,
            )
            .wait()
            .map_err(IOError::ReadControlError)
    }

    pub fn write_control(
        &self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<(), IOError> {
        debug!("Write CONTROL: {:02} {:04x} {:04x}", request, value, index);
        debug!("COMMAND {:02x?}", data);
        self.interface
            .control_out(
                ControlOut {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Device,
                    request,
                    value,
                    index,
                    data,
                },
                TIMEOUT,
            )
            .wait()
            .map_err(IOError::WriteControlError)
    }

    pub fn read_chip(
        &self,
        request: u8,
        mode1: u8,
        mode2: u8,
        addr: u16,
        length: u16,
    ) -> Result<Vec<u8>, IOError> {
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

        self.write_control(request, 0, 0, &write_buf)?;

        let mut endpoint: Endpoint<Bulk, In> = self
            .interface
            .endpoint(0x81)
            .map_err(|_e| IOError::ReadBulkError(TransferError::Unknown(0)))?;

        let buffer_length = if length % 64 == 0 {
            length
        } else {
            length + (64 - (length % 64))
        };

        let buffer = Buffer::new(buffer_length.into());
        let completion = endpoint.transfer_blocking(buffer, TIMEOUT);
        completion
            .status
            .map_err(IOError::ReadBulkError)?;
        Ok(completion.buffer.iter().take(length as usize).cloned().collect())
    }

    pub fn write_chip(
        &self,
        request: u8,
        mode1: u8,
        mode2: u8,
        addr: u16,
        length: u16,
        buf: Vec<u8>,
    ) -> Result<(), IOError> {
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

        self.write_control(request, 0, 0, &write_buf)?;
        debug!("WRITING: {:?}", buf);

        let mut endpoint: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(0x02)
            .map_err(|_e| IOError::WriteBulkError(TransferError::Unknown(0)))?;

        let buffer_length = if buf.len() % 64 == 0 {
            buf.len()
        } else {
            buf.len() + (64 - (buf.len() % 64))
        };

        let mut buffer = Buffer::new(buffer_length.into());
        buffer.extend_from_slice(&buf);
        let completion = endpoint.transfer_blocking(buffer, TIMEOUT);
        completion
            .status
            .map_err(IOError::WriteBulkError)
    }

    pub fn configure_read(&self) -> Result<(), IOError> {
        let chip_type = self.chip_type;
        debug!("Sending config payload");
        let buf: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04,
        ];
        self.write_control(16, 0, 0, &buf)?;

        let mut config = vec![0; 1024];
        // Config structure:
        // 0-1: header (7887)
        // 2: checksum (bd)
        // 3: Chip Type (07)
        // 4: (00)
        // 5: Power setting - 0x02 - 5V, 0x01 - 3.3V, 0x03 - External
        // 6-13: (0402040000050000)
        // 14: CustomBlock (03)
        // 15: ProductBlock (01)
        // 16-47: (0620000000000000000800000000000000000000000000000000000000000008)
        // 47-55: code options (a4e063c00f000088)
        // 162-167: chip model (06080f09000a)
        // 181-185: PartNumber (68f90a0000)
        // 1008-1013: current date (230308203607 = 2023-03-08 20:36:07)
        config.copy_from_slice(&hex!("
            7887
            bd
            07
            00
            02
            0402040000050000
            03
            01
            0620000000000000000800000000000000000000000000000000000000000008
            a4e063c00f000088
            00000000000000000000010040ff0000fd8f3600000000000000000000000100b36300000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c11
            06080f09000a
            ff000000000000091200000500
            68f90a0000
            0000000000040000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000012000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
            230308203607
            05500000000000000000
        "));

        config[3] = chip_type.chip_type;

        config[5] = self.power_setting.to_byte();

        config[14] = chip_type.custom_block;
        config[15] = chip_type.product_block;

        config[47..47 + 8].clone_from_slice(&chip_type.default_code_options);

        config[162..162 + 6].clone_from_slice(&chip_type.model);

        config[181..181 + 5].clone_from_slice(&chip_type.part_number);

        let dt = Utc.with_ymd_and_hms(2023, 3, 8, 20, 36, 7).unwrap();
        let dt_string = dt.format("%y%m%d%H%M%S").to_string();
        let date_bytes: Vec<u8> = dt_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|number| u8::from_str_radix(&number, 16).unwrap())
            .collect();
        config[1008..1008 + 6].clone_from_slice(&date_bytes);

        let mut endpoint: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(0x02)
            .map_err(|_e| IOError::WriteBulkError(TransferError::Unknown(0)))?;

        let mut buffer = Buffer::new(config.len());
        buffer.extend_from_slice(&config);
        let completion = endpoint.transfer_blocking(buffer, TIMEOUT);
        completion
            .status
            .map_err(IOError::WriteBulkError)
    }

    pub fn configure_write(&self) -> Result<(), IOError> {
        let chip_type = self.chip_type;
        debug!("Sending config payload");
        let buf: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x04,
        ];
        self.write_control(0x40, 16, 0, &buf)?;

        let mut config: Vec<u8> = vec![0; 1024];

        config.copy_from_slice(&hex!("
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
        "));

        config[3] = chip_type.chip_type;

        config[5] = self.power_setting.to_byte();

        config[14] = chip_type.custom_block;
        config[15] = chip_type.product_block;

        config[47..47 + 8].clone_from_slice(&chip_type.default_code_options);

        config[162..162 + 6].clone_from_slice(&chip_type.model);

        config[181..181 + 5].clone_from_slice(&chip_type.part_number);

        let dt = Utc.with_ymd_and_hms(2023, 3, 8, 20, 36, 7).unwrap();
        let dt_string = dt.format("%y%m%d%H%M%S").to_string();
        let date_bytes: Vec<u8> = dt_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|number| u8::from_str_radix(&number, 16).unwrap())
            .collect();
        config[1008..1008 + 6].clone_from_slice(&date_bytes);

        let mut endpoint: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(0x02)
            .map_err(|_e| IOError::WriteBulkError(TransferError::Unknown(0)))?;

        let mut buffer = Buffer::new(config.len());
        buffer.extend_from_slice(&config);
        let completion = endpoint.transfer_blocking(buffer, TIMEOUT);
        completion
            .status
            .map_err(IOError::WriteBulkError)
    }

    pub fn read_flash(&self) -> Result<Vec<u8>, IOError> {
        let mut contents = vec![0; self.chip_type.flash_size];
        const PAGE_SIZE: usize = 64;
        for addr in (0..self.chip_type.flash_size).step_by(PAGE_SIZE) {
            let buf = self.read_chip(68, 0x01, 0x00, addr as u16, PAGE_SIZE as u16)?;
            contents[addr..(addr + PAGE_SIZE)].clone_from_slice(&buf[0..PAGE_SIZE]);
        }

        Ok(contents)
    }

    pub fn write_flash(&self, firmware: &[u8]) -> Result<(), IOError> {
        const PAGE_SIZE: usize = 1024;
        for addr in (0..self.chip_type.flash_size).step_by(PAGE_SIZE) {
            let data = Vec::from(&firmware[addr..(addr + PAGE_SIZE)]);
            self.write_chip(66, 0x01, 0x00, addr as u16, PAGE_SIZE as u16, data)?;
            let buf_5 = self.read_control(22, 0, 0, 5)?;
            assert!(buf_5[0] == 0x00);
        }
        Ok(())
    }
}
