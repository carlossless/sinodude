use super::super::parts::{
    find_parts_by_jtag_id, find_parts_by_part_number, format_parsed_options, parse_code_options,
    Part, Region, Voltage,
};
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

// Expected firmware version (must match firmware)
const EXPECTED_VERSION_MAJOR: u8 = 2;

// Serial protocol commands (must match firmware)
mod cmd {
    // System commands
    pub const CMD_PING: u8 = 0x01;
    pub const CMD_GET_VERSION: u8 = 0x02;

    // Connection
    pub const CMD_CONNECT: u8 = 0x03;
    pub const CMD_DISCONNECT: u8 = 0x04;

    // Identification
    pub const CMD_GET_ID: u8 = 0x05;

    // Configuration
    pub const CMD_SET_CONFIG: u8 = 0x06;
    pub const CMD_GET_CONFIG: u8 = 0x07;

    // Memory operations
    pub const CMD_READ_FLASH: u8 = 0x08;
    pub const CMD_WRITE_FLASH: u8 = 0x09;
    pub const CMD_ERASE_FLASH_SECTOR: u8 = 0x0A;
    pub const CMD_MASS_ERASE: u8 = 0x0B;
    pub const CMD_READ_CUSTOM_REGION: u8 = 0x0C;
    pub const CMD_WRITE_CUSTOM_REGION: u8 = 0x0D;

    // Response codes
    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

const CHUNK_SIZE: usize = 1024;
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
    #[error("Mass erase failed")]
    MassEraseFailed,
    #[error("Custom region write failed at address {0:#x}")]
    CustomRegionWriteFailed(u32),
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
    #[error("Operation cancelled")]
    Cancelled,
    #[error("Customer option length {provided} exceeds maximum {max}")]
    CustomerOptionLengthExceeded { provided: usize, max: usize },
    #[error("Non-editable bits modified at byte {byte}: provided {provided:#04x}, expected {expected:#04x} (mask {mask:#04x})")]
    NonEditableBitsModified {
        byte: usize,
        provided: u8,
        expected: u8,
        mask: u8,
    },
    #[error("Writing security bits is only supported for security_level 4 and chip_type 0x07 (got security_level {security_level}, chip_type {chip_type:#04x})")]
    UnsupportedSecurityWrite { security_level: u8, chip_type: u8 },
    #[error("Custom region verification failed at address {addr:#x}: expected {expected:02x?}, got {actual:02x?}")]
    CustomRegionVerificationFailed {
        addr: u32,
        expected: Vec<u8>,
        actual: Vec<u8>,
    },
    #[error("Part does not support 5.0V required by sinodude-serial programmer. Supported voltages: {supported}")]
    UnsupportedVoltage { supported: String },
}

pub struct SinodudeSerialProgrammer {
    port: Box<dyn serialport::SerialPort>,
    chip_type: &'static Part,
    connected: bool,
    cancelled: Arc<AtomicBool>,
    /// True if code options have non-editable bits that differ from defaults (use 0xc3 erase)
    use_alternate_erase: bool,
    /// Stored custom fields read from device during init
    stored_customer_id: Option<[u8; 4]>,
    stored_operation_number: Option<[u8; 2]>,
    stored_customer_option: Option<Vec<u8>>,
    stored_security: Option<Vec<u8>>,
    stored_serial_number: Option<[u8; 4]>,
}

impl SinodudeSerialProgrammer {
    pub fn new(
        port_name: &str,
        chip_type: &'static Part,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Self, SinodudeSerialProgrammerError> {
        // Check that the part supports 5.0V (required by sinodude-serial programmer)
        if !chip_type.compatible_voltages.contains(&Voltage::V5_0) {
            let supported = chip_type
                .compatible_voltages
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SinodudeSerialProgrammerError::UnsupportedVoltage { supported });
        }

        eprintln!("Opening serial port: {}", port_name);

        let port = serialport::new(port_name, BAUD_RATE)
            .timeout(TIMEOUT)
            .open()
            .map_err(|e| SinodudeSerialProgrammerError::PortOpenError(e.to_string()))?;

        // Give the Arduino time to reset after serial connection
        std::thread::sleep(Duration::from_secs(2));

