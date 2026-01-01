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

    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

const CHUNK_SIZE: usize = 64;
const SECTOR_SIZE: usize = 1024;
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

    pub fn read_init(&mut self) -> Result<(), SerialProgrammerError> {
        self.ping()?;
        // self.power_on()?;
        self.connect()?;
        Ok(())
    }

    pub fn write_init(&mut self) -> Result<(), SerialProgrammerError> {
        self.ping()?;
        self.power_on()?;
        self.connect()?;
        Ok(())
    }

    fn read_chunk(&mut self, addr: u32, len: u16) -> Result<Vec<u8>, SerialProgrammerError> {
        self.send_command(cmd::CMD_READ_FLASH)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        // Send length (2 bytes, little endian)
        let len_bytes = len.to_le_bytes();
        self.send_bytes(&len_bytes)?;

        // Read response
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SerialProgrammerError::OperationFailed);
        }

        // Read data length
        let data_len = {
            let len_buf = self.read_bytes(2)?;
            u16::from_le_bytes([len_buf[0], len_buf[1]]) as usize
        };

        // Read data
        let data = self.read_bytes(data_len)?;
        Ok(data)
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>, SerialProgrammerError> {
        let flash_size = self.chip_type.flash_size;
        let mut contents = vec![0u8; flash_size];

        info!("Reading {} bytes from flash...", flash_size);

        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            let chunk = self.read_chunk(addr as u32, CHUNK_SIZE as u16)?;
            contents[addr..addr + chunk.len()].copy_from_slice(&chunk);

            if addr % 4096 == 0 {
                info!("Read progress: {}/{} bytes", addr, flash_size);
            }
        }

        info!("Flash read complete");
        Ok(contents)
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
        for addr in (0..flash_size).step_by(SECTOR_SIZE) {
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
