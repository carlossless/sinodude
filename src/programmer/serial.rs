use super::super::part::*;
use super::*;
use log::{debug, info, warn};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use thiserror::Error;

// Serial protocol commands (must match firmware)
mod cmd {
    pub const CMD_PING: u8 = 0x01;
    pub const CMD_CONNECT: u8 = 0x02;
    pub const CMD_DISCONNECT: u8 = 0x03;
    pub const CMD_READ_FLASH: u8 = 0x04;
    pub const CMD_WRITE_FLASH: u8 = 0x05;
    pub const CMD_ERASE_FLASH: u8 = 0x06;
    pub const CMD_POWER_ON: u8 = 0x07;
    pub const CMD_POWER_OFF: u8 = 0x08;
    pub const CMD_GET_ID: u8 = 0x09;
    pub const CMD_SET_CONFIG: u8 = 0x0A;

    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

const CHUNK_SIZE: usize = 16;
const BAUD_RATE: u32 = 115200;
const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Error)]
pub enum SerialProgrammerError {
    #[error("Failed to open serial port: {0}")]
    PortOpenError(String),
    #[error("Serial I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Device not responding")]
    NoResponse,
    #[error("Invalid response from device")]
    InvalidResponse,
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Operation failed")]
    OperationFailed,
    #[error("Erase failed at address {0:#x}")]
    EraseFailed(u32),
    #[error("Write failed at address {0:#x}")]
    WriteFailed(u32),
    #[error("Verification failed at address {0:#x}")]
    VerificationFailed(u32),
}

pub struct SerialProgrammer {
    port: Box<dyn serialport::SerialPort>,
    chip_type: &'static Part,
    #[allow(dead_code)]
    power_setting: PowerSetting,
}

impl SerialProgrammer {
    pub fn new(
        port_name: &str,
        chip_type: &'static Part,
        power_setting: PowerSetting,
    ) -> Result<Self, SerialProgrammerError> {
        info!("Opening serial port: {}", port_name);

        let port = serialport::new(port_name, BAUD_RATE)
            .timeout(TIMEOUT)
            .open()
            .map_err(|e| SerialProgrammerError::PortOpenError(e.to_string()))?;

        // Give the Arduino time to reset after serial connection
        std::thread::sleep(Duration::from_secs(2));

        Ok(Self {
            port,
            chip_type,
            power_setting,
        })
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), SerialProgrammerError> {
        debug!("Sending command: {:#04x}", cmd);
        self.port.write_all(&[cmd])?;
        self.port.flush()?;
        Ok(())
    }

    fn send_bytes(&mut self, data: &[u8]) -> Result<(), SerialProgrammerError> {
        debug!("Sending {} bytes", data.len());
        self.port.write_all(data)?;
        self.port.flush()?;
        Ok(())
    }

    fn read_byte(&mut self) -> Result<u8, SerialProgrammerError> {
        let mut buf = [0u8; 1];
        self.port.read_exact(&mut buf)?;
        debug!("Received byte: {:#04x}", buf[0]);
        Ok(buf[0])
    }

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, SerialProgrammerError> {
        let mut buf = vec![0u8; len];
        self.port.read_exact(&mut buf)?;
        debug!("Received {} bytes", len);
        Ok(buf)
    }

    fn expect_ok(&mut self) -> Result<(), SerialProgrammerError> {
        let response = self.read_byte()?;
        if response == cmd::RSP_OK {
            Ok(())
        } else if response == cmd::RSP_ERR {
            Err(SerialProgrammerError::OperationFailed)
        } else {
            Err(SerialProgrammerError::InvalidResponse)
        }
    }

    pub fn ping(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Pinging programmer...");
        self.send_command(cmd::CMD_PING)?;

        let response = self.read_byte()?;
        if response != cmd::RSP_OK {
            return Err(SerialProgrammerError::NoResponse);
        }

        // Read signature bytes "SW"
        let sig = self.read_bytes(2)?;
        if sig != [b'S', b'W'] {
            return Err(SerialProgrammerError::InvalidResponse);
        }

        info!("Programmer responded successfully");
        Ok(())
    }

    pub fn power_on(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Powering on target...");
        self.send_command(cmd::CMD_POWER_ON)?;
        self.expect_ok()?;
        info!("Target powered on");
        // thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    pub fn power_off(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Powering off target...");
        self.send_command(cmd::CMD_POWER_OFF)?;
        self.expect_ok()?;
        info!("Target powered off");
        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Connecting to target MCU...");
        self.send_command(cmd::CMD_CONNECT)?;
        self.expect_ok().map_err(|_| SerialProgrammerError::ConnectionFailed)?;
        info!("Connected to target MCU");
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Disconnecting from target MCU...");
        self.send_command(cmd::CMD_DISCONNECT)?;
        self.expect_ok()?;
        info!("Disconnected from target MCU");
        Ok(())
    }