        Ok(Self {
            port,
            chip_type,
            connected: false,
            cancelled,
            use_alternate_erase: false,
            stored_customer_id: None,
            stored_operation_number: None,
            stored_customer_option: None,
            stored_security: None,
            stored_serial_number: None,
        })
    }

    fn check_cancelled(&self) -> Result<(), SinodudeSerialProgrammerError> {
        if self.cancelled.load(Ordering::SeqCst) {
            Err(SinodudeSerialProgrammerError::Cancelled)
        } else {
            Ok(())
        }
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
        eprintln!("Pinging programmer...");
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

        eprintln!("Programmer responded successfully");
        Ok(())
    }

    pub fn get_version(&mut self) -> Result<(u8, u8), SinodudeSerialProgrammerError> {
        debug!("Getting firmware version...");
        self.send_command(cmd::CMD_GET_VERSION)?;

        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        let major = self.read_byte()?;
        let minor = self.read_byte()?;

        eprintln!("Firmware version: {}.{}", major, minor);

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

    pub fn connect(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        eprintln!("Connecting to target MCU...");
        self.send_command(cmd::CMD_CONNECT)?;
        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::ConnectionFailed)?;
        self.connected = true;
        eprintln!("Connected to target MCU");
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        eprintln!("Disconnecting from target MCU...");
        self.send_command(cmd::CMD_DISCONNECT)?;
        self.expect_ok()?;
        self.connected = false;
        eprintln!("Disconnected from target MCU");
        Ok(())
    }

    pub fn get_id(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        debug!("Getting target MCU ID...");
        self.send_command(cmd::CMD_GET_ID)?;

        // Read response
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        let mut id_bytes = [0u8; 2];
        for byte in &mut id_bytes {
            *byte = self.read_byte()?;
        }

        let id = u16::from_le_bytes(id_bytes);
        eprintln!("Target MCU ID: {:04x}", id);

        let expected_id = self.chip_type.jtag_id;
        if id != expected_id {
            let matching_parts = find_parts_by_jtag_id(id);
            if !matching_parts.is_empty() {
                eprintln!(
                    "Parts matching JTAG ID {:#06x}: {}",
                    id,
                    matching_parts.join(", ")
                );
            }
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
        debug!(
            "Configuration set for chip type: {:#04x}",
            self.chip_type.chip_type
        );
        Ok(())
    }

    pub fn get_config(&mut self) -> Result<u8, SinodudeSerialProgrammerError> {
        debug!("Getting firmware config...");
        self.send_command(cmd::CMD_GET_CONFIG)?;

        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }

        let chip_type = self.read_byte()?;
        debug!("Firmware chip type: {:#04x}", chip_type);

        Ok(chip_type)
    }

    pub fn get_part_number(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let custom_block_addr: u32 = match self.chip_type.custom_block {
            0x02 => 0x0A00,
            0x03 => 0x1200,
            0x04 => 0x2200,
            _ => return Err(SinodudeSerialProgrammerError::OperationFailed),
        };
        self.send_command(cmd::CMD_READ_CUSTOM_REGION)?;
        self.send_bytes(&custom_block_addr.to_le_bytes())?; // Address
        self.send_bytes(&(16u16.to_le_bytes()))?; // Length
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
        eprintln!("Target Part Number: {}", actual_str);

        let expected_part_number = self.chip_type.part_number;
        if part_number != expected_part_number {
            let expected_str = expected_part_number
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            let matching_parts = find_parts_by_part_number(&part_number);
            if !matching_parts.is_empty() {
                eprintln!(
                    "Parts matching part number {}: {}",
                    actual_str,
                    matching_parts.join(", ")
                );
            }
            return Err(SinodudeSerialProgrammerError::PartNumberMismatch {
                expected: expected_str,
                actual: actual_str,
            });
        }

        Ok(())
    }

    fn read_region(
        &mut self,
        region: Region,
        address: u32,
        size: usize,
    ) -> Result<Vec<u8>, SinodudeSerialProgrammerError> {
        debug!(
            "Reading {} bytes from {:?} at address {:#06x}",
            size, region, address
        );
        let cmd = match region {
            Region::Custom => cmd::CMD_READ_CUSTOM_REGION,
            Region::Flash => cmd::CMD_READ_FLASH,
        };
        self.send_command(cmd)?;
        self.send_bytes(&address.to_le_bytes())?;
        self.send_bytes(&(size as u16).to_le_bytes())?;
        let response = self.read_byte()?;
        if response != cmd::RSP_DATA {
            return Err(SinodudeSerialProgrammerError::OperationFailed);
        }
        let recv_len_l = self.read_byte()?;
        let recv_len_h = self.read_byte()?;
        let recv_len = u16::from_le_bytes([recv_len_l, recv_len_h]) as usize;
        if recv_len != size {
            return Err(SinodudeSerialProgrammerError::InvalidResponse);
        }
        self.read_bytes(size)
    }

    pub fn get_code_options(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        let region = self.chip_type.options_region();

        let customer_id_addr = self.chip_type.customer_id.address;

        // Read 64 bytes from customer_id address in one transaction
        const REGION_SIZE: usize = 64;
        let buffer = self.read_region(region, customer_id_addr, REGION_SIZE)?;

        // Extract customer_id (at offset 0)
        let customer_id: [u8; 4] = buffer[0..4].try_into().unwrap();
        eprintln!(
            "Customer ID: {}",
            customer_id
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
        self.stored_customer_id = Some(customer_id);

        // Extract operation_number
        let field = &self.chip_type.operation_number;
        let offset = (field.address - customer_id_addr) as usize;
        if offset + 2 <= REGION_SIZE {
            let operation_number: [u8; 2] = buffer[offset..offset + 2].try_into().unwrap();
            eprintln!(
                "Operation Number: {}",
                operation_number
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            );
            self.stored_operation_number = Some(operation_number);
        }

        // Extract security bits
        let field = &self.chip_type.security;
        let offset = (field.address - customer_id_addr) as usize;
        let security_len = self.chip_type.security_length();
        if offset + security_len <= REGION_SIZE {
            let security_bits = buffer[offset..offset + security_len].to_vec();
            eprintln!(
                "Security Bits: {}",
                security_bits
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            );
            self.stored_security = Some(security_bits);
        }

        // Extract serial_number
        let field = &self.chip_type.serial_number;
        let offset = (field.address - customer_id_addr) as usize;
        if offset + 4 <= REGION_SIZE {
            let serial_number: [u8; 4] = buffer[offset..offset + 4].try_into().unwrap();
            eprintln!(
                "Serial Number: {}",
                serial_number
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            );
            self.stored_serial_number = Some(serial_number);
        }

        // Extract and handle code options
        let field = &self.chip_type.customer_option;
        let option_byte_count = self.chip_type.option_byte_count;
        let offset = (field.address - customer_id_addr) as usize;
        let first_part_size = 4.min(option_byte_count);

        // First part from main buffer
        let mut code_options = buffer[offset..offset + first_part_size].to_vec();

        // Read upper bytes from 0x1100 in separate transaction if needed
        if option_byte_count > 4 {
            let second_part_size = option_byte_count - 4;
            let second_part = self.read_region(region, 0x1100, second_part_size)?;
            code_options.extend_from_slice(&second_part);
        }

        // Store the full customer_option (both parts)
        self.stored_customer_option = Some(code_options.clone());

        eprintln!(
            "Code Options: {}",
            code_options
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );

        // Check if non-editable bits differ from defaults (only upper bytes for >4-byte options)
        if let Some(expected_upper) = self.chip_type.upper_code_option_defaults() {
            let mask = self.chip_type.code_option_mask;
            for (i, &expected) in expected_upper.iter().enumerate() {
                let idx = 4 + i;
                if idx < code_options.len() {
                    let current = code_options[idx] & !mask[idx];
                    if current != expected {
                        eprintln!(
                                "Warning: Code option byte {} has non-editable bits that differ from defaults (current: {:#04x}, expected: {:#04x})",
                                idx, current, expected
                            );
                        self.use_alternate_erase = true;
                    }
                }
            }
        }

        // Parse and display options in user-friendly format
        let options_metadata = (self.chip_type.options)();
        let parsed = parse_code_options(&code_options, &options_metadata);
        eprintln!("Code Options (parsed):\n{}", format_parsed_options(&parsed));

        Ok(())
    }

    pub fn read_init(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.ping()?;
        self.check_version()?;
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
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        self.get_code_options()?;
        Ok(())
    }

    pub fn erase_init(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.ping()?;
        self.check_version()?;
        self.connect()?;
        self.get_id()?;
        self.set_config()?;
        self.get_part_number()?;
        Ok(())
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>, SinodudeSerialProgrammerError> {
        let flash_size: u32 = self.chip_type.flash_size as u32;
        let mut contents = vec![0u8; 0];

        eprintln!("Reading {} bytes from flash...", flash_size);

        let buffer_size: u16 = CHUNK_SIZE as u16;

        let progress = ProgressBar::new(flash_size as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("=>-"),
        );
        progress.set_message("Reading");

        let start = Instant::now();
        for addr in (0..flash_size).step_by(buffer_size as usize) {
            self.check_cancelled().inspect_err(|_| {
                progress.abandon_with_message("Cancelled");
            })?;
            let result = self.read_chunk(addr, buffer_size).inspect_err(|_| {
                progress.abandon_with_message("Read failed");
            })?;
            contents.extend_from_slice(&result);
            progress.set_position(addr as u64 + buffer_size as u64);
        }
        let elapsed = start.elapsed();

        progress.finish_with_message(format!("Read complete in {:.2?}", elapsed));
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
        self.send_command(cmd::CMD_ERASE_FLASH_SECTOR)?;

        // Send address (4 bytes, little endian)
        let addr_bytes = addr.to_le_bytes();
        self.send_bytes(&addr_bytes)?;

        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::EraseFailed(addr))
    }

    /// Erase sectors covering the given address range
    pub fn erase_sectors(
        &mut self,
        start_addr: u32,
        end_addr: u32,
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let sector_size = self.chip_type.sector_size as u32;
        let first_sector = start_addr / sector_size;
        let last_sector = (end_addr.saturating_sub(1)) / sector_size;
        let num_sectors = last_sector - first_sector + 1;

        eprintln!(
            "Erasing {} sector(s) from {:#x} to {:#x}...",
            num_sectors,
            first_sector * sector_size,
            (last_sector + 1) * sector_size
        );

        let progress = ProgressBar::new(num_sectors as u64);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
                .unwrap()
                .progress_chars("=>-"),
        );
        progress.set_message("Erasing");

        let start = Instant::now();
        for sector in first_sector..=last_sector {
            self.check_cancelled().inspect_err(|_| {
                progress.abandon_with_message("Cancelled");
            })?;
            let sector_addr = sector * sector_size;
            self.erase_sector(sector_addr).inspect_err(|_| {
                progress.abandon_with_message("Erase failed");
            })?;
            progress.inc(1);
        }
        let elapsed = start.elapsed();
        progress.finish_with_message(format!("Erase complete in {:.2?}", elapsed));

        Ok(())
    }

    pub fn mass_erase(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        if self.use_alternate_erase {
            eprintln!("Mass erasing flash (alternate mode due to non-default code options)...");
        } else {
            eprintln!("Mass erasing flash...");
        }
        let start = Instant::now();

        self.send_command(cmd::CMD_MASS_ERASE)?;
        // Send flag: 1 = alternate erase (0xc3), 0 = normal erase (0x4b)
        self.send_bytes(&[if self.use_alternate_erase { 1 } else { 0 }])?;
        self.expect_ok()
            .map_err(|_| SinodudeSerialProgrammerError::MassEraseFailed)?;

        let elapsed = start.elapsed();
        eprintln!("Mass erase complete in {:.2?}", elapsed);

        // Blank security and set high code option defaults
        self.blank_security_and_set_code_option_defaults()?;

        Ok(())
    }

    /// Blank security region and set high code option defaults after mass erase.
    /// For parts with >4-byte code options, writes the non-editable bits from defaults to 0x1100.
    fn blank_security_and_set_code_option_defaults(
        &mut self,
    ) -> Result<(), SinodudeSerialProgrammerError> {
        // Blank security region
        {
            let security = &self.chip_type.security;
            let security_length = self.chip_type.security_length();
            eprintln!(
                "Blanking security region at {:#x} ({} bytes)...",
                security.address, security_length
            );
            let zeros = vec![0u8; security_length];
            self.write_custom_region(security.address, &zeros)?;
        }

        // Set high code option defaults for parts with >4 byte options
        if let Some(upper) = self.chip_type.upper_code_option_defaults() {
            eprintln!(
                "Setting high code option defaults at {:#x} ({} bytes)...",
                0x1100,
                upper.len()
            );
            self.write_custom_region(0x1100, &upper)?;
        }

        Ok(())
    }

    pub fn write_custom_region(
        &mut self,
        addr: u32,
        data: &[u8],
    ) -> Result<(), SinodudeSerialProgrammerError> {
        debug!(
            "Writing {} bytes to custom region at {:#x}",
            data.len(),
            addr
        );
        self.send_command(cmd::CMD_WRITE_CUSTOM_REGION)?;

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
            .map_err(|_| SinodudeSerialProgrammerError::CustomRegionWriteFailed(addr))?;

        // Verify by reading back
        let read_back = self.read_region(Region::Custom, addr, data.len())?;
        if read_back != data {
            return Err(
                SinodudeSerialProgrammerError::CustomRegionVerificationFailed {
                    addr,
                    expected: data.to_vec(),
                    actual: read_back,
                },
            );
        }

        Ok(())
    }

    pub fn write_customer_id(
        &mut self,
        data: &[u8; 4],
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let field = &self.chip_type.customer_id;
        eprintln!("Writing customer ID at {:#x}...", field.address);
        self.write_custom_region(field.address, data)?;
        Ok(())
    }

    pub fn write_operation_number(
        &mut self,
        data: &[u8; 2],
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let field = &self.chip_type.operation_number;
        eprintln!("Writing operation number at {:#x}...", field.address);
        self.write_custom_region(field.address, data)?;
        Ok(())
    }

    pub fn write_customer_option(
        &mut self,
        data: &[u8],
    ) -> Result<(), SinodudeSerialProgrammerError> {
        // Validate length
        if data.len() > self.chip_type.option_byte_count {
            return Err(
                SinodudeSerialProgrammerError::CustomerOptionLengthExceeded {
                    provided: data.len(),
                    max: self.chip_type.option_byte_count,
                },
            );
        }

        // Validate that non-editable bits match default values
        let mask = self.chip_type.code_option_mask;
        let defaults = self.chip_type.default_code_options;
        for (i, &byte) in data.iter().enumerate() {
            if i < mask.len() && i < defaults.len() {
                // Non-editable bits are where mask is 0
                // Check that (provided & ~mask) == (default & ~mask)
                let non_editable_mask = !mask[i];
                let provided_non_editable = byte & non_editable_mask;
                let default_non_editable = defaults[i] & non_editable_mask;
                if provided_non_editable != default_non_editable {
                    return Err(SinodudeSerialProgrammerError::NonEditableBitsModified {
                        byte: i,
                        provided: byte,
                        expected: (byte & mask[i]) | default_non_editable,
                        mask: mask[i],
                    });
                }
            }
        }

        {
            let field = &self.chip_type.customer_option;
            // Split write: first 4 bytes to customer_option.address, rest to 0x1100
            // Write second region first, then the first region
            let first_part_size = 4.min(data.len());
            let second_part_size = data.len().saturating_sub(4);

            if second_part_size > 0 {
                eprintln!(
                    "Writing customer option ({} bytes) at {:#x}...",
                    second_part_size, 0x1100
                );
                self.write_custom_region(0x1100, &data[first_part_size..])?;
            }

            eprintln!(
                "Writing customer option ({} bytes) at {:#x}...",
                first_part_size, field.address
            );
            self.write_custom_region(field.address, &data[..first_part_size])?;
        }
        Ok(())
    }

    pub fn write_security(&mut self, data: &[u8]) -> Result<(), SinodudeSerialProgrammerError> {
        // Only security_level 4 and chip_type 0x07 are supported for writing security
        if self.chip_type.security_level != 4 || self.chip_type.chip_type != 0x07 {
            return Err(SinodudeSerialProgrammerError::UnsupportedSecurityWrite {
                security_level: self.chip_type.security_level,
                chip_type: self.chip_type.chip_type,
            });
        }

        let field = &self.chip_type.security;
        eprintln!("Writing security at {:#x}...", field.address);
        self.write_custom_region(field.address, data)?;
        Ok(())
    }

    pub fn write_serial_number(
        &mut self,
        data: &[u8; 4],
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let field = &self.chip_type.serial_number;
        eprintln!("Writing serial number at {:#x}...", field.address);
        self.write_custom_region(field.address, data)?;
        Ok(())
    }

    /// Write all custom fields in one transaction (from customer_id to +0x50).
    /// If `use_stored_defaults` is true, uses stored values from device as defaults
    /// for fields not provided by the user. If false, unprovided fields are left as zeros.
    pub fn write_custom_fields(
        &mut self,
        customer_id: Option<&[u8; 4]>,
        operation_number: Option<&[u8; 2]>,
        customer_option: Option<&[u8]>,
        security: Option<&[u8]>,
        serial_number: Option<&[u8; 4]>,
        use_stored_defaults: bool,
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let customer_id_addr = self.chip_type.customer_id.address;

        const REGION_SIZE: usize = 64;
        let mut buffer = [0u8; REGION_SIZE];

        // Calculate offsets relative to customer_id base address
        // Use provided value or fall back to stored value from device (if use_stored_defaults)
        {
            let stored = if use_stored_defaults {
                self.stored_customer_id.as_ref()
            } else {
                None
            };
            let data = customer_id.or(stored);
            if let Some(data) = data {
                // customer_id is at offset 0
                buffer[0..4].copy_from_slice(data);
            }
        }

        {
            let stored = if use_stored_defaults {
                self.stored_operation_number.as_ref()
            } else {
                None
            };
            let data = operation_number.or(stored);
            if let Some(data) = data {
                let field = &self.chip_type.operation_number;
                let offset = (field.address - customer_id_addr) as usize;
                if offset + 2 <= REGION_SIZE {
                    buffer[offset..offset + 2].copy_from_slice(data);
                }
            }
        }

        // Track upper part of customer_option for writing to 0x1100
        let mut customer_option_upper: Option<Vec<u8>> = None;
        {
            let stored = if use_stored_defaults {
                self.stored_customer_option.as_deref()
            } else {
                None
            };
            let data = customer_option.or(stored);
            let field = &self.chip_type.customer_option;
            let offset = (field.address - customer_id_addr) as usize;

            if let Some(data) = data {
                // First 4 bytes go to the buffer at customer_option offset
                let first_part_len = data.len().min(4).min(REGION_SIZE - offset);
                if offset < REGION_SIZE {
                    buffer[offset..offset + first_part_len]
                        .copy_from_slice(&data[..first_part_len]);
                }
                // Remaining bytes (if any) go to 0x1100
                if data.len() > 4 {
                    customer_option_upper = Some(data[4..].to_vec());
                }
            }
        }

        {
            let stored = if use_stored_defaults {
                self.stored_security.as_deref()
            } else {
                None
            };
            let data = security.or(stored);
            if let Some(data) = data {
                let field = &self.chip_type.security;
                let offset = (field.address - customer_id_addr) as usize;
                let len = data.len().min(REGION_SIZE - offset);
                if offset < REGION_SIZE {
                    buffer[offset..offset + len].copy_from_slice(&data[..len]);
                }
            }
        }

        {
            let stored = if use_stored_defaults {
                self.stored_serial_number.as_ref()
            } else {
                None
            };
            let data = serial_number.or(stored);
            if let Some(data) = data {
                let field = &self.chip_type.serial_number;
                let offset = (field.address - customer_id_addr) as usize;
                if offset + 4 <= REGION_SIZE {
                    buffer[offset..offset + 4].copy_from_slice(data);
                }
            }
        }

        eprintln!(
            "Writing custom fields region ({} bytes) at {:#x}...",
            REGION_SIZE, customer_id_addr
        );
        self.write_custom_region(customer_id_addr, &buffer)?;

        // Write upper part of customer_option to 0x1100 if present
        if let Some(upper) = customer_option_upper {
            eprintln!(
                "Writing customer option upper ({} bytes) at {:#x}...",
                upper.len(),
                0x1100
            );
            self.write_custom_region(0x1100, &upper)?;
        }

        Ok(())
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

        eprintln!("Writing {} bytes to flash...", flash_size);

        let style = ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-");

        // Then write data in chunks
        let write_progress = ProgressBar::new(flash_size as u64);
        write_progress.set_style(style.clone());
        write_progress.set_message("Writing");

        let start = Instant::now();
        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            self.check_cancelled().inspect_err(|_| {
                write_progress.abandon_with_message("Cancelled");
            })?;
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let chunk = &firmware[addr..end];
            self.write_chunk(addr as u32, chunk).inspect_err(|_| {
                write_progress.abandon_with_message("Write failed");
            })?;
            write_progress.set_position(end as u64);
        }
        let elapsed = start.elapsed();
        write_progress.finish_with_message(format!("Write complete in {:.2?}", elapsed));

        // Verify
        let verify_progress = ProgressBar::new(flash_size as u64);
        verify_progress.set_style(style);
        verify_progress.set_message("Verifying");

        let start = Instant::now();
        for addr in (0..flash_size).step_by(CHUNK_SIZE) {
            self.check_cancelled().inspect_err(|_| {
                verify_progress.abandon_with_message("Cancelled");
            })?;
            let end = (addr + CHUNK_SIZE).min(flash_size);
            let expected = &firmware[addr..end];
            let actual = self.read_chunk(addr as u32, (end - addr) as u16)?;

            if expected != actual.as_slice() {
                verify_progress.abandon_with_message("Verify failed");
                eprintln!("Verification failed at address {:#x}", addr);
                eprintln!("Expected: {:02x?}", expected);
                eprintln!("Actual:   {:02x?}", actual);
                return Err(SinodudeSerialProgrammerError::VerificationFailed(
                    addr as u32,
                ));
            }
            verify_progress.set_position(end as u64);
        }
        let elapsed = start.elapsed();
        verify_progress.finish_with_message(format!("Verify complete in {:.2?}", elapsed));

        Ok(())
    }

    /// Write a specific range of flash (addresses are inclusive of start, exclusive of end)
    pub fn write_flash_range(
        &mut self,
        firmware: &[u8],
        start_addr: usize,
        end_addr: usize,
    ) -> Result<(), SinodudeSerialProgrammerError> {
        let flash_size = self.chip_type.flash_size.min(firmware.len());
        let start_addr = start_addr.min(flash_size);
        let end_addr = end_addr.min(flash_size);
        let range_size = end_addr.saturating_sub(start_addr);

        if range_size == 0 {
            eprintln!("Nothing to write (empty range)");
            return Ok(());
        }

        eprintln!(
            "Writing {} bytes to flash (range {:#x}-{:#x})...",
            range_size, start_addr, end_addr
        );

        let style = ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-");

        // Write data in chunks
        let write_progress = ProgressBar::new(range_size as u64);
        write_progress.set_style(style.clone());
        write_progress.set_message("Writing");

        let start = Instant::now();
        for addr in (start_addr..end_addr).step_by(CHUNK_SIZE) {
            self.check_cancelled().inspect_err(|_| {
                write_progress.abandon_with_message("Cancelled");
            })?;
            let end = (addr + CHUNK_SIZE).min(end_addr);
            let chunk = &firmware[addr..end];
            self.write_chunk(addr as u32, chunk).inspect_err(|_| {
                write_progress.abandon_with_message("Write failed");
            })?;
            write_progress.set_position((end - start_addr) as u64);
        }
        let elapsed = start.elapsed();
        write_progress.finish_with_message(format!("Write complete in {:.2?}", elapsed));

        // Verify
        let verify_progress = ProgressBar::new(range_size as u64);
        verify_progress.set_style(style);
        verify_progress.set_message("Verifying");

        let start = Instant::now();
        for addr in (start_addr..end_addr).step_by(CHUNK_SIZE) {
            self.check_cancelled().inspect_err(|_| {
                verify_progress.abandon_with_message("Cancelled");
            })?;
            let end = (addr + CHUNK_SIZE).min(end_addr);
            let expected = &firmware[addr..end];
            let actual = self.read_chunk(addr as u32, (end - addr) as u16)?;

            if expected != actual.as_slice() {
                verify_progress.abandon_with_message("Verify failed");
                eprintln!("Verification failed at address {:#x}", addr);
                eprintln!("Expected: {:02x?}", expected);
                eprintln!("Actual:   {:02x?}", actual);
                return Err(SinodudeSerialProgrammerError::VerificationFailed(
                    addr as u32,
                ));
            }
            verify_progress.set_position((end - start_addr) as u64);
        }
        let elapsed = start.elapsed();
        verify_progress.finish_with_message(format!("Verify complete in {:.2?}", elapsed));

        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), SinodudeSerialProgrammerError> {
        self.disconnect()?;
        Ok(())
    }
}

impl Drop for SinodudeSerialProgrammer {
    fn drop(&mut self) {
        // Best effort cleanup, only if needed
        if self.connected {
            let _ = self.disconnect();
        }
    }
}
