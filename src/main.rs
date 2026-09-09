use clap::*;
use log::info;
use simple_logger::SimpleLogger;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{env, fs, io::Read};

mod ihex;
pub mod parts;
mod programmer;

pub use crate::{ihex::*, parts::*, programmer::*};

fn parse_hex(s: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    if !s.len().is_multiple_of(2) {
        return Err("Hex string must have even length".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
        .collect()
}

fn parse_addr(s: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        usize::from_str_radix(&s[2..], 16).map_err(|e| e.into())
    } else {
        s.parse::<usize>().map_err(|e| e.into())
    }
}

fn cli() -> Command {
    Command::new("sinodude")
        .about("programming tool for sinowealth devices")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Karolis Stasaitis")
        .subcommand(
            Command::new("read")
                .short_flag('r')
                .about("Read the chips flash contents")
                .arg(arg!(output_file: <OUTPUT_FILE> "file to write flash contents to"))
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinodude-serial", "sinolink"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(--port <PORT> "Serial port for sinodude-serial programmer (e.g., /dev/ttyUSB0)")
                        .required(false),
                )
                .arg(
                    arg!(--key <KEY> "Unlock key / password for a protected part (8 bytes hex, e.g., 0102030405060708)")
                        .required(false),
                )
                .arg(
                    arg!(--ocd "Read flash over the 3-wire OCD MOVC read-protect bypass instead of the fast ICP read. Recovers read-protected flash (the chip's own CPU code-fetch is not gated); slower (~16 ms/byte). sinodude-serial only.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    arg!(--power <POWER> "Target supply for the sinolink programmer: 5v (default), 3v3, or external")
                        .value_parser(["5v", "3v3", "external"])
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("write")
                .short_flag('w')
                .about("Write to flash")
                .arg(arg!(input_file: <INPUT_FILE> "file to write to flash"))
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinodude-serial", "sinolink"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(--port <PORT> "Serial port for sinodude-serial programmer (e.g., /dev/ttyUSB0)")
                        .required(false),
                )
                .arg(
                    arg!(--customer_id <CUSTOMER_ID> "Customer ID (4 bytes hex, e.g., 01020304)")
                        .required(false),
                )
                .arg(
                    arg!(--operation_number <OPERATION_NUMBER> "Operation number (2 bytes hex, e.g., 0102)")
                        .required(false),
                )
                .arg(
                    arg!(--customer_option <CUSTOMER_OPTION> "Customer option (hex string)")
                        .required(false),
                )
                .arg(
                    arg!(--security <SECURITY> "Security bits (hex string)")
                        .required(false),
                )
                .arg(
                    arg!(--serial_number <SERIAL_NUMBER> "Serial number (4 bytes hex, e.g., 01020304)")
                        .required(false),
                )
                .arg(
                    arg!(--start_addr <START_ADDR> "Start address for partial write (hex, e.g., 0x1000)")
                        .required(false),
                )
                .arg(
                    arg!(--end_addr <END_ADDR> "End address for partial write (hex, e.g., 0x2000)")
                        .required(false),
                )
                .arg(
                    arg!(--key <KEY> "Unlock key / password for a protected part (8 bytes hex)")
                        .required(false),
                )
                .arg(
                    arg!(--code_options <CODE_OPTIONS> "Code-option fuse bytes for the sinolink programmer (8 bytes hex, e.g. A4E063C00F000088). Defaults to the part's generic default.")
                        .required(false),
                )
                .arg(
                    arg!(--program_isp "Also program the ISP/BootROM region (flash 0xEC00+); required to restore a bootloader. SinoLink only.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    arg!(--program_eeprom "Also program the data-EEPROM region. SinoLink only; no-op on parts without EEPROM.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    arg!(--program_otp "Also program the OTP/ChipDiff region. SinoLink only.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    arg!(--lock "LOCK the part: burn the --key password into the Security region (read/write then require that password, unless cleared by a full mass-erase). SinoLink only; RE-inferred, NOT validated.")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    arg!(--power <POWER> "Target supply for the sinolink programmer: 5v (default), 3v3, or external")
                        .value_parser(["5v", "3v3", "external"])
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("erase")
                .short_flag('e')
                .about("Erase the chip's flash (mass erase or specific sectors)")
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinodude-serial", "sinolink"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(--port <PORT> "Serial port for sinodude-serial programmer (e.g., /dev/ttyUSB0)")
                        .required(false),
                )
                .arg(
                    arg!(--start_addr <START_ADDR> "Start address for sector erase (hex, e.g., 0x1000)")
                        .required(false),
                )
                .arg(
                    arg!(--end_addr <END_ADDR> "End address for sector erase (hex, e.g., 0x2000)")
                        .required(false),
                )
                .arg(
                    arg!(--key <KEY> "Unlock key / password for a protected part (8 bytes hex)")
                        .required(false),
                )
                .arg(
                    arg!(--power <POWER> "Target supply for the sinolink programmer: 5v (default), 3v3, or external")
                        .value_parser(["5v", "3v3", "external"])
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("security")
                .about("Read (probe) or apply the chip's sector protection / Security region. SinoLink only.")
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinolink"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(--power <POWER> "Target supply: 5v (default), 3v3, or external")
                        .value_parser(["5v", "3v3", "external"])
                        .required(false),
                )
                .arg(
                    arg!(--dump "Read and print the current Security region (RE probe; non-destructive)")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(arg!(--addr <ADDR> "Info-bank address to dump (hex, default = Security address)").required(false))
                .arg(arg!(--len <LEN> "Bytes to dump (hex, default 0x40)").required(false))
                .arg(arg!(--"write-raw" <HEX> "RE probe: write raw bytes to the Security region (hex), then dump. Reversible by mass-erase.").required(false))
                .arg(arg!(--"read-protect" <SECTORS> "Read-protect these sectors, e.g. \"0-7,16-23\" or \"all\". Granularity = 8 sectors on sh68f90a. Reversible by mass-erase.").required(false))
                .arg(arg!(--"write-protect" <SECTORS> "Write-protect these sectors (blocks reprogramming), e.g. \"0-7\" or \"all\". Reversible by mass-erase.").required(false))
                .arg(
                    arg!(--key <KEY> "Unlock key / password for a protected part (8 bytes hex)")
                        .required(false),
                ),
        )
}

/// Parse a sector-range spec like `"0-7,16-23"`, `"3"`, or `"all"` into a per-sector boolean mask (inclusive ranges).
fn parse_sector_spec(s: &str, num_sectors: usize) -> Result<Vec<bool>, String> {
    let mut mask = vec![false; num_sectors];
    let s = s.trim();
    if s.eq_ignore_ascii_case("all") {
        return Ok(vec![true; num_sectors]);
    }
    for part in s.split(',').map(|p| p.trim()).filter(|p| !p.is_empty()) {
        let (lo, hi) = match part.split_once('-') {
            Some((a, b)) => (
                a.trim().parse::<usize>().map_err(|e| e.to_string())?,
                b.trim().parse::<usize>().map_err(|e| e.to_string())?,
            ),
            None => {
                let v = part.parse::<usize>().map_err(|e| e.to_string())?;
                (v, v)
            }
        };
        if lo > hi || hi >= num_sectors {
            return Err(format!("sector range '{part}' out of bounds (0..{})", num_sectors - 1));
        }
        for m in mask.iter_mut().take(hi + 1).skip(lo) {
            *m = true;
        }
    }
    Ok(mask)
}

/// Collapse a per-sector mask into per-group booleans: a group of `spb` sectors is protected if any of its sectors is selected.
fn sectors_to_groups(mask: &[bool], spb: usize) -> Vec<bool> {
    let groups = mask.len().div_ceil(spb.max(1));
    (0..groups)
        .map(|g| {
            let lo = g * spb;
            let hi = (lo + spb).min(mask.len());
            mask[lo..hi].iter().any(|&b| b)
        })
        .collect()
}

/// Parse an 8-byte (16 hex char) unlock key.
fn parse_unlock_key(s: &str) -> Result<[u8; 8], String> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    if s.len() != 16 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("--key must be exactly 8 bytes (16 hex chars), got '{}'", s));
    }
    let mut key = [0u8; 8];
    for (i, b) in key.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).map_err(|e| e.to_string())?;
    }
    Ok(key)
}

