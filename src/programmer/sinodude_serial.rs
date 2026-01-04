use super::super::part::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info, warn};
use std::io::{Read, Write};
use std::time::Duration;
use thiserror::Error;

// Expected firmware version (must match firmware)
const EXPECTED_VERSION_MAJOR: u8 = 1;

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
    pub const CMD_GET_VERSION: u8 = 0x0B;

    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

const CHUNK_SIZE: usize = 16;
const BAUD_RATE: u32 = 115200;
const TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Error)]
pub enum SinodudeSerialProgrammerError {
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
    #[error("Firmware version mismatch: expected major version {expected}, got {actual}")]
    VersionMismatch { expected: u8, actual: u8 },
    #[error("JTAG ID mismatch: expected {expected:#06x}, got {actual:#06x}")]
    JtagIdMismatch { expected: u16, actual: u16 },
    #[error("Part number mismatch: expected {expected}, got {actual}")]
    PartNumberMismatch { expected: String, actual: String },
}

pub struct SinodudeSerialProgrammer {
    port: Box<dyn serialport::SerialPort>,
    chip_type: &'static Part,
}

impl SinodudeSerialProgrammer {
    pub fn new(
        port_name: &str,
        chip_type: &'static Part,
    ) -> Result<Self, SinodudeSerialProgrammerError> {
        info!("Opening serial port: {}", port_name);

        let port = serialport::new(port_name, BAUD_RATE)
            .timeout(TIMEOUT)
            .open()
            .map_err(|e| SinodudeSerialProgrammerError::PortOpenError(e.to_string()))?;

        // Give the Arduino time to reset after serial connection
        std::thread::sleep(Duration::from_secs(2));

        Ok(Self { port, chip_type })
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), SinodudeSerialProgrammerError> {
        debug!("Sending command: {:#04x}", cmd);
        self.port.write_all(&[cmd])?;
        self.port.flush()?;
        Ok(())
    }

    fn send_bytes(&mut self, data: &[u8]) -> Result<(), SinodudeSerialProgrammerError> {
        debug!("Sending {} bytes", data.len());
        self.port.write_all(data)?;
        self.port.flush()?;
        Ok(())
    }

    fn read_byte(&mut self) -> Result<u8, SinodudeSerialProgrammerError> {
        let mut buf = [0u8; 1];
        self.port.read_exact(&mut buf)?;
        debug!("Received byte: {:#04x}", buf[0]);
        Ok(buf[0])
    }

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, SinodudeSerialProgrammerError> {
        let mut buf = vec![0u8; len];
        self.port.read_exact(&mut buf)?;
        debug!("Received {} bytes", len);
        Ok(buf)
    }

    fn expect_ok(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let response = self.read_byte()?;
        if response == cmd::RSP_OK {
            Ok(())
        } else if response == cmd::RSP_ERR {
            Err(SinodudeSerialProgrammerError::OperationFailed)
        } else {
            Err(SinodudeSerialProgrammerError::InvalidResponse)
        }
    }

    pub fn ping(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Pinging programmer...");
        self.send_command(cmd::CMD_PING)?;

        let response = self.read_byte()?;
        if response != cmd::RSP_OK {
            return Err(SinodudeSerialProgrammerError::NoResponse);
        }

        // Read signature bytes "SW"
        let sig = self.read_bytes(2)?;
        if sig != [b'S', b'W'] {
            return Err(SinodudeSerialProgrammerError::InvalidResponse);
        }

        info!("Programmer responded successfully");
        Ok(())
    }

    pub fn get_version(&mut self) -> Result<(u8, u8), SinodudeSerialProgrammerError> {
        info!("Getting firmware version...");
        self.send_command(cmd::CMD_GET_VERSION)?;

        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        let major = self.read_byte()?;
        let minor = self.read_byte()?;

        info!("Firmware version: {}.{}", major, minor);

        Ok((major, minor))
    }

    pub fn check_version(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let (major, _minor) = self.get_version()?;

        if major != EXPECTED_VERSION_MAJOR {
            return Err(SinodudeSerialProgrammerError::VersionMismatch {
                expected: EXPECTED_VERSION_MAJOR,
                actual: major,
            });
        }

        Ok(())
    }

    pub fn power_on(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Powering on target...");
        self.send_command(cmd::CMD_POWER_ON)?;
        self.expect_ok()?;
        info!("Target powered on");
        // thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    pub fn power_off(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Powering off target...");
        self.send_command(cmd::CMD_POWER_OFF)?;
        self.expect_ok()?;
        info!("Target powered off");
        Ok(())
    }

