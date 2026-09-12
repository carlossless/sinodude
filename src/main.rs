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

fn parse_group_spec(
    spec: Option<&str>,
    groups: usize,
) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    let mut selected = vec![false; groups];
    let Some(spec) = spec else {
        return Ok(selected);
    };
    let spec = spec.trim();
    if spec.eq_ignore_ascii_case("all") {
        return Ok(vec![true; groups]);
    }
    if spec.is_empty() || spec.eq_ignore_ascii_case("none") {
        return Ok(selected);
    }
    for part in spec.split(',') {
        let part = part.trim();
        let (lo, hi) = match part.split_once('-') {
            Some((a, b)) => (a.trim().parse::<usize>()?, b.trim().parse::<usize>()?),
            None => {
                let g = part.parse::<usize>()?;
                (g, g)
            }
        };
        if lo > hi {
            return Err(format!("group range {} is inverted", part).into());
        }
        if hi >= groups {
            return Err(format!(
                "group {} is out of range; this part has {} group(s), 0-{}",
                hi,
                groups,
                groups - 1
            )
            .into());
        }
        for slot in selected.iter_mut().take(hi + 1).skip(lo) {
            *slot = true;
        }
    }
    Ok(selected)
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
                        .value_parser(["sinodude-serial"])
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
                    arg!(--key <KEY> "Customer unlock key for a password-protected part (8 bytes hex)")
                        .required(false),
                )
                .arg(
                    arg!(--ocd "Read flash over the 3-wire OCD MOVC read-protect bypass instead of the fast ICP read. Recovers read-protected flash (the chip's own CPU code-fetch is not gated); slower (~16 ms/byte).")
                        .required(false),
                )
                .arg(
                    arg!(--eeprom "Read the data EEPROM instead of code flash")
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
                        .value_parser(["sinodude-serial"])
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
                    arg!(--key <KEY> "Customer unlock key for a password-protected part (8 bytes hex)")
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
                    arg!(--eeprom "Write the data EEPROM instead of code flash (erases the whole EEPROM first)")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("erase")
                .short_flag('e')
                .about("Erase the chip's flash (mass erase or specific sectors)")
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinodude-serial"])
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
                    arg!(--key <KEY> "Customer unlock key for a password-protected part (8 bytes hex)")
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
                    arg!(--eeprom "Erase the data EEPROM (256-byte pages) instead of code flash")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(--erase_mode <MODE> "Force a vendor erase-mode index (0-5) instead of the part's own; 3 and 4 are not otherwise reachable")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("security")
                .about("Show or apply the chip's per-group read/write protection")
                .arg(
                    arg!(-c --programmer <PROGRAMMER>)
                        .value_parser(["sinodude-serial"])
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
                    arg!(--key <KEY> "Customer unlock key for a password-protected part (8 bytes hex)")
                        .required(false),
                )
                .arg(
                    arg!(--read_protect <GROUPS> "Read-protect these groups: \"all\", or a list like 1,2,5-6,13")
                        .required(false),
                )
                .arg(
                    arg!(--write_protect <GROUPS> "Write-protect these groups: \"all\", or a list like 1,2,5-6,13")
                        .required(false),
                ),
        )
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

            let port = sub_matches
                .get_one::<String>("port")
                .expect("--port is required for sinodude-serial programmer");
            let mut programmer = SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
            if sub_matches.get_flag("ocd") {
                programmer.set_ocd_read(true);
            }
            programmer.read_init()?;
            let result = if sub_matches.get_flag("eeprom") {
                programmer.read_eeprom()?
            } else {
                programmer.read_flash()?
            };
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

            let port = sub_matches
                .get_one::<String>("port")
                .expect("--port is required for sinodude-serial programmer");

            if sub_matches.get_flag("eeprom") {
                if part.eeprom_size == 0 {
                    return Err(format!("{} has no data EEPROM", part_name).into());
                }
                let mut data = from_ihex(&file_str, part.eeprom_size)?;
                data.resize(part.eeprom_size, 0);
                let mut programmer = SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                if let Some(key) = sub_matches.get_one::<String>("key") {
                    let bytes = parse_hex(key)?;
                    let key: [u8; 8] = bytes
                        .as_slice()
                        .try_into()
                        .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                    programmer.set_unlock_key(key);
                }
                programmer.write_init()?;
                programmer.write_eeprom(&data)?;
                programmer.finish()?;
                return Ok(());
            }

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

            let mut programmer = SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
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
                (None, None) => match sub_matches.get_one::<String>("erase_mode") {
                    Some(m) => programmer.mass_erase_with_mode(m.parse::<u8>()?)?,
                    None => programmer.mass_erase()?,
                },
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
        Some(("security", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();
            let part = PARTS.get(part_name).unwrap();
            let port = sub_matches
                .get_one::<String>("port")
                .expect("--port is required for sinodude-serial programmer");

            let mut programmer = SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
            programmer.read_init()?;

            let groups = part.protect_group_count();
            let group_size = part.protect_group_size();

            let read_spec = sub_matches.get_one::<String>("read_protect");
            let write_spec = sub_matches.get_one::<String>("write_protect");

            if read_spec.is_none() && write_spec.is_none() {
                let prot = programmer.read_protection()?;
                println!(
                    "Protection: {} group(s) of {:#x} bytes ({} sector(s) per bit)",
                    groups,
                    group_size,
                    part.sectors_per_protect_bit()
                );
                println!("Record: {}", hex_string(&prot.record));
                let mut any = false;
                for g in 0..groups {
                    if prot.read[g] || prot.write[g] {
                        any = true;
                        println!(
                            "  group {:<2} {:#06x}-{:#06x}  {}{}",
                            g,
                            g * group_size,
                            (g + 1) * group_size - 1,
                            if prot.read[g] { "read " } else { "" },
                            if prot.write[g] { "write" } else { "" },
                        );
                    }
                }
                if !any {
                    println!("  (unprotected)");
                }
            } else {
                let read = parse_group_spec(read_spec.map(|s| s.as_str()), groups)?;
                let write = parse_group_spec(write_spec.map(|s| s.as_str()), groups)?;
                eprintln!(
                    "Applying protection: {} read-protected, {} write-protected, of {} group(s)",
                    read.iter().filter(|&&b| b).count(),
                    write.iter().filter(|&&b| b).count(),
                    groups
                );
                eprintln!("This cannot be undone without a full mass erase.");
                programmer.apply_protection(&read, &write)?;
                eprintln!("Protection applied.");
            }

            programmer.finish()?;
        }
        Some(("erase", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            let port = sub_matches
                .get_one::<String>("port")
                .expect("--port is required for sinodude-serial programmer");

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

            let mut programmer = SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
            programmer.erase_init()?;

            if sub_matches.get_flag("eeprom") {
                programmer.erase_eeprom()?;
                programmer.finish()?;
                return Ok(());
            }

            // Use sector-based erase for partial erases, mass erase otherwise
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

            programmer.finish()?;
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