/// Parse the 8 code-option fuse bytes (16 hex chars, e.g. `A4E063C00F000088`).
fn parse_code_options_hex(s: &str) -> Result<[u8; 8], String> {
    let t = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    if t.len() != 16 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("--code-options must be exactly 8 bytes (16 hex chars), got '{}'", s));
    }
    let mut out = [0u8; 8];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&t[i * 2..i * 2 + 2], 16).map_err(|e| e.to_string())?;
    }
    Ok(out)
}

/// Construct the selected programmer backend behind a `Box<dyn Programmer>` and apply the optional `--key` password.
fn make_programmer(
    sub_matches: &ArgMatches,
    part: &'static Part,
    cancelled: Arc<AtomicBool>,
) -> Result<Box<dyn Programmer>, Box<dyn std::error::Error>> {
    let name = sub_matches
        .get_one::<String>("programmer")
        .map(|s| s.as_str())
        .unwrap();
    let mut programmer: Box<dyn Programmer> = match name {
        "sinodude-serial" => {
            let port = sub_matches
                .get_one::<String>("port")
                .ok_or("--port is required for the sinodude-serial programmer")?;
            let mut p = SinodudeSerialProgrammer::new(port, part, cancelled)?;
            // --ocd is only defined on the `read` subcommand; try_get_one avoids a panic on others.
            if sub_matches.try_get_one::<bool>("ocd").ok().flatten().copied().unwrap_or(false) {
                p.set_ocd_read(true);
            }
            Box::new(p)
        }
        "sinolink" => {
            let power = PowerSetting::from_arg(sub_matches.get_one::<String>("power").map(|s| s.as_str()));
            let mut p = SinoLinkProgrammer::new(part, cancelled, power)?;
            // Board-specific code-option fuses; without them the part.s generic default is burned, wrong for boards that edit the editable bits.
            if let Some(opt) = sub_matches.try_get_one::<String>("code_options").ok().flatten() {
                p.set_code_options(parse_code_options_hex(opt)?);
            }
            // Region-program options (write only; args absent on read/erase -> try_get_one).
            let flag = |id: &str| sub_matches.try_get_one::<bool>(id).ok().flatten().copied().unwrap_or(false);
            if flag("program_isp") {
                p.set_program_isp(true);
            }
            if flag("program_eeprom") {
                p.set_program_eeprom(true);
            }
            if flag("program_otp") {
                p.set_program_otp(true);
            }
            if flag("lock") {
                p.set_lock(true);
            }
            Box::new(p)
        }
        other => return Err(format!("unknown programmer '{other}'").into()),
    };
    if let Some(k) = sub_matches.get_one::<String>("key") {
        programmer.set_unlock_key(parse_unlock_key(k)?);
    }
    Ok(programmer)
}