    pub fn connect(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Connecting to target MCU...");
        self.send_command(cmd::CMD_CONNECT)?;
        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::ConnectionFailed)?;
        info!("Connected to target MCU");
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Disconnecting from target MCU...");
        self.send_command(cmd::CMD_DISCONNECT)?;
        self.expect_ok()?;
        info!("Disconnected from target MCU");
        Ok(())
    }

    pub fn get_id(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        info!("Getting target MCU ID...");
        self.send_command(cmd::CMD_GET_ID)?;

        // Read response
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        let mut id_bytes = [0u8; 2];
        for i in 0..2 {
            id_bytes[i] = self.read_byte()?;
        }

        let id = u16::from_le_bytes(id_bytes);
        info!("Target MCU ID: {:#06x}", id);

        let expected_id = self.chip_type.jtag_id;
        if id != expected_id {
            return Err(SinodudeSerialProgrammerError::JtagIdMismatch {
                expected: expected_id,
                actual: id,
            });
        }

        Ok(())
    }

    pub fn set_config(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.send_command(cmd::CMD_SET_CONFIG)?;
        self.send_bytes(&[self.chip_type.chip_type])?;
        self.expect_ok()?;
        info!(
            "Configuration set for chip type: {:#04x}",
            self.chip_type.chip_type
        );
        Ok(())
    }

    pub fn get_part_number(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let custom_block_addr: u32 = match self.chip_type.custom_block {
            0x02 => 0x0A00,
            0x03 => 0x1200,
            0x04 => 0x2200,
            _ => return Err(SinodudeSerialProgrammerError::OperationFailed),
        };
        self.send_command(cmd::CMD_READ_FLASH)?;
        self.send_bytes(&custom_block_addr.to_le_bytes())?; // Address
        self.send_bytes(&(16u16.to_le_bytes()))?; // Length
        self.send_bytes(&[0x01])?; // Custom block read
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }
        let recv_len_l = self.read_byte()?; // Data length (low byte)
        let recv_len_h = self.read_byte()?; // Data length (high byte)
        let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
        if recv_len != 16 {
            return Err(SinodudeSerialProgrammerError::InvalidResponse);
        }

        let data = self.read_bytes(16)?;
        let mut part_number = [0u8; 5];
        part_number.copy_from_slice(&data[9..14]);

        let actual_str = part_number
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        info!("Target Part Number: {}", actual_str);

        let expected_part_number = self.chip_type.part_number;
        if part_number != expected_part_number {
            let expected_str = expected_part_number
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            return Err(SinodudeSerialProgrammerError::PartNumberMismatch {
                expected: expected_str,
                actual: actual_str,
            });
        }

        Ok(())
    }

    pub fn get_code_options(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let (options_addr, flash, size): (u32, bool, u16) =
            match (self.chip_type.custom_block, self.chip_type.chip_type) {
                (0x02, 0x02) => (0x0800, false, 64),
                (0x03, 0x02) => (0x1000, false, 64),
                (0x03, 0x07) => (0x1000, false, 512),
                (0x04, _) => (0x2000, false, 64),
                (0x06, _) => ((self.chip_type.flash_size - 32) as u32, true, 32),
                (_, _) => ((self.chip_type.flash_size - 64) as u32, true, 64),
            };

        debug!(
            "Reading code options from address {:#06x}, flash: {}, size: {}",
            options_addr, flash, size
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
                return Err(SinodudeSerialProgrammerError::OperationFailed);
            }
            let recv_len_l = self.read_byte()?; // Data length (low byte)
            let recv_len_h = self.read_byte()?; // Data length (high byte)
            let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
            if recv_len != buffer_size {
                return Err(SinodudeSerialProgrammerError::InvalidResponse);
            }

            let result = self.read_bytes(buffer_size)?;
            data.extend_from_slice(&result);
        }

        info!(
            "Code options:\n{}",
            data.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );

        Ok(())
    }

    pub fn read_init(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.ping()?;
        self.check_version()?;
        self.power_on()?;
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        self.get_code_options()?;
        Ok(())
    }

    pub fn write_init(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.ping()?;
        self.check_version()?;
        self.power_on()?;
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        self.get_code_options()?;
        Ok(())
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>, SinodudeSerialProgrammerError> {
        let flash_size: u32 = self.chip_type.flash_size as u32;
        let mut contents = vec![0u8; 0];

        info!("Reading {} bytes from flash...", flash_size);

        let buffer_size = 16;

        let progress = ProgressBar::new(flash_size as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("=>-"),
        );
        progress.set_message("Reading");

        for addr in (0..flash_size).step_by(buffer_size) {
            debug!("Reading flash at address {:#06x}", addr);
            self.send_command(cmd::CMD_READ_FLASH)?;
            self.send_bytes(&addr.to_le_bytes())?; // Address
            self.send_bytes(&(buffer_size as u16).to_le_bytes())?; // Length
            self.send_bytes(&[0x00])?; // Flash read
            let response = self.read_byte()?;
            if response != cmd::RSP_DATA {
                progress.abandon_with_message("Read failed");
                return Err(SinodudeSerialProgrammerError::OperationFailed);
            }
            let recv_len_l = self.read_byte()?; // Data length (low byte)
            let recv_len_h = self.read_byte()?; // Data length (high byte)
            let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
            if recv_len != buffer_size {
                progress.abandon_with_message("Read failed");
                return Err(SinodudeSerialProgrammerError::InvalidResponse);
            }

            let result = self.read_bytes(buffer_size)?;
            contents.extend_from_slice(&result);
            progress.set_position(addr as u64 + buffer_size as u64);
        }

        progress.finish_with_message("Read complete");
        Ok(contents)
    }

    pub fn read_chunk(
        &mut self,
        addr: u32,
        length: u16,
    ) -> Result<Vec<u8>, SinodudeSerialProgrammerError> {
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
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        // Read length
        let recv_len_l = self.read_byte()?; // Data length (low byte)
        let recv_len_h = self.read_byte()?; // Data length (high byte)
        let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]);
        if recv_len != length {
            return Err(SinodudeSerialProgrammerError::InvalidResponse);
        }

        // Read data
        let data = self.read_bytes(length as usize)?;
        Ok(data)
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), SinodudeSerialProgrammerError> {
        debug!("Erasing sector at {:#x}", addr);
        self.send_command(cmd::CMD_ERASE_FLASH)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::EraseFailed(addr))
    }

    fn write_chunk(&mut self, addr: u32, data: &[u8]) -> Result<(), SinodudeSerialProgrammerError> {
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

        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::WriteFailed(addr))
    }

    pub fn write_flash(&mut self, firmware: &[u8]) -> Result<(), SinodudeSerialProgrammerError> {
        let flash_size = self.chip_type.flash_size.min(firmware.len());
        let sector_size = self.chip_type.sector_size;

        info!("Writing {} bytes to flash...", flash_size);

        let style = ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-");

        // First, erase all sectors
        let erase_progress = ProgressBar::new(flash_size as u64);
        erase_progress.set_style(style.clone());
        erase_progress.set_message("Erasing");

        for addr in (0..flash_size).step_by(sector_size) {
            self.erase_sector(addr as u32).map_err(|e| {
                erase_progress.abandon_with_message("Erase failed");
                e
            })?;
            erase_progress.set_position((addr + sector_size) as u64);
        }
        erase_progress.finish_with_message("Erase complete");

        // Then write data in chunks
        let write_progress = ProgressBar::new(flash_size as u64);
        write_progress.set_style(style.clone());
        write_progress.set_message("Writing");

        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let chunk = &firmware[addr..end];
            self.write_chunk(addr as u32, chunk).map_err(|e| {
                write_progress.abandon_with_message("Write failed");
                e
            })?;
            write_progress.set_position(end as u64);
        }
        write_progress.finish_with_message("Write complete");

        // Verify
        let verify_progress = ProgressBar::new(flash_size as u64);
        verify_progress.set_style(style);
        verify_progress.set_message("Verifying");

        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let expected = &firmware[addr..end];
            let actual = self.read_chunk(addr as u32, (end - addr) as u16)?;

            if expected != actual.as_slice() {
                verify_progress.abandon_with_message("Verify failed");
                warn!("Verification failed at address {:#x}", addr);
                warn!("Expected: {:02x?}", expected);
                warn!("Actual:   {:02x?}", actual);
                return Err(SinodudeSerialProgrammerError::VerificationFailed(
                    addr as u32,
                ));
            }
            verify_progress.set_position(end as u64);
        }
        verify_progress.finish_with_message("Verify complete");

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.disconnect()?;
        self.power_off()?;
        Ok(())
    }
}

impl Drop for SinodudeSerialProgrammer {
    fn drop(&mut self) {
        // Best effort cleanup
        let _ = self.disconnect();
        let _ = self.power_off();
    }
}