    pub fn get_id(&mut self) -> Result<(), SerialProgrammerError> {
        info!("Getting target MCU ID...");
        self.send_command(cmd::CMD_GET_ID)?;

        // Read response
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SerialProgrammerError::OperationFailed);
        }

        let mut id_bytes = [0u8; 2];
        for i in 0..2 {
            id_bytes[i] = self.read_byte()?;
        }

        let id = u16::from_le_bytes(id_bytes);
        info!("Target MCU ID: {:#06x}", id);
        Ok(())
    }

    pub fn set_config(&mut self) -> Result<(), SerialProgrammerError> {
        self.send_command(cmd::CMD_SET_CONFIG)?;
        self.send_bytes(&[self.chip_type.chip_type])?;
        self.expect_ok()?;
        info!("Configuration set for chip type: {:#04x}", self.chip_type.chip_type);
        Ok(())
    }

    pub fn get_part_number(&mut self) -> Result<(), SerialProgrammerError> {
        let custom_block_addr: u32 = match self.chip_type.custom_block {
            0x02 => 0x0A00,
            0x03 => 0x1200,
            0x04 => 0x2200,
            _ => return Err(SerialProgrammerError::OperationFailed),
        };
        self.send_command(cmd::CMD_READ_FLASH)?;
        self.send_bytes(&custom_block_addr.to_le_bytes())?; // Address
        self.send_bytes(&(16u16.to_le_bytes()))?; // Length
        self.send_bytes(&[0x01])?; // Custom block read
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SerialProgrammerError::OperationFailed);
        }
        let recv_len_l = self.read_byte()?; // Data length (low byte)
        let recv_len_h = self.read_byte()?; // Data length (high byte)
        let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
        if recv_len != 16 {
            return Err(SerialProgrammerError::InvalidResponse);
        }

        let data = self.read_bytes(16)?;
        let mut part_number = [0u8; 4];
        part_number.copy_from_slice(&data[9..13]);
        info!(
            "Target Part Number: {}",
            part_number.iter().map(|b| format!("{:02x}", b)).collect::<String>()
        );
        Ok(())
    }

    pub fn get_code_options(&mut self) -> Result<(), SerialProgrammerError> {
        let (options_addr, flash, size): (u32, bool, u16) = match (self.chip_type.custom_block, self.chip_type.chip_type) {
            (0x02, 0x02) => (0x0800, false, 64),
            (0x03, 0x02) => (0x1000, false, 64),
            (0x03, 0x07) => (0x1000, false, 512),
            (0x04, _) => (0x2000, false, 64),
            (0x06, _) => ((self.chip_type.flash_size - 32) as u32, true, 32),
            (_, _) => ((self.chip_type.flash_size - 64) as u32, true, 64),
        };

        debug!(
            "Reading code options from address {:#06x}, flash: {}, size: {}",
            options_addr,
            flash,
            size
        );

        let buffer_size = 16;
        let mut data = vec![0u8; 0];

        for addr in (options_addr..(options_addr + size as u32)).step_by(buffer_size) {
            debug!("Reading code options at address {:#06x}", addr);
            self.send_command(cmd::CMD_READ_FLASH)?;
            self.send_bytes(&addr.to_le_bytes())?; // Address
            self.send_bytes(&(buffer_size as u16).to_le_bytes())?; // Length
            self.send_bytes(&[if flash { 0x00 } else { 0x01 }])?; // Custom block read
            let response = self.read_byte()?;
            if response != cmd::RSP_DATA {
                return Err(SerialProgrammerError::OperationFailed);
            }
            let recv_len_l = self.read_byte()?; // Data length (low byte)
            let recv_len_h = self.read_byte()?; // Data length (high byte)
            let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
            if recv_len != buffer_size {
                return Err(SerialProgrammerError::InvalidResponse);
            }

            let result = self.read_bytes(buffer_size)?;
            data.extend_from_slice(&result);
        }

        info!(
            "Code options:\n{}",
            data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
        );

        Ok(())
    }

    pub fn read_init(&mut self) -> Result<(), SerialProgrammerError> {
        self.ping()?;
        self.power_on()?;
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        self.get_code_options()?;
        Ok(())
    }

    pub fn write_init(&mut self) -> Result<(), SerialProgrammerError> {
        self.ping()?;
        self.power_on()?;
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        self.get_code_options()?;
        Ok(())
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>, SerialProgrammerError> {
        let flash_size: u32 = self.chip_type.flash_size as u32;
        let mut contents = vec![0u8; 0];

        info!("Reading {} bytes from flash...", flash_size);

        let buffer_size = 16;

        for addr in (0..flash_size).step_by(buffer_size) {
            debug!("Reading code options at address {:#06x}", addr);
            self.send_command(cmd::CMD_READ_FLASH)?;
            self.send_bytes(&addr.to_le_bytes())?; // Address
            self.send_bytes(&(buffer_size as u16).to_le_bytes())?; // Length
            self.send_bytes(&[0x00])?; // Flash read
            let response = self.read_byte()?;
            if response != cmd::RSP_DATA {
                return Err(SerialProgrammerError::OperationFailed);
            }
            let recv_len_l = self.read_byte()?; // Data length (low byte)
            let recv_len_h = self.read_byte()?; // Data length (high byte)
            let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
            if recv_len != buffer_size {
                return Err(SerialProgrammerError::InvalidResponse);
            }

            let result = self.read_bytes(buffer_size)?;
            contents.extend_from_slice(&result);
        }

        info!("Flash read complete");
        Ok(contents)
    }

    pub fn read_chunk(&mut self, addr: u32, length: u16) -> Result<Vec<u8>, SerialProgrammerError> {
        debug!("Reading {} bytes at {:#x}", length, addr);
        self.send_command(cmd::CMD_READ_FLASH)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        // Send length (2 bytes, little endian)
        let len_bytes = length.to_le_bytes();
        self.send_bytes(&len_bytes)?;

        // Send flash read indicator
        self.send_bytes(&[0x00])?;

        // Expect data response
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SerialProgrammerError::OperationFailed);
        }

        // Read length
        let recv_len_l = self.read_byte()?; // Data length (low byte)
        let recv_len_h = self.read_byte()?; // Data length (high byte)
        let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]);
        if recv_len != length {
            return Err(SerialProgrammerError::InvalidResponse);
        }

        // Read data
        let data = self.read_bytes(length as usize)?;
        Ok(data)
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), SerialProgrammerError> {
        debug!("Erasing sector at {:#x}", addr);
        self.send_command(cmd::CMD_ERASE_FLASH)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        self.expect_ok().map_err(|_| SerialProgrammerError::EraseFailed(addr))
    }

    fn write_chunk(&mut self, addr: u32, data: &[u8]) -> Result<(), SerialProgrammerError> {
        debug!("Writing {} bytes at {:#x}", data.len(), addr);
        self.send_command(cmd::CMD_WRITE_FLASH)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        // Send length (2 bytes, little endian)
        let len = data.len() as u16;
        let len_bytes = len.to_le_bytes();
        self.send_bytes(&len_bytes)?;

        // Send data
        self.send_bytes(data)?;

        self.expect_ok().map_err(|_| SerialProgrammerError::WriteFailed(addr))
    }

    pub fn write_flash(&mut self, firmware: &[u8]) -> Result<(), SerialProgrammerError> {
        let flash_size = self.chip_type.flash_size.min(firmware.len());

        info!("Writing {} bytes to flash...", flash_size);

        // First, erase all sectors
        info!("Erasing flash...");
        for addr in (0..flash_size).step_by(self.chip_type.sector_size) {
            self.erase_sector(addr as u32)?;
            if addr % 4096 == 0 {
                info!("Erase progress: {}/{} bytes", addr, flash_size);
            }
        }
        info!("Erase complete");

        // Then write data in chunks
        info!("Programming flash...");
        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let chunk = &firmware[addr..end];
            self.write_chunk(addr as u32, chunk)?;

            if addr % 4096 == 0 {
                info!("Write progress: {}/{} bytes", addr, flash_size);
            }
        }
        info!("Programming complete");

        // Verify
        info!("Verifying flash...");
        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let expected = &firmware[addr..end];
            let actual = self.read_chunk(addr as u32, (end - addr) as u16)?;

            if expected != actual.as_slice() {
                warn!("Verification failed at address {:#x}", addr);
                warn!("Expected: {:02x?}", expected);
                warn!("Actual:   {:02x?}", actual);
                return Err(SerialProgrammerError::VerificationFailed(addr as u32));
            }

            if addr % 4096 == 0 {
                info!("Verify progress: {}/{} bytes", addr, flash_size);
            }
        }
        info!("Verification complete");

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), SerialProgrammerError> {
        self.disconnect()?;
        self.power_off()?;
        Ok(())
    }
}

impl Drop for SerialProgrammer {
    fn drop(&mut self) {
        // Best effort cleanup
        let _ = self.disconnect();
        let _ = self.power_off();
    }
}