fn run(cancelled: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read", sub_matches)) => {
            let output_file = sub_matches
                .get_one::<String>("output_file")
                .map(|s| s.as_str())
                .unwrap();

            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
            programmer.read_init()?;
            let result = programmer.read_flash()?;
            programmer.finish()?;

            let digest = md5::compute(&result);
            info!("MD5: {:x}", digest);

            let ihex = to_ihex(result)?;
            fs::write(output_file, ihex)?;
        }
        Some(("write", sub_matches)) => {
            let input_file = sub_matches
                .get_one::<String>("input_file")
                .map(|s| s.as_str())
                .unwrap();

            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            let mut file = fs::File::open(input_file)?;
            let mut file_buf = Vec::new();
            file.read_to_end(&mut file_buf)?;
            let file_str = String::from_utf8_lossy(&file_buf[..]);
            let mut firmware = from_ihex(&file_str, part.flash_size)?;

            if firmware.len() < part.flash_size {
                firmware.resize(part.flash_size, 0);
            }

            // Parse and validate address range before connecting
            let sector_size = part.sector_size;
            let start_addr = sub_matches
                .get_one::<String>("start_addr")
                .map(|s| parse_addr(s))
                .transpose()?;
            let end_addr = sub_matches
                .get_one::<String>("end_addr")
                .map(|s| parse_addr(s))
                .transpose()?;

            if let Some(addr) = start_addr {
                if addr % sector_size != 0 {
                    return Err(format!(
                        "Start address {:#x} is not aligned to sector size {:#x}",
                        addr, sector_size
                    )
                    .into());
                }
            }
            if let Some(addr) = end_addr {
                if addr % sector_size != 0 {
                    return Err(format!(
                        "End address {:#x} is not aligned to sector size {:#x}",
                        addr, sector_size
                    )
                    .into());
                }
            }

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
            programmer.write_init()?;

            // Use sector-based erase for partial writes, mass erase for full writes
            match (start_addr, end_addr) {
                (Some(start), Some(end)) => {
                    programmer.erase_sectors(start as u32, end as u32)?;
                }
                (Some(start), None) => {
                    programmer.erase_sectors(start as u32, part.flash_size as u32)?;
                }
                (None, Some(end)) => {
                    programmer.erase_sectors(0, end as u32)?;
                }
                (None, None) => {
                    programmer.mass_erase()?;
                }
            }

            // Parse custom fields
            let customer_id: Option<[u8; 4]> = sub_matches
                .get_one::<String>("customer_id")
                .map(|s| parse_hex(s))
                .transpose()?
                .map(|v| {
                    if v.len() != 4 {
                        return Err("Customer ID must be exactly 4 bytes");
                    }
                    Ok(v.as_slice().try_into().unwrap())
                })
                .transpose()?;

            let operation_number: Option<[u8; 2]> = sub_matches
                .get_one::<String>("operation_number")
                .map(|s| parse_hex(s))
                .transpose()?
                .map(|v| {
                    if v.len() != 2 {
                        return Err("Operation number must be exactly 2 bytes");
                    }
                    Ok(v.as_slice().try_into().unwrap())
                })
                .transpose()?;

            let customer_option: Option<Vec<u8>> = sub_matches
                .get_one::<String>("customer_option")
                .map(|s| parse_hex(s))
                .transpose()?;

            let security: Option<Vec<u8>> = sub_matches
                .get_one::<String>("security")
                .map(|s| parse_hex(s))
                .transpose()?;

            let serial_number: Option<[u8; 4]> = sub_matches
                .get_one::<String>("serial_number")
                .map(|s| parse_hex(s))
                .transpose()?
                .map(|v| {
                    if v.len() != 4 {
                        return Err("Serial number must be exactly 4 bytes");
                    }
                    Ok(v.as_slice().try_into().unwrap())
                })
                .transpose()?;

            // Write all custom fields in one transaction (use stored values as defaults)
            programmer.write_custom_fields(
                customer_id.as_ref(),
                operation_number.as_ref(),
                customer_option.as_deref(),
                security.as_deref(),
                serial_number.as_ref(),
                true, // use_stored_defaults
            )?;

            // Use range write for partial writes, full write otherwise
            match (start_addr, end_addr) {
                (Some(start), Some(end)) => {
                    programmer.write_flash_range(&firmware, start, end)?;
                }
                (Some(start), None) => {
                    programmer.write_flash_range(&firmware, start, firmware.len())?;
                }
                (None, Some(end)) => {
                    programmer.write_flash_range(&firmware, 0, end)?;
                }
                (None, None) => {
                    programmer.write_flash(&firmware)?;
                }
            }

            programmer.finish()?;
        }
        Some(("erase", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            // Parse and validate address range before connecting
            let sector_size = part.sector_size;
            let start_addr = sub_matches
                .get_one::<String>("start_addr")
                .map(|s| parse_addr(s))
                .transpose()?;
            let end_addr = sub_matches
                .get_one::<String>("end_addr")
                .map(|s| parse_addr(s))
                .transpose()?;

            if let Some(addr) = start_addr {
                if addr % sector_size != 0 {
                    return Err(format!(
                        "Start address {:#x} is not aligned to sector size {:#x}",
                        addr, sector_size
                    )
                    .into());
                }
            }
            if let Some(addr) = end_addr {
                if addr % sector_size != 0 {
                    return Err(format!(
                        "End address {:#x} is not aligned to sector size {:#x}",
                        addr, sector_size
                    )
                    .into());
                }
            }

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
            programmer.erase_init()?;

            // Sector-based erase for partial ranges, mass erase otherwise; the opcode is auto-selected from chip_type/JTAG-ID and the live content read in erase_init.
            match (start_addr, end_addr) {
                (Some(start), Some(end)) => {
                    programmer.erase_sectors(start as u32, end as u32)?;
                }
                (Some(start), None) => {
                    programmer.erase_sectors(start as u32, part.flash_size as u32)?;
                }
                (None, Some(end)) => {
                    programmer.erase_sectors(0, end as u32)?;
                }
                (None, None) => programmer.mass_erase()?,
            }

            programmer.finish()?;
        }
        Some(("security", sub_matches)) => {
            let part_name = sub_matches.get_one::<String>("part").map(|s| s.as_str()).unwrap();
            let part = PARTS.get(part_name).unwrap();
            let power = PowerSetting::from_arg(sub_matches.get_one::<String>("power").map(|s| s.as_str()));
            let mut p = SinoLinkProgrammer::new(part, cancelled.clone(), power)?;
            if let Some(k) = sub_matches.get_one::<String>("key") {
                p.set_unlock_key(parse_unlock_key(k)?);
            }
            let write_raw = sub_matches.get_one::<String>("write-raw").map(|s| parse_hex(s)).transpose()?;
            let rp_spec = sub_matches.get_one::<String>("read-protect");
            let wp_spec = sub_matches.get_one::<String>("write-protect");
            if let Some(blob) = write_raw {
                p.write_init()?;
                p.write_security_raw(&blob)?;
            } else if rp_spec.is_some() || wp_spec.is_some() {
                let num_sectors = part.flash_size / part.sector_size.max(1);
                let spb = p.sectors_per_protect_bit();
                let rp = rp_spec.map(|s| parse_sector_spec(s, num_sectors)).transpose()?.unwrap_or_else(|| vec![false; num_sectors]);
                let wp = wp_spec.map(|s| parse_sector_spec(s, num_sectors)).transpose()?.unwrap_or_else(|| vec![false; num_sectors]);
                let rg = sectors_to_groups(&rp, spb);
                let wg = sectors_to_groups(&wp, spb);
                eprintln!(
                    "Applying protection: {} read-protected + {} write-protected sectors ({} sectors/group). \
                     Reversible by mass-erase.",
                    rp.iter().filter(|&&b| b).count(),
                    wp.iter().filter(|&&b| b).count(),
                    spb,
                );
                p.write_init()?;
                p.apply_protection(&rg, &wg)?;
            } else {
                p.read_init()?;
            }
            {
                let addr = match sub_matches.get_one::<String>("addr") {
                    Some(s) => parse_addr(s)? as u16,
                    None => part.security.address as u16,
                };
                let len = match sub_matches.get_one::<String>("len") {
                    Some(s) => parse_addr(s)? as u16,
                    None => 0x40,
                };
                let data = p.read_info(addr, len)?;
                println!("Info-bank read @ {:#06x} ({} bytes), 0x44 mode 1/1:", addr, data.len());
                for (i, chunk) in data.chunks(16).enumerate() {
                    let hex: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
                    println!("  {:#06x}: {}", addr as usize + i * 16, hex.join(" "));
                }
            }
            p.finish()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Off)
        .env()
        .init()
        .unwrap();

    // Set up cancellation token for Ctrl+C handling
    let cancelled = Arc::new(AtomicBool::new(false));
    let c = cancelled.clone();
    ctrlc::set_handler(move || {
        c.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(e) = run(cancelled) {
        // Don't print cancellation errors - the user already knows they cancelled
        if e.to_string() != "Operation cancelled" {
            eprintln!("Error: {e}");
        }
        std::process::exit(1);
    }
}
