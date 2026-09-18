//! Driver for the genuine SinoWealth SinoLink USB dongle running the 8051 ICP firmware.
//!
//! Protocol reference: `sinowealth-sinolink-rev-eng/prowriter/SINOLINK_8051_DRIVING.md`.

use super::Protection;
use crate::parts::{
    find_parts_by_part_number, hex_string, Part, Region, SecurityRecordFormat, Voltage,
    PROTECTION_PASSWORD_LEN, PROTECTION_PASSWORD_OFFSET, PROTECTION_RECORD_LEN,
};
use hex_literal::hex;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use nusb::transfer::{
    Buffer, Bulk, ControlIn, ControlOut, ControlType, In, Out, Recipient, TransferError,
};
use nusb::{list_devices, Device, Endpoint, Interface, MaybeFuture};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(5);
/// Reading straight after a write returns the dongle's stale buffer rather than the target: a full
/// dump taken immediately after an option write came back with the option bytes at offset 0.
const SETTLE: Duration = Duration::from_millis(20);
const POWER_CYCLE: Duration = Duration::from_millis(300);
/// Code option bytes 4..8 live on their own page, not next to 0..4 in the custom fields region.
const UPPER_OPTION_ADDR: u32 = 0x1100;

const SINOWEALTH_VID: u16 = 0x258a;
const SINOLINK_PIDS: [u16; 2] = [0x5063, 0x5007];
const BULK_IN: u8 = 0x81;
const BULK_OUT: u8 = 0x02;

pub mod req {
    pub const PROBE: u8 = 0x00;
    pub const DOWNLOAD_BLOB: u8 = 0x10;
    pub const LOAD_OPTION_BYTES: u8 = 0x11;
    pub const SET_READY: u8 = 0x04;
    pub const ERASE_STORED_BLOB: u8 = 0x12;
    pub const LOAD_PARAMS: u8 = 0x13;
    pub const STATUS_WORD: u8 = 0x15;
    pub const OP_STATUS: u8 = 0x16;
    pub const CLEAR_STATUS_BLOCK: u8 = 0x18;
    pub const PIN_SUPPLY: u8 = 0x3e;
    pub const SET_GO: u8 = 0x3f;
    pub const CONNECT: u8 = 0x40;
    pub const POWER_RESET: u8 = 0x41;
    pub const PROGRAM: u8 = 0x42;
    pub const READ: u8 = 0x44;
    pub const ERASE: u8 = 0x45;
    pub const UNLOCK_KEY: u8 = 0x46;
    pub const CHECKSUM: u8 = 0x49;
}

/// Byte 6 of the parameter block. Flash selects ICP opcodes 0x44 read / 0x6e program; Custom
/// selects 0x4a / 0xa5, the space that also holds the data EEPROM.
fn region_byte(region: Region) -> u8 {
    match region {
        Region::Flash => 0,
        Region::Custom => 1,
    }
}

fn erase_mode_for(mode: u8) -> EraseMode {
    match mode {
        2 => EraseMode::ProtectedMass,
        3 => EraseMode::Mode3,
        4 => EraseMode::Mode4,
        5 => EraseMode::Full,
        _ => EraseMode::Mass,
    }
}

/// Erase mode carried in byte 6 of the parameter block for bRequest 0x45.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseMode {
    /// Sector erase (ICP 0xe6), or a 256-byte EEPROM page erase (0x55) when `page` is set.
    Sector {
        page: bool,
    },
    /// Mass erase: ICP 0x4b on ChipType 7, 0xaa otherwise. Leaves the ISP region intact.
    Mass,
    /// Protected mass erase: ICP 0x3c on ChipType 7, 0xda otherwise.
    ProtectedMass,
    Mode3,
    Mode4,
    /// Code, ISP and EEPROM together (ICP 0xc3). Required before writing the boot region.
    Full,
}

impl EraseMode {
    fn bytes(self) -> (u8, u8) {
        match self {
            EraseMode::Sector { page } => (0, u8::from(page)),
            EraseMode::Mass => (1, 0),
            EraseMode::ProtectedMass => (2, 0),
            EraseMode::Mode3 => (3, 0),
            EraseMode::Mode4 => (4, 0),
            EraseMode::Full => (5, 0),
        }
    }
}

/// Argument to `bRequest 0x3e` subcommand 1. Rail A measures ~3.3 V and rail B ~5 V on the
/// dongles seen so far, which is why [`Power`] verifies against the ADC rather than trusting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Supply {
    RailA = 1,
    RailB = 2,
    Off = 0xff,
}

/// How the target is powered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Power {
    /// Dongle supplies 3.3 V.
    V3_3,
    /// Dongle supplies 5 V.
    V5_0,
    /// Target is powered by the board; the dongle's load switch stays off.
    External,
}

