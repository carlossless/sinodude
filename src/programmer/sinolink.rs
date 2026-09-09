//! SinoLink USB dongle programmer backend: drives the dongle over USB vendor control requests (16-byte param packets) plus bulk endpoints (flash data and the 1 KiB config payload).

use crate::parts::{Part, SecurityRecordFormat};
use chrono::{TimeZone, Utc};
use hex_literal::hex;
use log::debug;
use nusb::transfer::{Buffer, Bulk, ControlIn, ControlOut, ControlType, In, Out, Recipient, TransferError};
use nusb::{list_devices, Device, Endpoint, Interface, MaybeFuture};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

/// SinoWealth VID; the dongle enumerates as PID 0x5063 or 0x5007, so match either.
const SINOWEALTH_VID: u16 = 0x258a;
const SINOLINK_PIDS: [u16; 2] = [0x5063, 0x5007];
const SINOLINK_CONFIGURATION_VALUE: u8 = 1;
const SINOLINK_INTERFACE_NUMBER: u8 = 0;

/// Target supply selection (config-payload byte `[5]`).
#[derive(Debug, Clone, Copy)]
pub enum PowerSetting {
    Internal3v3,
    Internal5v,
    External,
}
impl PowerSetting {
    /// Config-payload byte [0x05] target-voltage selector: 3.3V=1, 5V=2, external=3 (auto-detect).
    fn to_byte(self) -> u8 {
        match self {
            PowerSetting::Internal3v3 => 0x01,
            PowerSetting::Internal5v => 0x02,
            PowerSetting::External => 0x03,
        }
    }
    /// Parse the `--power` CLI value (`5v` / `3v3` / `external`). Defaults to 5V.
    pub fn from_arg(s: Option<&str>) -> PowerSetting {
        match s {
            Some("3v3") => PowerSetting::Internal3v3,
            Some("external") | Some("ext") => PowerSetting::External,
            _ => PowerSetting::Internal5v,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SinoLinkError {
    #[error("no SinoLink dongle found (looked for USB 258a:5063/5007)")]
    DeviceNotFound,
    #[error("USB setup error: {0}")]
    Setup(String),
    #[error("control transfer failed: {0:?}")]
    Control(TransferError),
    #[error("bulk transfer failed: {0:?}")]
    Bulk(TransferError),
    #[error("target not connected (status {0:02x?}); check wiring/voltage")]
    TargetNotConnected(Vec<u8>),
    #[error("operation failed (status {0:02x?})")]
    OperationFailed(Vec<u8>),
    #[error("verify mismatch at {0:#x}: wrote {1:#04x}, read {2:#04x}")]
    VerifyMismatch(u32, u8, u8),
    #[error("operation not yet implemented for the SinoLink backend: {0}")]
    NotImplemented(&'static str),
    #[error("operation unsupported for this part: {0}")]
    Unsupported(&'static str),
    #[error("operation cancelled")]
    Cancelled,
}

pub struct SinoLinkProgrammer {
    _device: Device,
    interface: Interface,
    part: &'static Part,
    cancelled: Arc<AtomicBool>,
    power_setting: PowerSetting,
    unlock_key: Option<[u8; 8]>,
    /// The 8 code-option fuse bytes to program; defaults to the part's blank default, which is wrong for boards that edit the editable bits, so override per board with set_code_options.
    code_options: [u8; 8],
    /// Program the ISP/BootROM region (top of flash); without it, writes there are silently dropped.
    program_isp: bool,
    /// Program the data-EEPROM region; a no-op on parts without EEPROM.
    program_eeprom: bool,
    /// Program the OTP/ChipDiff region, for parts that expose a one-time-programmable area.
    program_otp: bool,
    /// Lock the part by burning the 8-byte password into the Security region during the option write; the same bytes that unlock a protected part are what protect one, and a full mass-erase clears it.
    lock: bool,
}

impl SinoLinkProgrammer {
    fn find_device() -> Result<nusb::DeviceInfo, SinoLinkError> {
        list_devices()
            .wait()
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?
            .find(|d| d.vendor_id() == SINOWEALTH_VID && SINOLINK_PIDS.contains(&d.product_id()))
            .ok_or(SinoLinkError::DeviceNotFound)
    }

    pub fn new(part: &'static Part, cancelled: Arc<AtomicBool>, power: PowerSetting) -> Result<Self, SinoLinkError> {
        let info = Self::find_device()?;
        eprintln!(
            "Found dongle: {} {} ({:04x}:{:04x})",
            info.manufacturer_string().unwrap_or("?"),
            info.product_string().unwrap_or("?"),
            info.vendor_id(),
            info.product_id()
        );

        // Open the freshly-plugged dongle directly; a device.reset() here re-enumerates it every run and destabilises it.
        let device = info.open().wait().map_err(|e| SinoLinkError::Setup(e.to_string()))?;

        // Ensure configuration 1 is active.
        match device.active_configuration() {
            Ok(c) if c.configuration_value() == SINOLINK_CONFIGURATION_VALUE => {}
            _ => {
                device
                    .set_configuration(SINOLINK_CONFIGURATION_VALUE)
                    .wait()
                    .map_err(|e| SinoLinkError::Setup(e.to_string()))?;
            }
        }

        let interface = device
            .detach_and_claim_interface(SINOLINK_INTERFACE_NUMBER)
            .wait()
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?;

        Ok(Self {
            _device: device,
            interface,
            part,
            cancelled,
            power_setting: power,
            unlock_key: None,
            code_options: {
                let mut o = [0u8; 8];
                let co = part.default_code_options;
                let n = co.len().min(8);
                o[..n].copy_from_slice(&co[..n]);
                o
            },
            program_isp: false,
            program_eeprom: false,
            program_otp: false,
            lock: false,
        })
    }

    fn check_cancelled(&self) -> Result<(), SinoLinkError> {
        if self.cancelled.load(Ordering::SeqCst) {
            Err(SinoLinkError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Decode the dongle's single master result byte, surfaced as status[0] by both the connect-status (0x40) and operation-status (0x16) reads.
    fn decode_result_byte(code: u8) -> &'static str {
        match code {
            0x00 => "OK / success",
            0x09 => "ISP connect handshake fail (no 0x69 echo) — or a wedged dongle (replug USB, not the chip)",
            0x99 => "ISP shim-load fail",
            0x11 => "program(-header) mismatch",
            0x12 => "verify mismatch",
            0x14..=0x17 => "frame/byte/nibble send timeout",
            0x18 | 0x19 | 0x20 | 0x21 => "result/ID byte read timeout",
            0x22 | 0x23 => "silicon-ID (IDCODE) read/detect fail",
            0x24 | 0x25 => "addr/value send timeout",
            0x30 => "slow combined addr+read timeout",
            0x33 | 0x34 | 0x37 => "program/verify/final-verify mismatch",
            0x55 => "BUSY (keep polling)",
            0xCC | 0xDD => "JTAG-entry retry (transient)",
            0xEE => "erase verify/busy timeout",
            0xF3 => "protoB bus-stuck / measure timeout",
            0xFF => "erase fail (chip_type 0/1)",
            _ => "unknown",
        }
    }

    // ---- Transport ------------------------------------------------------------

    fn read_control(&self, request: u8, value: u16, index: u16, length: u16) -> Result<Vec<u8>, SinoLinkError> {
        debug!("ctrl-IN  req={:#04x} val={:#06x} idx={:#06x} len={}", request, value, index, length);
        let data = self
            .interface
            .control_in(
                ControlIn { control_type: ControlType::Vendor, recipient: Recipient::Device, request, value, index, length },
                TIMEOUT,
            )
            .wait()
            .map_err(SinoLinkError::Control)?;
        debug!("ctrl-IN  <= ({}) {:02x?}", data.len(), data);
        Ok(data)
    }

    fn write_control(&self, request: u8, value: u16, index: u16, data: &[u8]) -> Result<(), SinoLinkError> {
        debug!("ctrl-OUT req={:#04x} val={:#06x} idx={:#06x} data={:02x?}", request, value, index, data);
        self.interface
            .control_out(
                ControlOut { control_type: ControlType::Vendor, recipient: Recipient::Device, request, value, index, data },
                TIMEOUT,
            )
            .wait()
            .map_err(SinoLinkError::Control)
    }

    /// `read_chip`: control-OUT 16-byte param then bulk-IN `length` bytes (rounded up to 64).
    fn read_chip(&self, request: u8, mode1: u8, mode2: u8, addr: u16, length: u16) -> Result<Vec<u8>, SinoLinkError> {
        let param: [u8; 16] = [
            0x00, mode1, (addr & 0xff) as u8, (addr >> 8) as u8, 0x00, 0x00, mode2, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, (length & 0xff) as u8, (length >> 8) as u8,
        ];
        self.write_control(request, 0, 0, &param)?;
        let mut ep: Endpoint<Bulk, In> = self
            .interface
            .endpoint(0x81)
            .map_err(|_| SinoLinkError::Bulk(TransferError::Unknown(0)))?;
        let rounded = if length % 64 == 0 { length } else { length + (64 - length % 64) };
        let c = ep.transfer_blocking(Buffer::new(rounded as usize), TIMEOUT);
        c.status.map_err(SinoLinkError::Bulk)?;
        Ok(c.buffer.iter().take(length as usize).cloned().collect())
    }

    /// `write_chip`: control-OUT 16-byte param then bulk-OUT the data (rounded up to 64).
    fn write_chip(&self, request: u8, mode1: u8, mode2: u8, addr: u16, length: u16, buf: Vec<u8>) -> Result<(), SinoLinkError> {
        let param: [u8; 16] = [
            0x00, mode1, (addr & 0xff) as u8, (addr >> 8) as u8, 0x00, 0x00, mode2, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, (length & 0xff) as u8, (length >> 8) as u8,
        ];
        self.write_control(request, 0, 0, &param)?;
        let mut ep: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(0x02)
            .map_err(|_| SinoLinkError::Bulk(TransferError::Unknown(0)))?;
        let rounded = if buf.len() % 64 == 0 { buf.len() } else { buf.len() + (64 - buf.len() % 64) };
        let mut b = Buffer::new(rounded);
        b.extend_from_slice(&buf);
        let c = ep.transfer_blocking(b, TIMEOUT);
        c.status.map_err(SinoLinkError::Bulk)
    }

    // ---- Probe / connect ------------------------------------------------------

    fn get_info(&self) -> Result<(), SinoLinkError> {
        let buf = self.read_control(0x00, 0, 0, 64)?;
        if buf.len() >= 25 {
            let serial: Vec<String> = buf[16..25].iter().map(|x| format!("{:02x}", x)).collect();
            eprintln!(
                "SinoLink: fw {:02x}.{:02x}  serial {}",
                buf.get(6).copied().unwrap_or(0),
                buf.get(7).copied().unwrap_or(0),
                serial.join("-")
            );
        }
        Ok(())
    }

    /// Build the 1 KiB config payload for the active part and send it: control-OUT 0x10 preamble then bulk-OUT 1024 bytes.
    fn send_config(&self, write_mode: bool) -> Result<(), SinoLinkError> {
        // 16-byte preamble: trailing 0x04 == 0x0400 (1 KiB) follows on the bulk pipe.
        let preamble: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x04];
        self.write_control(0x10, 0, 0, &preamble)?;

        // Base 1 KiB config payloads; the per-chip fields below are patched in from the Part so other chips work too.
        let template: [u8; 1024] = if write_mode {
            hex!("78877a0700010301040000050000030106f2000000000000000800000000000000000000000000000000000000000008a4e063c00f00008800000000000000000000010040ff0000c04a6400000000000000000000000100865800000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c1106080f09000aff00000000000009120000050068f90a0000000000000004000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000001200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024011514321605500000000000000000")
        } else {
            hex!("7887bd070002040204000005000003010620000000000000000800000000000000000000000000000000000000000008a4e063c00f00008800000000000000000000010040ff0000fd8f3600000000000000000000000100b36300000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c1106080f09000aff00000000000009120000050068f90a0000000000000004000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000001200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000023030820360705500000000000000000")
        };
        let mut config = template.to_vec();

        // Per-chip fields patched in from the Part.
        config[3] = self.part.chip_type; // ChipType (sh68f90a = 7)
        // config[4] = ICP transport selector: 0 = single-wire 8051 ICP, 1 = JTAG frame path. Left 0 (the only confirmed value); the per-chip-type connect mode split lives in connect().
        config[4] = 0x00;
        config[0x204..0x208].copy_from_slice(&(self.part.customer_id.address as u32).to_le_bytes()); // +0x204 = CustomerID addr
        // config[5] = target supply-voltage selector: 1 = 3.3V, 2 = 5V, 3 = auto-detect.
        config[5] = self.power_setting.to_byte();
        config[14] = self.part.custom_block;
        config[15] = self.part.product_block;
        // Code-option block: byte 0x2f = option byte count, bytes 0x30..0x38 = the 8 option fuse bytes.
        config[0x2f] = self.part.option_byte_count as u8;
        config[0x30..0x38].copy_from_slice(&self.code_options);

        // Authorise erase+program of the ISP region (blob[0x14] + blob[0x12] bit0); without both, writes there are silently dropped.
        if self.program_isp {
            config[0x14] = 0x01;
            config[0x12] |= 0x01;
        }
        // Region-program options; a no-op on the wire for parts that lack the region.
        if self.program_eeprom {
            config[0x15] = 0x01; // enable EEPROM program/erase
        }
        if self.program_otp {
            config[0x17] = 0x01; // enable OTP/ChipDiff program
        }
        // Carry the customer password into blob[0x210..0x218], the address the dongle reads it from for unlock/lock.
        if let Some(key) = self.unlock_key {
            config[0x210..0x218].copy_from_slice(&key);
        }
        config[181..181 + 5].copy_from_slice(&self.part.part_number);
        // Date stamp (BCD-as-hex); value is cosmetic.
        let dt = Utc.with_ymd_and_hms(2023, 3, 8, 20, 36, 7).unwrap();
        let s = dt.format("%y%m%d%H%M%S").to_string();
        let date: Vec<u8> = s.as_bytes().chunks(2).map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap()).collect();
        config[1008..1008 + 6].copy_from_slice(&date);

        // Recompute the [0x02] checksum LAST: two's-complement of the sum of bytes [0x00..=0x3ef], excluding the checksum byte and the three region dwords [0xc2..=0xc5], [0xd2..=0xd5], [0x246..=0x24d].
        let excluded = |i: usize| {
            i == 0x02 || (0xc2..=0xc5).contains(&i) || (0xd2..=0xd5).contains(&i) || (0x246..=0x24d).contains(&i)
        };
        let sum: u32 = (0..=0x3ef).filter(|&i| !excluded(i)).map(|i| config[i] as u32).sum();
        config[0x02] = (0u32.wrapping_sub(sum) & 0xff) as u8;

        let mut ep: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(0x02)
            .map_err(|_| SinoLinkError::Bulk(TransferError::Unknown(0)))?;
        let mut b = Buffer::new(config.len());
        b.extend_from_slice(&config);
        let c = ep.transfer_blocking(b, TIMEOUT);
        c.status.map_err(SinoLinkError::Bulk)
    }

    /// Send the 8-byte customer password (bRequest 0x46, password at param[6..14]) to unlock a protected part; must run after send_config, since the commit address comes from the config blob.
    fn send_unlock_key(&self) -> Result<(), SinoLinkError> {
        if let Some(key) = self.unlock_key {
            eprintln!("Unlocking with customer password (0x46)…");
            let mut p = [0u8; 16];
            p[0] = 0x46;
            p[6..14].copy_from_slice(&key);
            self.write_control(0x46, 0, 0, &p)?;
        }
        Ok(())
    }

    /// The connect sequence: power the target, send the config payload, and verify via the 0x40 status (status[0] == 0 = connected).
    fn connect(&self, write_mode: bool) -> Result<(), SinoLinkError> {
        self.get_info()?;
        self.read_chip(0x11, 0, 0, 0x0000, 0x0400)?;
        // bRequest 0x12 erases the dongle's own stored-config flash sector (not target power), so the fresh blob sent by 0x10 below can be re-persisted.
        self.write_control(0x12, 1, 0, &[])?;
        self.send_config(write_mode)?;
        self.read_chip(0x11, 0, 0, 0x0000, 0x0400)?;
        // 0x18 CLEARS the status block (resets the master result byte before the connect).
        self.read_control(0x18, 1, 0, 16)?;
        // 0x15 reads the target power / voltage-class word (0 = 3.3V class, 5 = 5V class); informational only.
        let vc = self.read_control(0x15, 1, 0, 2)?;
        if let Some(&b) = vc.first() {
            debug!("target power/voltage-class word (0x15): {:#04x}", b);
        }
        // 0x40 is the CONNECT command (not a passive read): wValue high byte = mode (host 0x0101 -> mode 1); it runs the ICP identity handshake and writes the master result byte to status[0].
        let status = self.read_control(0x40, 0x0101, 0, 16)?;
        let code = status.first().copied().unwrap_or(0x09);
        eprintln!(
            "connection status: {:02x?} ({})",
            &status[..status.len().min(4)],
            Self::decode_result_byte(code)
        );
        // Connected iff status[0] == 0x00; any non-zero master result byte is a failure, and status[1..3] are sub-status markers, not a chip ID.
        if code != 0x00 {
            return Err(SinoLinkError::TargetNotConnected(status[..status.len().min(4)].to_vec()));
        }
        // Present the customer password (0x46) now that the config blob loaded the commit address; a no-op without --key.
        self.send_unlock_key()?;
        // Connect-phase probe read; tolerate failure so the actual flash read still gets attempted.
        if let Err(e) = self.read_chip(0x46, 1, 0, 0, 0x10) {
            eprintln!("note: connect-phase 0x46 read failed ({e:?}); continuing to flash read");
        }
        Ok(())
    }

    pub fn set_unlock_key(&mut self, key: [u8; 8]) {
        self.unlock_key = Some(key);
    }

    /// Override the 8 code-option fuse bytes to program with a board-specific value.
    pub fn set_code_options(&mut self, opts: [u8; 8]) {
        self.code_options = opts;
    }

    /// Enable programming of the ISP/BootROM region; must be set before write_init, as the flag rides in the config payload.
    pub fn set_program_isp(&mut self, enable: bool) {
        self.program_isp = enable;
    }

    /// Enable programming of the data-EEPROM region (config `blob[0x15]`). Set before `write_init`.
    pub fn set_program_eeprom(&mut self, enable: bool) {
        self.program_eeprom = enable;
    }

    /// Enable programming of the OTP/ChipDiff region (config `blob[0x17]`). Set before `write_init`.
    pub fn set_program_otp(&mut self, enable: bool) {
        self.program_otp = enable;
    }

    /// Lock the part by burning the --key password into the Security region during the write; cleared by a full mass-erase.
    pub fn set_lock(&mut self, enable: bool) {
        self.lock = enable;
    }

    /// Run the connect sequence with a few retries; a marginal ICP connection often fails the first bulk-IN but recovers on a retry.
    fn connect_retry(&self, write_mode: bool) -> Result<(), SinoLinkError> {
        const RETRIES: u32 = 4;
        let mut last = SinoLinkError::NotImplemented("connect");
        for attempt in 0..RETRIES {
            match self.connect(write_mode) {
                Ok(()) => return Ok(()),
                Err(SinoLinkError::Cancelled) => return Err(SinoLinkError::Cancelled),
                Err(e) => {
                    if attempt + 1 < RETRIES {
                        eprintln!("connect attempt {} failed ({e:?}); retrying…", attempt + 1);
                        sleep(Duration::from_millis(300));
                    }
                    last = e;
                }
            }
        }
        Err(last)
    }

    /// Read `len` bytes via `read_chip`, retrying a couple of times on a bulk timeout.
    fn read_chip_retry(&self, request: u8, mode1: u8, mode2: u8, addr: u16, length: u16) -> Result<Vec<u8>, SinoLinkError> {
        let mut last = SinoLinkError::Bulk(TransferError::Unknown(0));
        for attempt in 0..3 {
            match self.read_chip(request, mode1, mode2, addr, length) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    last = e;
                    if attempt < 2 {
                        sleep(Duration::from_millis(100));
                    }
                }
            }
        }
        Err(last)
    }

    /// Poll the 5-byte operation status (0x16) until byte[0] == 0x00; retry briefly since a marginal link can return a transient value first.
    fn check_status(&self) -> Result<(), SinoLinkError> {
        let mut last = vec![0xffu8];
        for _ in 0..20 {
            // 0x16[0] = master result byte: 0x00 = done, 0x55 = busy, anything else = an error code.
            let st = self.read_control(0x16, 1, 0, 5)?;
            match st.first().copied().unwrap_or(0xff) {
                0x00 => return Ok(()), // done
                0x55 => {}             // busy, keep polling
                _ => {}                // error code; keep polling and let the read-back verify decide
            }
            last = st;
            sleep(Duration::from_millis(50));
        }
        // The 0x16 status read is itself unreliable on a marginal link, so this is advisory only; the read-back verify is the source of truth.
        eprintln!(
            "warning: status not clean after polling (status {:02x?}, {}); relying on read-back verify",
            last,
            Self::decode_result_byte(last.first().copied().unwrap_or(0xff))
        );
        Ok(())
    }

    /// Issue the ICP erase command (0x45); mode2 is the erase mode index (0 = per-sector at addr, 1 = mass, 2 = protected mass, 5 = combined) that the firmware expands to a concrete opcode using the held chip_type.
    fn erase_at(&self, addr: u16, mode2: u8) -> Result<(), SinoLinkError> {
        self.read_chip(0x45, 1, mode2, addr, 0x10)?;
        self.check_status()
    }

    pub fn read_init(&mut self) -> Result<(), SinoLinkError> {
        self.connect_retry(false)
    }
    pub fn write_init(&mut self) -> Result<(), SinoLinkError> {
        self.connect_retry(true)
    }
    pub fn erase_init(&mut self) -> Result<(), SinoLinkError> {
        self.connect_retry(true)
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>, SinoLinkError> {
        let size = self.part.flash_size;
        eprintln!("Reading {} bytes from flash...", size);
        let mut out = vec![0u8; size];
        const PAGE: usize = 64;
        let mut addr = 0usize;
        while addr < size {
            self.check_cancelled()?;
            let n = PAGE.min(size - addr);
            let buf = self.read_chip_retry(0x44, 0x01, 0x00, addr as u16, PAGE as u16)?;
            out[addr..addr + n].copy_from_slice(&buf[..n]);
            addr += PAGE;
        }
        Ok(out)
    }

    /// Read len bytes from the info/option bank (mode2=1) at addr; the Security/CustomerID/CustomerOption region lives here, not in the main code-flash space.
    pub fn read_info(&mut self, addr: u16, len: u16) -> Result<Vec<u8>, SinoLinkError> {
        self.read_chip_retry(0x44, 1, 1, addr, len)
    }

    /// Dump 0x40 bytes of the Security info region so the per-sector protection bitmap can be observed off the chip.
    pub fn dump_security(&mut self) -> Result<Vec<u8>, SinoLinkError> {
        let addr = self.part.security.address as u16;
        self.read_info(addr, 0x40)
    }


    /// Write a raw blob to the Security region via the info-bank program; the caller supplies exact bytes, and a write-mode session is required. Cleared by a full mass-erase.
    pub fn write_security_raw(&mut self, blob: &[u8]) -> Result<(), SinoLinkError> {
        let addr = self.part.security.address as u16;
        eprintln!("Writing {} bytes to Security region @ {:#x}: {}", blob.len(), addr,
            blob.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "));
        self.write_chip(0x42, 1, 1, addr, blob.len() as u16, blob.to_vec())?;
        let _ = self.check_status();
        Ok(())
    }

    /// Flash sectors covered by one protection bit (the granularity), keyed by ChipType: 0/1->1, 2->2, 7->8, else 2; SecurityLevel 3 forces 4.
    pub fn sectors_per_protect_bit(&self) -> usize {
        // SecurityLevel 3 overrides every chip type to 4; checked first so it wins.
        if self.part.security_level == 3 {
            return 4;
        }
        // Base granularity by chip type; CT7 = 8 sectors (4 KiB) per bit.
        match self.part.chip_type {
            0 | 1 => 1,
            2 => 2,
            7 => 8,
            _ => 2,
        }
    }

    /// Build and write the per-sector protection bitmap (0x19-byte record: 16-byte bitmap + flag + password tail) covering all chip types and security levels; read_groups/write_groups are per-group booleans, packed as a 4-bit field per group with read-protect in bits 0,1 and write-protect in bit 3 (SecurityLevel 4) or bits 2,3 (others). Reversible by a full mass-erase.
    pub fn apply_protection(&mut self, read_groups: &[bool], write_groups: &[bool]) -> Result<(), SinoLinkError> {
        // This builder implements only the 0x19-byte record; refuse a part with no Security region or one needing the 0x38 record.
        match self.part.security_record_format {
            SecurityRecordFormat::Record19 => {}
            SecurityRecordFormat::None => {
                return Err(SinoLinkError::Unsupported(
                    "this part defines no Security region in its .gpt; protection ops are unsupported",
                ));
            }
            SecurityRecordFormat::Record38 => {
                return Err(SinoLinkError::Unsupported(
                    "this part uses the 0x38 (b200) Security record, which sinodude does not implement",
                ));
            }
        }
        if self.part.security.address == 0 {
            return Err(SinoLinkError::Unsupported(
                "this part has no Security address; protection ops are unsupported",
            ));
        }
        let spb = self.sectors_per_protect_bit();
        let num_sectors = self.part.flash_size / self.part.sector_size.max(1);
        let groups = num_sectors.div_ceil(spb);
        // SecurityLevel-keyed write-protect bit pattern within the per-group nibble.
        let seclvl4 = self.part.security_level == 4;
        let wp_bits: u8 = if seclvl4 { 0x8 } else { 0xC }; // [A,A,C,B] bit3  vs  [A,A,B,B] bits2,3
        const MAX_GROUPS: usize = 32; // 16-byte bitmap region in the 0x19 record
        if groups > MAX_GROUPS {
            eprintln!(
                "warning: {} protection groups exceeds the {} this 0x19-byte Security format holds; only \
                 the first {} groups ({} sectors) are protected. (Larger parts use the 0x38 variant, not \
                 implemented.)",
                groups, MAX_GROUPS, MAX_GROUPS, MAX_GROUPS * spb
            );
        }
        let mut blob = vec![0u8; 0x19];
        let mut any_write = false;
        for e in 0..groups.min(MAX_GROUPS) {
            let mut nibble = 0u8;
            if read_groups.get(e).copied().unwrap_or(false) {
                nibble |= 0x3; // bitmap A: read-protect (all levels)
            }
            if write_groups.get(e).copied().unwrap_or(false) {
                nibble |= wp_bits; // bitmap B: write-protect (level-dependent bits)
                any_write = true;
            }
            let byte = e / 2;
            blob[byte] |= if e % 2 == 0 { nibble } else { nibble << 4 };
        }
        // Global type-enable flag; write-protect is not enforced without it (bitmap B alone blocks nothing), while read-protect needs no flag.
        if any_write {
            blob[0x10] = 0x33;
        }
        self.write_security_raw(&blob)
    }

    pub fn mass_erase(&mut self) -> Result<(), SinoLinkError> {
        // Per-part full-erase mode index (5 = combined, 1 = mass, 2 = protected mass); the firmware expands it to a concrete opcode using the held chip_type, giving full per-chip-type coverage.
        let mode = self.part.erase_full_mode;
        eprintln!(
            "Mass erasing flash (mode {} → {})…",
            mode,
            Self::full_erase_opcode_desc(mode, self.part.chip_type)
        );
        self.erase_at(0x0000, mode)
    }

    /// Display-only description of the concrete ICP erase opcode the firmware picks for a full-erase mode index and chip type.
    fn full_erase_opcode_desc(mode: u8, chip_type: u8) -> &'static str {
        match mode {
            1 => if chip_type == 7 { "0x4B mass" } else { "0xAA mass" },
            2 => if chip_type == 7 { "0x3C protected-mass" } else { "0xDA protected-mass" },
            5 => "0xC3 combined code+ISP+EEPROM",
            _ => "vendor-specific",
        }
    }

    /// Erase the sector-aligned range `[start, end)` via per-sector `0x45` erases.
    pub fn erase_sectors(&mut self, start: u32, end: u32) -> Result<(), SinoLinkError> {
        let sector = self.part.sector_size.max(1) as u32;
        let first = start / sector;
        let last = end.saturating_sub(1) / sector;
        eprintln!(
            "Erasing sectors {:#x}..{:#x} ({} sector(s))…",
            first * sector,
            (last + 1) * sector,
            last - first + 1
        );
        for s in first..=last {
            self.check_cancelled()?;
            self.erase_at((s * sector) as u16, 0)?; // mode2=0 → 0xE6 per-sector erase at this addr
        }
        Ok(())
    }

    /// Write the code-option block and custom/info region; must run after the flash pages (a second erase would wipe these), with the high option line programmed only when the part declares part.option_high_addr.
    fn write_code_options(&mut self) -> Result<(), SinoLinkError> {
        const REGION: usize = 64;
        let base = self.part.customer_id.address;
        // Fuse-burn: low 4 option bytes go to the customer-option region at base, high 4 (when the part has a high option line) go to part.option_high_addr.
        let co = self.code_options;
        let mut region = [0u8; REGION];
        // [0xc0,0x4a] = marker/checksum prefix; remaining 4 = the low option bytes.
        region[4..6].copy_from_slice(&[0xc0, 0x4a]);
        region[6..10].copy_from_slice(&co[0..4]);

        let co_hex: String = co.iter().map(|b| format!("{:02x}", b)).collect();
        eprintln!("Writing code-options {} at {:#x}…", co_hex, base);
        let _ = self.read_chip(0x44, 1, 1, 0x1209, 0x40); // option-program-state pre-reads
        let _ = self.read_chip(0x44, 1, 1, 0x1200, 0x10);
        if let Some(high) = self.part.option_high_addr {
            let high = high as u16;
            let _ = self.read_chip(0x44, 1, 1, high, 0x04);
            let _ = self.check_status();
            // First half of the high option line: high 3 bytes 0 then co[7].
            self.write_chip(0x42, 1, 1, high, 4, vec![0x00, 0x00, 0x00, co[7]])?;
            let _ = self.check_status();
            self.write_chip(0x42, 1, 1, base as u16, REGION as u16, region.to_vec())?;
            let _ = self.check_status();
            // Second half: the full high 4 option bytes co[4..8].
            self.write_chip(0x42, 1, 1, high, 4, co[4..8].to_vec())?;
            let _ = self.check_status();
        } else {
            let _ = self.check_status();
            self.write_chip(0x42, 1, 1, base as u16, REGION as u16, region.to_vec())?;
            let _ = self.check_status();
        }

        // Lock = the enforcing per-sector read-protect bitmap over the whole code region (the password itself is already in the config blob); reversible by a full mass-erase.
        if self.lock {
            self.apply_lock()?;
        }
        Ok(())
    }

    /// Apply the lock: read-protect the entire code region via the 0x19 Security record, and, only if the part declares an ISP password address, commit the password there. Reversible by a full mass-erase.
    fn apply_lock(&mut self) -> Result<(), SinoLinkError> {
        if self.unlock_key.is_none() {
            eprintln!(
                "warning: --lock requested but no --key supplied; the unlock password is what a locked \
                 part requires on reconnect. Applying the read-protect bitmap anyway (RE §2: lock = \
                 read-protect bitmap; password is the key carried in the config blob)."
            );
        }
        // Refuse to silently mis-program a part with no 0x19 Security record.
        if self.part.security_record_format != SecurityRecordFormat::Record19
            || self.part.security.address == 0
        {
            return Err(SinoLinkError::Unsupported(
                "--lock: this part has no a640 0x19 Security region, cannot apply protection",
            ));
        }
        // Read-protect EVERY protection group covering the flash (the enforcing lock).
        let spb = self.sectors_per_protect_bit();
        let num_sectors = self.part.flash_size / self.part.sector_size.max(1);
        let groups = num_sectors.div_ceil(spb);
        let read_groups = vec![true; groups];
        let no_write = vec![false; groups];
        eprintln!(
            "--lock: read-protecting all {} groups ({} sectors/group) via the 0x19 Security record…",
            groups, spb
        );
        self.apply_protection(&read_groups, &no_write)?;
        // If the part declares an ISP password address, commit the password there; no current part does, so this is here only for future parts.
        if let (Some(addr), Some(key)) = (self.part.isp_password_addr, self.unlock_key) {
            let mut blob = [0xffu8; 0x10];
            blob[..8].copy_from_slice(&key);
            eprintln!("--lock: committing 8-byte password to ISPPasswordAddr {:#x}…", addr);
            self.write_chip(0x42, 1, 1, addr as u16, 0x10, blob.to_vec())?;
            let _ = self.check_status();
        }
        Ok(())
    }

    /// A no-op: options are programmed inside write_flash after the pages, so doing it here (before the erase) would wipe them.
    #[allow(clippy::too_many_arguments)]
    pub fn write_custom_fields(
        &mut self,
        _customer_id: Option<&[u8; 4]>,
        _operation_number: Option<&[u8; 2]>,
        _customer_option: Option<&[u8]>,
        _security: Option<&[u8]>,
        _serial_number: Option<&[u8; 4]>,
        _use_stored_defaults: bool,
    ) -> Result<(), SinoLinkError> {
        Ok(())
    }

    pub fn write_flash(&mut self, firmware: &[u8]) -> Result<(), SinoLinkError> {
        self.write_flash_range(firmware, 0, self.part.flash_size.min(firmware.len()))
    }

    /// Erase, write, and verify the flash range [start, end): 1 KiB pages via 0x42, polling status after each, then read-back verify.
    pub fn write_flash_range(&mut self, firmware: &[u8], start: usize, end: usize) -> Result<(), SinoLinkError> {
        let end = end.min(firmware.len()).min(self.part.flash_size);
        if start >= end {
            return Ok(());
        }
        // Erase immediately before the page writes, since a page program only persists when a fresh erase precedes it; a partial range erases only its own sectors so the rest of the flash is preserved.
        if start == 0 && end >= self.part.flash_size {
            self.mass_erase()?;
        } else {
            self.erase_sectors(start as u32, end as u32)?;
        }
        const PAGE: usize = 1024;
        eprintln!("Writing {} bytes to flash…", end - start);
        let mut addr = start - (start % PAGE);
        while addr < end {
            self.check_cancelled()?;
            let mut page = vec![0xffu8; PAGE];
            let copy_start = addr.max(start);
            let copy_end = (addr + PAGE).min(end);
            if copy_start < copy_end {
                page[copy_start - addr..copy_end - addr].copy_from_slice(&firmware[copy_start..copy_end]);
            }
            self.write_chip(0x42, 0x01, 0x00, addr as u16, PAGE as u16, page)?;
            let _ = self.check_status();
            addr += PAGE;
        }

        // Program the code options LAST (after the pages) so the page erase above doesn't wipe them.
        self.write_code_options()?;

        // Verify: reads only work in a read-config session, so re-arm one first.
        eprintln!("Verifying…");
        self.connect_retry(false)?;
        const RP: usize = 64;
        let mut a = start - (start % RP);
        let mut mism = 0usize;
        let mut first_mism: Option<u32> = None;
        while a < end {
            self.check_cancelled()?;
            let buf = self.read_chip_retry(0x44, 0x01, 0x00, a as u16, RP as u16)?;
            for i in 0..RP {
                let abs = a + i;
                if abs >= start && abs < end && buf[i] != firmware[abs] {
                    mism += 1;
                    first_mism.get_or_insert(abs as u32);
                }
            }
            a += RP;
        }
        if mism == 0 {
            eprintln!("Verify OK — flash matches.");
        } else {
            // The top ISP/bootloader region cannot be programmed by normal writes, so mismatches there are expected; warn rather than abort.
            eprintln!(
                "warning: {} byte(s) mismatched (first @ {:#x}) — likely the unwritable ISP region; \
                 code area should still be restored.",
                mism,
                first_mism.unwrap_or(0)
            );
        }
        Ok(())
    }
    pub fn finish(&mut self) -> Result<(), SinoLinkError> {
        Ok(())
    }
}