impl Power {
    pub fn parse(s: Option<&str>) -> Power {
        match s.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
            Some("5v") | Some("5.0") | Some("5v0") | Some("b") => Power::V5_0,
            Some("external") | Some("ext") | Some("off") | Some("target") => Power::External,
            _ => Power::V3_3,
        }
    }

    fn supply(self) -> Option<Supply> {
        match self {
            Power::V3_3 => Some(Supply::RailA),
            Power::V5_0 => Some(Supply::RailB),
            Power::External => None,
        }
    }

    /// Millivolt window the ADC reading has to land in for the selection to be believed.
    fn expected_mv(self) -> Option<(u16, u16)> {
        match self {
            Power::V3_3 => Some((2900, 3700)),
            Power::V5_0 => Some((4400, 5600)),
            Power::External => None,
        }
    }

    /// Refuse a rail the part is not rated for. Every path that can energise a target has to call
    /// this: the low-level sinolink actions happily put 5 V on a 3.3 V-only part otherwise.
    pub fn check(self, part: &Part) -> Result<()> {
        if let Some(v) = self.voltage() {
            if !part.compatible_voltages.contains(&v) {
                return Err(SinoLinkError::UnsupportedVoltage {
                    part: hex_string(&part.part_number),
                    power: self,
                    supported: part
                        .compatible_voltages
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                });
            }
        }
        Ok(())
    }

    fn voltage(self) -> Option<Voltage> {
        match self {
            Power::V3_3 => Some(Voltage::V3_3),
            Power::V5_0 => Some(Voltage::V5_0),
            Power::External => None,
        }
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Power::V3_3 => write!(f, "3.3 V"),
            Power::V5_0 => write!(f, "5 V"),
            Power::External => write!(f, "external"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SinoLinkError {
    #[error("no SinoLink dongle found (looked for USB 258a:5063 / 258a:5007)")]
    DeviceNotFound,
    #[error("USB setup failed: {0}")]
    Setup(String),
    #[error("control transfer failed: {0:?}")]
    Control(TransferError),
    #[error("bulk transfer failed: {0:?}")]
    Bulk(TransferError),
    #[error("target did not connect (status {0:02x?}: {1})")]
    NotConnected(Vec<u8>, &'static str),
    #[error("operation failed (status {0:#04x}: {1})")]
    Failed(u8, &'static str),
    #[error("program refused at {addr:#x}: the target did not echo back what was written. Common causes: the region is not erased, or it is outside programmable code flash (an ISP/EEPROM carve-out, whose size comes from factory option byte 7 and cannot be changed). Reads and erases of such a region still appear to work, and writing zeros there also appears to succeed.")]
    ProgramRefused { addr: u32 },
    #[error("operation still busy after {0:?}")]
    Timeout(Duration),
    #[error("{part} does not support {power}; it supports {supported}")]
    UnsupportedVoltage {
        part: String,
        power: Power,
        supported: String,
    },
    #[error(
        "asked for {power} but the target measures {mv} mV; check the supply selection and wiring"
    )]
    VoltageOutOfRange { power: Power, mv: u16 },
    #[error("no target power detected ({mv} mV); with --power external the board must supply the target")]
    NoExternalPower { mv: u16 },
    #[error("part number mismatch: expected {expected}, got {actual}")]
    PartNumberMismatch { expected: String, actual: String },
    #[error("unlock key rejected: the target still reads locked")]
    UnlockKeyRejected,
    #[error("unsupported for this part: {0}")]
    Unsupported(&'static str),
    #[error("protection at offset {offset} cannot be reduced: {current:#04x} is already burnt, {requested:#04x} requested; mass erase first")]
    ProtectionNotReducible {
        offset: usize,
        current: u8,
        requested: u8,
    },
    #[error("cancelled")]
    Cancelled,
}

type Result<T> = std::result::Result<T, SinoLinkError>;

/// Decode the dongle's master result byte, reported by both the 0x40 connect status and the
/// 0x16 operation status.
pub fn decode_status(code: u8) -> &'static str {
    match code {
        0x00 => "ok",
        0x09 => "ISP connect handshake failed",
        0x11 => "program mismatch",
        0x12 => "verify mismatch",
        0x14..=0x17 => "frame/byte send timeout",
        0x18 | 0x19 | 0x20 | 0x21 => "result byte read timeout",
        0x22 | 0x23 => "silicon ID read failed",
        0x24 | 0x25 => "address/value send timeout",
        0x28 => "erase frame not acknowledged",
        0x30 => "combined address+read timeout",
        0x33 | 0x34 | 0x37 => "program/verify mismatch",
        0x55 => "busy",
        0x99 => "ISP shim load failed",
        0xcc | 0xdd => "debug entry retry",
        0xee => "erase busy timeout",
        0xf3 => "single-wire bus stuck",
        0xff => "erase failed",
        _ => "unknown",
    }
}

pub struct SinoLink {
    _device: Device,
    interface: Interface,
}

impl SinoLink {
    pub fn open() -> Result<Self> {
        let info = list_devices()
            .wait()
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?
            .find(|d| d.vendor_id() == SINOWEALTH_VID && SINOLINK_PIDS.contains(&d.product_id()))
            .ok_or(SinoLinkError::DeviceNotFound)?;

        eprintln!(
            "SinoLink: {} {} ({:04x}:{:04x})",
            info.manufacturer_string().unwrap_or("?"),
            info.product_string().unwrap_or("?"),
            info.vendor_id(),
            info.product_id()
        );

        let device = info
            .open()
            .wait()
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?;
        if !matches!(device.active_configuration(), Ok(c) if c.configuration_value() == 1) {
            device
                .set_configuration(1)
                .wait()
                .map_err(|e| SinoLinkError::Setup(e.to_string()))?;
        }
        let interface = device
            .detach_and_claim_interface(0)
            .wait()
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?;

        Ok(Self {
            _device: device,
            interface,
        })
    }

    // --- transport ---------------------------------------------------------

    fn ctrl_in(&self, request: u8, value: u16, index: u16, length: u16) -> Result<Vec<u8>> {
        debug!("ctrl-IN  {request:#04x} val={value:#06x} idx={index:#06x} len={length}");
        let data = self
            .interface
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
            .map_err(SinoLinkError::Control)?;
        debug!("ctrl-IN  <= ({}) {:02x?}", data.len(), data);
        Ok(data)
    }

    fn ctrl_out(&self, request: u8, value: u16, index: u16, data: &[u8]) -> Result<()> {
        debug!("ctrl-OUT {request:#04x} val={value:#06x} idx={index:#06x} data={data:02x?}");
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
            .map_err(SinoLinkError::Control)
    }

    /// The 16-byte parameter block that precedes a bulk transfer. Address is little-endian at
    /// `[2..6]`, region or erase mode at `[6]`, sector flag at `[7]`, length at `[14..16]`.
    fn param_block(mode1: u8, addr: u32, mode: u8, flag: u8, length: u16) -> [u8; 16] {
        let a = addr.to_le_bytes();
        let l = length.to_le_bytes();
        [
            0, mode1, a[0], a[1], a[2], a[3], mode, flag, 0, 0, 0, 0, 0, 0, l[0], l[1],
        ]
    }

    fn cmd(&self, request: u8, addr: u32, mode: u8, flag: u8, length: u16) -> Result<()> {
        self.cmd_m(request, 1, addr, mode, flag, length)
    }

    /// mode1 rides in byte 1 of the parameter block. ProWriter sends 1 for program, read and the
    /// option writes; the firmware's data-out latch never reads the byte, but the target does.
    fn cmd_m(
        &self,
        request: u8,
        mode1: u8,
        addr: u32,
        mode: u8,
        flag: u8,
        length: u16,
    ) -> Result<()> {
        let m1 = std::env::var("SINOLINK_MODE1")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(mode1);
        self.ctrl_out(
            request,
            0,
            0,
            &Self::param_block(m1, addr, mode, flag, length),
        )
    }

    fn bulk_in(&self, length: usize) -> Result<Vec<u8>> {
        let mut ep: Endpoint<Bulk, In> = self
            .interface
            .endpoint(BULK_IN)
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?;
        let rounded = length.next_multiple_of(64);
        let c = ep.transfer_blocking(Buffer::new(rounded), TIMEOUT);
        c.status.map_err(SinoLinkError::Bulk)?;
        Ok(c.buffer[..length.min(c.buffer.len())].to_vec())
    }

    fn bulk_out(&self, data: &[u8]) -> Result<()> {
        let mut ep: Endpoint<Bulk, Out> = self
            .interface
            .endpoint(BULK_OUT)
            .map_err(|e| SinoLinkError::Setup(e.to_string()))?;
        let padded = data.len().next_multiple_of(64);
        let mut b = Buffer::new(padded);
        b.extend_from_slice(data);
        b.extend_from_slice(&vec![0xff; padded - data.len()]);
        let c = ep.transfer_blocking(b, TIMEOUT);
        c.status.map_err(SinoLinkError::Bulk)
    }

    // --- dongle-only commands, safe without a target ------------------------

    /// Escape hatch for poking a vendor request the driver has no wrapper for.
    pub fn raw_in(&self, request: u8, value: u16, index: u16, length: u16) -> Result<Vec<u8>> {
        self.ctrl_in(request, value, index, length)
    }

    /// 64-byte probe and identity block.
    pub fn probe(&self) -> Result<Vec<u8>> {
        self.ctrl_in(req::PROBE, 0, 0, 64)
    }

    /// Supply class and connect mode: 0 = 3.3 V class, 5 = 5 V class, 1/2 after a connect.
    pub fn status_word(&self) -> Result<u8> {
        Ok(self.ctrl_in(req::STATUS_WORD, 0, 0, 1)?[0])
    }

    /// Master result byte plus the 32-bit sub-result.
    pub fn op_status(&self) -> Result<(u8, u32)> {
        let r = self.ctrl_in(req::OP_STATUS, 0, 0, 5)?;
        let sub = u32::from_le_bytes([r[1], r[2], r[3], r[4]]);
        Ok((r[0], sub))
    }

    pub fn clear_status(&self) -> Result<()> {
        self.ctrl_in(req::CLEAR_STATUS_BLOCK, 0, 0, 4).map(|_| ())
    }

    /// `bRequest 0x3e`. The firmware byte-swaps wValue, so the subcommand goes in the low byte
    /// and its argument in the high byte.
    pub fn pin_supply(&self, sub: u8, arg: u8) -> Result<Vec<u8>> {
        self.ctrl_in(
            req::PIN_SUPPLY,
            u16::from(sub) | (u16::from(arg) << 8),
            0,
            64,
        )
    }

    /// Switch the target supply and return the measured DUT_VDD in millivolts.
    pub fn set_supply(&self, supply: Supply) -> Result<u16> {
        let r = self.pin_supply(1, supply as u8)?;
        Ok(u16::from_le_bytes([r[4], r[5]]))
    }

    /// Sample PB7 with an optional pull: 1 = pull-up, 2 = pull-down, 0 = none.
    pub fn sample_data_pin(&self, pull: u8) -> Result<u8> {
        Ok(self.pin_supply(5, pull)?[7])
    }

    // --- session -----------------------------------------------------------

    /// Erase the dongle's stored config sector, then download `blob` and latch it.
    pub fn download_blob(&self, blob: &[u8; 1024]) -> Result<()> {
        self.ctrl_out(req::ERASE_STORED_BLOB, 1, 0, &[])?;
        self.cmd(req::DOWNLOAD_BLOB, 0, 0, 0, blob.len() as u16)?;
        self.bulk_out(blob)?;
        self.load_params()
    }

    /// Stage option bytes out of the stored config blob and drive them into the target. addr is
    /// an offset into the blob, so the 8 code options sit at 0x30.
    pub fn load_option_bytes(&self, blob_offset: u32, len: u16, mode: u8) -> Result<()> {
        self.cmd(req::LOAD_OPTION_BYTES, blob_offset, mode, 0, len)?;
        self.wait_idle(Duration::from_secs(5))
    }

    /// Set the ready flag the firmware main loop waits on before its deferred programming step.
    pub fn set_ready(&self) -> Result<()> {
        self.ctrl_in(req::SET_READY, 0, 0, 1).map(|_| ())
    }

    /// Set the second go flag.
    pub fn set_go(&self) -> Result<()> {
        self.ctrl_in(req::SET_GO, 0, 0, 1).map(|_| ())
    }

    /// Re-read the stored blob into the runtime globals.
    pub fn load_params(&self) -> Result<()> {
        self.ctrl_in(req::LOAD_PARAMS, 0, 0, 1).map(|_| ())
    }

    /// `bRequest 0x41`. Subcommand rides in wValue's high byte: 0 power down, 1 ISP entry,
    /// 2 finish readback, 3 re-arm, 4 reset and enter ISP.
    pub fn power_reset(&self, sub: u8) -> Result<Vec<u8>> {
        self.ctrl_in(req::POWER_RESET, u16::from(sub) << 8, 0, 2)
    }

    /// `bRequest 0x40`. Mode rides in wValue's high byte: 1 = ICP, 2 = OCD, 4 = finalize only.
    pub fn connect(&self, mode: u8) -> Result<Vec<u8>> {
        self.clear_status()?;
        let status = self.ctrl_in(req::CONNECT, u16::from(mode) << 8, 0, 16)?;
        let code = status.first().copied().unwrap_or(0x09);
        if code != 0 {
            return Err(SinoLinkError::NotConnected(
                status[..status.len().min(8)].to_vec(),
                decode_status(code),
            ));
        }
        Ok(status)
    }

    // --- target operations --------------------------------------------------

    pub fn read(&self, addr: u32, region: Region, length: u16) -> Result<Vec<u8>> {
        self.cmd(req::READ, addr, region_byte(region), 0, length)?;
        self.bulk_in(length as usize)
    }

    pub fn program(&self, addr: u32, region: Region, data: &[u8]) -> Result<()> {
        self.cmd(
            req::PROGRAM,
            addr,
            region_byte(region),
            0,
            data.len() as u16,
        )?;
        self.bulk_out(data)?;
        self.wait_idle(Duration::from_secs(5))?;
        std::thread::sleep(SETTLE);
        Ok(())
    }

    pub fn erase(&self, addr: u32, mode: EraseMode) -> Result<()> {
        let (m, flag) = mode.bytes();
        self.cmd(req::ERASE, addr, m, flag, 0)?;
        self.wait_idle(Duration::from_secs(60))
    }

    /// Present the 8-byte customer password and read the result back.
    ///
    /// The host sends this as a bulk-IN packet, not a bare control-OUT: param[0] is the 0x80 IN
    /// marker, the length field is 0x10, and the 16-byte bulk-IN that follows is what clocks the
    /// verify read out of the target. Without it the dongle loads the password but never completes
    /// the apply, so the part stays locked.
    pub fn unlock(&self, key: &[u8; 8]) -> Result<Vec<u8>> {
        let mut p = [0u8; 16];
        p[0] = 0x80;
        p[1] = 1;
        p[6..14].copy_from_slice(key);
        p[14] = 0x10;
        p[15] = 0x00;
        self.ctrl_out(req::UNLOCK_KEY, 0, 0, &p)?;
        self.bulk_in(16)
    }

    /// Poll `bRequest 0x16` until the master result byte leaves `0x55` (busy).
    pub fn wait_idle(&self, limit: Duration) -> Result<()> {
        let start = std::time::Instant::now();
        loop {
            let (code, sub) = self.op_status()?;
            if code == 0x00 {
                return Ok(());
            }
            if code != 0x55 {
                debug!("op status {code:#04x} sub={sub:#010x}");
                return Err(SinoLinkError::Failed(code, decode_status(code)));
            }
            if start.elapsed() > limit {
                return Err(SinoLinkError::Timeout(limit));
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

/// The config blob ProWriter sends for an SH68F90A, captured from the wire: the read-session
/// variant and the write-session variant. Synthesising a blob from scratch for an arbitrary part
/// is not solved (see `8051_TRANSPORT_GAP.md` in the reverse-engineering repo), so a known-good
/// capture is the base and only the fields whose derivation is established get patched.
const BLOB_WRITE: [u8; 1024] = hex!(
    "78877a0700010301040000050000030106f20000000000000008000000000000"
    "00000000000000000000000000000008a4e063c00f0000880000000000000000"
    "0000010040ff0000c04a64000000000000000000000001008658000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0002000080000000000000000000000000000000000000000000000000000000"
    "081c1106080f09000aff00000000000009120000050068f90a00000000000000"
    "0400000000000000000000000000000004000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000100000001200000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000024011514321605500000000000000000"
);

const BLOB_READ: [u8; 1024] = hex!(
    "7887bd0700020402040000050000030106200000000000000008000000000000"
    "00000000000000000000000000000008a4e063c00f0000880000000000000000"
    "0000010040ff0000fd8f3600000000000000000000000100b363000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0002000080000000000000000000000000000000000000000000000000000000"
    "081c1106080f09000aff00000000000009120000050068f90a00000000000000"
    "0400000000000000000000000000000004000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000100000001200000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000000000000000000000000000000000000"
    "0000000000000000000000000000000023030820360705500000000000000000"
);

/// True when `build_blob` can produce a blob this driver trusts. Everything else gets the
/// SH68F90A capture with a handful of fields patched, which is wrong in ways that show up as
/// uniform garbage on read.
pub fn blob_is_verified(part: &Part) -> bool {
    part.part_number == hex!("68f90a0000")
}

/// Build the 1024-byte config blob the dongle latches ChipType, the link class, geometry and the
/// region addresses from.
pub fn build_blob(part: &Part, power: Power, write_mode: bool) -> [u8; 1024] {
    let mut b = if write_mode { BLOB_WRITE } else { BLOB_READ };
    // Escape hatch for telling a driver bug apart from a target problem: send the capture exactly
    // as ProWriter did, with none of the patching below.
    if std::env::var("SINOLINK_RAW_BLOB").is_ok() {
        return b;
    }
    b[0x03] = part.chip_type;
    b[0x04] = 0x00;
    // Left at the captured VDDVoltageType. The rail is chosen by bRequest 0x3e subcommand 1, and
    // the encoding of this field is not established, so overwriting it buys nothing.
    let _ = power;
    b[0x0e] = part.custom_block;
    b[0x2f] = part.option_byte_count as u8;
    // Deliberately NOT patched from part.default_code_options: the whole blob is an SH68F90A
    // capture, and mixing the part table's defaults into it put a real part into a state where the
    // top of flash stopped accepting writes. Options are driven separately by load_option_bytes.
    b[0xb6..0xbb].copy_from_slice(&part.part_number);
    b[0x204..0x208].copy_from_slice(&part.customer_id.address.to_le_bytes());
    b[0x208..0x20c].copy_from_slice(&part.customer_option.address.to_le_bytes());
    apply_blob_patch(&mut b);
    set_blob_checksum(&mut b);
    b
}

/// SINOLINK_BLOB_PATCH="0x14=1,0x12=0f" overrides blob bytes before the checksum is recomputed,
/// so a suspect field can be swept against real hardware without a rebuild.
fn apply_blob_patch(b: &mut [u8; 1024]) {
    let Ok(spec) = std::env::var("SINOLINK_BLOB_PATCH") else {
        return;
    };
    for item in spec.split(',').filter(|s| !s.trim().is_empty()) {
        let Some((k, v)) = item.split_once('=') else {
            continue;
        };
        let parse = |t: &str| {
            let t = t.trim();
            let t = t.strip_prefix("0x").unwrap_or(t);
            usize::from_str_radix(t, 16).ok()
        };
        if let (Some(off), Some(val)) = (parse(k), parse(v)) {
            if off < b.len() {
                b[off] = val as u8;
            }
        }
    }
}

/// Two's complement of the sum of `[0x00..=0x3ef]`, skipping the checksum byte itself and the
/// three region dwords the firmware excludes.
pub fn set_blob_checksum(b: &mut [u8; 1024]) {
    let excluded = |i: usize| {
        i == 0x02
            || (0xc2..=0xc5).contains(&i)
            || (0xd2..=0xd5).contains(&i)
            || (0x246..=0x24d).contains(&i)
    };
    let sum: u32 = (0..=0x3ef)
        .filter(|&i| !excluded(i))
        .map(|i| u32::from(b[i]))
        .sum();
    b[0x02] = sum.wrapping_neg() as u8;
}

// ---------------------------------------------------------------------------
// CLI-facing backend
// ---------------------------------------------------------------------------

const EEPROM_PAGE_SIZE: usize = 256;
const CUSTOM_FIELDS_REGION: usize = 64;
const CHUNK: usize = 0x400;

/// The [`crate::programmer::Programmer`] backend for the SinoLink dongle.
pub struct SinoLinkProgrammer {
    link: SinoLink,
    part: &'static Part,
    cancelled: Arc<AtomicBool>,
    power: Power,
    connect_mode: u8,
    unlock_key: Option<[u8; 8]>,
    stored_custom: Option<Vec<u8>>,
    powered_down: bool,
    non_default_option_bits: bool,
}

impl SinoLinkProgrammer {
    pub fn new(part: &'static Part, cancelled: Arc<AtomicBool>, power: Power) -> Result<Self> {
        power.check(part)?;
        let link = SinoLink::open()?;
        if !blob_is_verified(part) {
            eprintln!(
                "warning: no verified config blob for {}; falling back to the SH68F90A capture with fields patched. Geometry, region addresses and receive calibration are likely wrong, which usually shows up as a uniform byte on read.",
                hex_string(&part.part_number)
            );
        }
        Ok(Self {
            link,
            part,
            cancelled,
            power,
            connect_mode: 1,
            unlock_key: None,
            stored_custom: None,
            powered_down: false,
            non_default_option_bits: false,
        })
    }

    pub fn set_unlock_key(&mut self, key: [u8; 8]) {
        self.unlock_key = Some(key);
    }

    fn check_cancelled(&self) -> Result<()> {
        if self.cancelled.load(Ordering::SeqCst) {
            Err(SinoLinkError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Power the target, load the part's config blob and connect. The blob differs between a read
    /// session and a write session, and a read taken while the dongle is still set up for writing
    /// returns stale data, so every operation re-runs this.
    ///
    /// An erase also leaves the ICP session unusable: the next program in the same session fails
    /// with result 0xcc. Erases therefore reconnect when they finish, which is why this is
    /// separate from the stored-field snapshot in session().
    /// Apply the requested power selection and check the ADC agrees. Rail A and rail B are just
    /// load-switch positions, so the only way to know which voltage a given dongle puts out is to
    /// measure it.
    fn apply_power(&self) -> Result<u16> {
        let mv = match self.power.supply() {
            Some(sup) => self.link.set_supply(sup)?,
            None => self.link.pin_supply(2, 0).map(|_| 0).unwrap_or(0),
        };
        match self.power.expected_mv() {
            Some((lo, hi)) if mv < lo || mv > hi => {
                return Err(SinoLinkError::VoltageOutOfRange {
                    power: self.power,
                    mv,
                })
            }
            _ => {}
        }
        Ok(mv)
    }

    fn connect_only(&mut self, write_mode: bool, quiet: bool) -> Result<()> {
        // Always start from a clean power-on reset. A previous run leaves the rail switched off,
        // and re-applying it without this drops the ICP handshake with status 0x09; the target
        // also refuses further password attempts until it sees a reset.
        let _ = self.link.power_reset(0);
        std::thread::sleep(POWER_CYCLE);
        let mv = self.apply_power()?;
        std::thread::sleep(POWER_CYCLE);
        self.powered_down = false;
        let blob = build_blob(self.part, self.power, write_mode);
        self.link.download_blob(&blob)?;
        let status = self.link.connect(self.connect_mode)?;
        std::thread::sleep(SETTLE);
        if !quiet {
            eprintln!("Target power: {} ({} mV)", self.power, mv);
            eprintln!("Connected (status {:02x?})", &status[..status.len().min(4)]);
        }
        if let Some(key) = self.unlock_key {
            let r = self.link.unlock(&key)?;
            eprintln!("Unlock key sent, reply {:02x?}", &r[..r.len().min(4)]);
        }
        Ok(())
    }

    fn session(&mut self, write_mode: bool) -> Result<()> {
        self.connect_only(write_mode, false)?;
        let base = self.part.customer_id.address;
        self.stored_custom = self
            .link
            .read(base, Region::Custom, CUSTOM_FIELDS_REGION as u16)
            .ok();
        // A still-locked part reads 0xff everywhere; erased custom flash reads 0x00 here. Only
        // worth checking on the session the caller opened, since the reconnect inside mass_erase
        // runs before the freshly written option page has settled and reads back stale.
        if self.unlock_key.is_some()
            && self
                .stored_custom
                .as_deref()
                .is_some_and(|s| s.iter().all(|&b| b == 0xff))
        {
            return Err(SinoLinkError::UnlockKeyRejected);
        }
        self.verify_part_number()?;
        self.non_default_option_bits = self.upper_options_differ();
        Ok(())
    }

    /// Read the part number the die reports and refuse a mismatch, so a wrong `--part` cannot
    /// program a chip with another part's geometry.
    fn verify_part_number(&mut self) -> Result<()> {
        let block = match self.part.custom_block {
            0x02 => 0x0a00,
            0x03 => 0x1200,
            0x04 => 0x2200,
            _ => return Ok(()),
        };
        let Ok(data) = self.link.read(block, Region::Custom, 16) else {
            return Ok(());
        };
        let Some(actual) = data.get(9..14) else {
            return Ok(());
        };
        // A locked or unpowered die reads uniformly; that is not a mismatch worth reporting.
        if actual.iter().all(|&b| b == 0xff) || actual.iter().all(|&b| b == 0x00) {
            return Ok(());
        }
        eprintln!("Target part number: {}", hex_string(actual));
        if actual != self.part.part_number {
            let matching = find_parts_by_part_number(actual.try_into().unwrap());
            if !matching.is_empty() {
                eprintln!(
                    "Parts matching {}: {}",
                    hex_string(actual),
                    matching.join(", ")
                );
            }
            return Err(SinoLinkError::PartNumberMismatch {
                expected: hex_string(&self.part.part_number),
                actual: hex_string(actual),
            });
        }
        Ok(())
    }

    /// Non-editable bits in the upper option page that no longer read as the factory default.
    /// A part in that state needs the code-plus-ISP-plus-EEPROM erase before a plain mass erase.
    fn upper_options_differ(&self) -> bool {
        let Some(expected) = self.part.upper_code_option_defaults() else {
            return false;
        };
        let Ok(current) = self
            .link
            .read(UPPER_OPTION_ADDR, Region::Custom, expected.len() as u16)
        else {
            return false;
        };
        expected.iter().enumerate().any(|(i, &want)| {
            let mask = self.part.code_option_mask.get(4 + i).copied().unwrap_or(0);
            current.get(i).is_some_and(|&c| c & !mask != want)
        })
    }

    pub fn read_init(&mut self) -> Result<()> {
        self.session(false)
    }

    pub fn write_init(&mut self) -> Result<()> {
        self.session(true)
    }

    pub fn erase_init(&mut self) -> Result<()> {
        self.session(true)
    }

    fn progress(&self, len: u64, what: &str) -> ProgressBar {
        let bar = ProgressBar::new(len);
        bar.set_style(
            ProgressStyle::with_template(&format!(
                "{what} {{bar:40}} {{bytes}}/{{total_bytes}} ({{eta}})"
            ))
            .unwrap(),
        );
        bar
    }

    pub fn read_region(
        &self,
        region: Region,
        addr: u32,
        len: usize,
        what: &str,
    ) -> Result<Vec<u8>> {
        let bar = self.progress(len as u64, what);
        let mut out = Vec::with_capacity(len);
        while out.len() < len {
            self.check_cancelled()?;
            let n = (len - out.len()).min(CHUNK);
            out.extend_from_slice(&self.link.read(addr + out.len() as u32, region, n as u16)?);
            bar.set_position(out.len() as u64);
        }
        bar.finish_and_clear();
        Ok(out)
    }

    pub fn write_region(&self, region: Region, addr: u32, data: &[u8], what: &str) -> Result<()> {
        let bar = self.progress(data.len() as u64, what);
        for (i, chunk) in data.chunks(CHUNK).enumerate() {
            self.check_cancelled()?;
            let at = addr + (i * CHUNK) as u32;
            match self.link.program(at, region, chunk) {
                Ok(()) => {}
                Err(SinoLinkError::Failed(0xcc, _)) => {
                    bar.finish_and_clear();
                    return Err(SinoLinkError::ProgramRefused { addr: at });
                }
                Err(e) => {
                    bar.finish_and_clear();
                    return Err(e);
                }
            }
            bar.set_position(((i * CHUNK) + chunk.len()) as u64);
        }
        bar.finish_and_clear();
        Ok(())
    }

    pub fn read_flash(&mut self) -> Result<Vec<u8>> {
        self.read_region(Region::Flash, 0, self.part.flash_size, "flash")
    }

    pub fn read_eeprom(&mut self) -> Result<Vec<u8>> {
        if self.part.eeprom_size == 0 {
            return Err(SinoLinkError::Unsupported("this part has no data EEPROM"));
        }
        self.read_region(Region::Custom, 0, self.part.eeprom_size, "eeprom")
    }

    pub fn erase_eeprom(&mut self) -> Result<()> {
        if self.part.eeprom_size == 0 {
            return Err(SinoLinkError::Unsupported("this part has no data EEPROM"));
        }
        let pages = self.part.eeprom_size.div_ceil(EEPROM_PAGE_SIZE);
        eprintln!("Erasing data EEPROM: {pages} page(s) of {EEPROM_PAGE_SIZE} bytes");
        for p in 0..pages {
            self.check_cancelled()?;
            self.link.erase(
                (p * EEPROM_PAGE_SIZE) as u32,
                EraseMode::Sector { page: true },
            )?;
        }
        self.connect_only(true, true)
    }

    pub fn write_eeprom(&mut self, data: &[u8]) -> Result<()> {
        if self.part.eeprom_size == 0 {
            return Err(SinoLinkError::Unsupported("this part has no data EEPROM"));
        }
        self.erase_eeprom()?;
        let n = self.part.eeprom_size.min(data.len());
        self.write_region(Region::Custom, 0, &data[..n], "eeprom")
    }

    /// A part whose security record is already burnt needs the protected erase first; mode 1
    /// alone comes back refused.
    fn full_erase_modes(&self) -> Vec<u8> {
        if self.non_default_option_bits && self.part.chip_type == 7 {
            return vec![5, 1];
        }
        let off = (self.part.security.address - self.part.customer_id.address) as usize;
        let protected = self
            .stored_custom
            .as_deref()
            .and_then(|s| s.get(off..(off + PROTECTION_RECORD_LEN).min(s.len())))
            .is_some_and(|s| s.iter().any(|&b| b != 0));
        if protected {
            vec![2, 1]
        } else {
            vec![1, 2]
        }
    }

    pub fn mass_erase(&mut self) -> Result<()> {
        let modes = self.full_erase_modes();
        let t = Instant::now();
        let mut last = None;
        for &mode in &modes {
            let em = erase_mode_for(mode);
            eprintln!("Mass erasing ({em:?})");
            match self.link.erase(0, em) {
                Ok(()) => {
                    last = None;
                    break;
                }
                Err(e) => last = Some(e),
            }
        }
        if let Some(e) = last {
            return Err(e);
        }
        eprintln!("Mass erase complete in {:.2?}", t.elapsed());
        self.blank_security_and_set_option_defaults()?;
        self.connect_only(true, true)
    }

    /// Mode is the byte the dongle maps to an ICP erase opcode: 1 mass, 2 protected mass, 5 code
    /// plus ISP plus EEPROM. On an SH68F90A mode 1 already clears the whole part including the
    /// boot region and the option region.
    pub fn mass_erase_with_mode(&mut self, mode: u8) -> Result<()> {
        let em = erase_mode_for(mode);
        eprintln!("Mass erasing ({em:?})");
        let t = Instant::now();
        self.link.erase(0, em)?;
        eprintln!("Mass erase complete in {:.2?}", t.elapsed());
        self.blank_security_and_set_option_defaults()?;
        self.connect_only(true, true)
    }

    /// Restore the security region and the upper code options, in the ICP session the erase left
    /// open. The target closes that window on the next reset, and reconnecting first is why the
    /// upper option page would not take a write: option byte 7 stayed 0, which reserves 8 KiB of
    /// ISP plus 4 KiB of EEPROM and caps programming at 0xd000 on a 64 KiB part. sinodude-serial
    /// does the same thing from inside mass_erase().
    fn blank_security_and_set_option_defaults(&mut self) -> Result<()> {
        let len = self.part.security_length();
        if len > 0 {
            let zeros = vec![0u8; len];
            let _ = self
                .link
                .program(self.part.security.address, Region::Custom, &zeros);
        }
        if let Some(upper) = self.part.upper_code_option_defaults() {
            eprintln!(
                "Setting upper code option defaults at 0x1100 ({} bytes)",
                upper.len()
            );
            self.link
                .program(UPPER_OPTION_ADDR, Region::Custom, &upper)?;
        }
        Ok(())
    }

    pub fn erase_sectors(&mut self, start_addr: u32, end_addr: u32) -> Result<()> {
        let size = self.part.sector_size as u32;
        let first = start_addr / size * size;
        let count = end_addr.saturating_sub(first).div_ceil(size);
        eprintln!("Erasing {count} sector(s) of {size} bytes from {first:#x}");
        for i in 0..count {
            self.check_cancelled()?;
            self.link
                .erase(first + i * size, EraseMode::Sector { page: false })?;
        }
        self.connect_only(true, true)
    }

    pub fn write_flash(&mut self, firmware: &[u8]) -> Result<()> {
        let n = self.part.flash_size.min(firmware.len());
        self.write_region(Region::Flash, 0, &firmware[..n], "flash")
    }

    pub fn write_flash_range(&mut self, firmware: &[u8], start: usize, end: usize) -> Result<()> {
        let limit = self.part.flash_size.min(firmware.len());
        let start = start.min(limit);
        let end = end.min(limit);
        if end <= start {
            return Ok(());
        }
        self.write_region(Region::Flash, start as u32, &firmware[start..end], "flash")
    }

    /// Assemble the 64-byte block at the CustomerID address from the individual fields and program
    /// it in one go, the same shape the serial backend uses.
    pub fn write_custom_fields(
        &mut self,
        customer_id: Option<&[u8; 4]>,
        operation_number: Option<&[u8; 2]>,
        customer_option: Option<&[u8]>,
        security: Option<&[u8]>,
        serial_number: Option<&[u8; 4]>,
        use_stored_defaults: bool,
    ) -> Result<()> {
        let base = self.part.customer_id.address;
        let stored = self.stored_custom.clone();
        let at = |field_addr: u32| (field_addr - base) as usize;
        let mut buf = [0u8; CUSTOM_FIELDS_REGION];

        let stored_slice = |off: usize, len: usize| -> Option<Vec<u8>> {
            if !use_stored_defaults {
                return None;
            }
            stored
                .as_ref()
                .filter(|s| s.len() >= off + len)
                .map(|s| s[off..off + len].to_vec())
        };

        for (off, len, given) in [
            (0usize, 4usize, customer_id.map(|v| v.to_vec())),
            (
                at(self.part.operation_number.address),
                2,
                operation_number.map(|v| v.to_vec()),
            ),
            (
                at(self.part.serial_number.address),
                4,
                serial_number.map(|v| v.to_vec()),
            ),
        ] {
            if let Some(v) = given.or_else(|| stored_slice(off, len)) {
                if off + len <= CUSTOM_FIELDS_REGION {
                    buf[off..off + len].copy_from_slice(&v[..len]);
                }
            }
        }

        // Only the low four option bytes live here; 4..8 are at 0x1100 and are restored by the
        // erase. Copying eight would run into the security record that starts right after.
        let opt_off = at(self.part.customer_option.address);
        if let Some(v) = customer_option
            .map(|v| v.to_vec())
            .or_else(|| stored_slice(opt_off, 4))
        {
            let n = v.len().min(4).min(CUSTOM_FIELDS_REGION - opt_off);
            buf[opt_off..opt_off + n].copy_from_slice(&v[..n]);
        }

        if let Some(v) = security {
            let off = at(self.part.security.address);
            let len = v.len().min(CUSTOM_FIELDS_REGION - off);
            buf[off..off + len].copy_from_slice(&v[..len]);
        }

        // Writing this page erases code flash 0x0000-0x0fff as a side effect, so it has to happen
        // before the flash image goes down, not after.
        eprintln!("Writing custom fields ({CUSTOM_FIELDS_REGION} bytes) at {base:#x}");
        self.write_region(Region::Custom, base, &buf, "custom")
    }

    pub fn read_protection(&mut self) -> Result<Protection> {
        if self.part.security_record_format != SecurityRecordFormat::Record19 {
            return Err(SinoLinkError::Unsupported(
                "this part has no Record19 security record",
            ));
        }
        if self.part.protection_exceeds_record() {
            return Err(SinoLinkError::Unsupported(
                "this part needs more protection groups than a 25-byte record can hold",
            ));
        }
        let raw = self.read_region(
            Region::Custom,
            self.part.security.address,
            PROTECTION_RECORD_LEN,
            "security",
        )?;
        let (read, write) = self.part.decode_protection_record(&raw);
        Ok(Protection {
            read,
            write,
            record: raw,
        })
    }

    pub fn apply_protection(
        &mut self,
        read_protect: &[bool],
        write_protect: &[bool],
        new_password: Option<&[u8; PROTECTION_PASSWORD_LEN]>,
    ) -> Result<()> {
        if self.part.security_record_format != SecurityRecordFormat::Record19 {
            return Err(SinoLinkError::Unsupported(
                "this part has no Record19 security record",
            ));
        }
        if self.part.protection_exceeds_record() {
            return Err(SinoLinkError::Unsupported(
                "this part needs more protection groups than a 25-byte record can hold",
            ));
        }
        let existing = self
            .read_region(
                Region::Custom,
                self.part.security.address,
                PROTECTION_RECORD_LEN,
                "security",
            )
            .unwrap_or_else(|_| vec![0u8; PROTECTION_RECORD_LEN]);
        let mut password = [0u8; PROTECTION_PASSWORD_LEN];
        if existing.len() >= PROTECTION_PASSWORD_OFFSET + PROTECTION_PASSWORD_LEN {
            password.copy_from_slice(
                &existing[PROTECTION_PASSWORD_OFFSET
                    ..PROTECTION_PASSWORD_OFFSET + PROTECTION_PASSWORD_LEN],
            );
        }
        if let Some(p) = new_password {
            password = *p;
        }
        let record = self
            .part
            .build_protection_record(read_protect, write_protect, &password);

        // Erased flash reads 0x00 here and programming only sets bits, so a bit already burnt
        // cannot be cleared without a mass erase.
        for (i, (&want, &have)) in record.iter().zip(existing.iter()).enumerate() {
            if have & !want != 0 {
                return Err(SinoLinkError::ProtectionNotReducible {
                    offset: i,
                    current: have,
                    requested: want,
                });
            }
        }
        eprintln!(
            "Writing protection record ({} bytes) at {:#x}",
            PROTECTION_RECORD_LEN, self.part.security.address
        );
        self.write_region(
            Region::Custom,
            self.part.security.address,
            &record,
            "security",
        )
    }

    /// Drop the dongle's supply so the board is not left powered from it. A board on external
    /// power keeps its own supply, so leave the load switch alone.
    pub fn finish(&mut self) -> Result<()> {
        if self.power != Power::External && !self.powered_down {
            self.link.set_supply(Supply::Off)?;
            self.powered_down = true;
        }
        Ok(())
    }
}

impl Drop for SinoLinkProgrammer {
    /// Without this an error path leaves the target powered, and the part refuses further
    /// password attempts until it sees a power-on reset.
    fn drop(&mut self) {
        if self.power != Power::External && !self.powered_down {
            let _ = self.link.set_supply(Supply::Off);
            self.powered_down = true;
        }
    }
}
