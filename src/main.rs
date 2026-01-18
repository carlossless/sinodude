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
    if s.len() % 2 != 0 {
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
    return Command::new("sinodude")
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
                        .value_parser(["sinolink", "sinodude-serial"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(-t --power <POWER_SETTING> "Power setting for sinolink programmer")
                        .value_parser(["3v3", "5v", "external"])
                        .required(false),
                )
                .arg(
                    arg!(--port <PORT> "Serial port for sinodude-serial programmer (e.g., /dev/ttyUSB0)")
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
                        .value_parser(["sinolink", "sinodude-serial"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(-t --power <POWER_SETTING> "Power setting for sinolink programmer")
                        .value_parser(["3v3", "5v", "external"])
                        .required(false),
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
                    arg!(--start_addr <START_ADDR> "Start address for sector erase (hex, e.g., 0x1000)")
                        .required(false),
                )
                .arg(
                    arg!(--end_addr <END_ADDR> "End address for sector erase (hex, e.g., 0x2000)")
                        .required(false),
                ),
        )
;
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

            let programmer_name = sub_matches
                .get_one::<String>("programmer")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            let result = match programmer_name {
                "sinolink" => {
                    let power_setting_name = sub_matches
                        .get_one::<String>("power")
                        .map(|s| s.as_str())
                        .expect("--power is required for sinolink programmer");
                    let power_setting = PowerSetting::from_option(power_setting_name);
                    let sinolink = SinolinkProgrammer::new(part, power_setting)?;
                    sinolink.read_init()?;
                    sinolink.read_flash()?
                }
                "sinodude-serial" => {
                    let port = sub_matches
                        .get_one::<String>("port")
                        .expect("--port is required for sinodude-serial programmer");
                    let mut programmer =
                        SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                    programmer.read_init()?;
                    let result = programmer.read_flash()?;
                    programmer.finish()?;
                    result
                }
                _ => unreachable!(),
            };

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

            let programmer_name = sub_matches
                .get_one::<String>("programmer")
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

            match programmer_name {
                "sinolink" => {
                    let power_setting_name = sub_matches
                        .get_one::<String>("power")
                        .map(|s| s.as_str())
                        .expect("--power is required for sinolink programmer");
                    let power_setting = PowerSetting::from_option(power_setting_name);
                    let sinolink = SinolinkProgrammer::new(part, power_setting)?;
                    sinolink.write_init()?;
                    sinolink.write_flash(&firmware[0..65536])?;
                }
                "sinodude-serial" => {
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

                    let mut programmer =
                        SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                    programmer.write_init()?;

                    // Use sector-based erase for partial writes, mass erase for full writes
                    let is_mass_erase = start_addr.is_none() && end_addr.is_none();
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

                    // Blank out security region and set higher half of code options after mass erase
                    if is_mass_erase {
                        if let Some(ref security) = part.security {
                            let security_length = part.security_length();
                            eprintln!(
                                "Blanking security region at {:#x} ({} bytes)...",
                                security.address, security_length
                            );
                            let zeros = vec![0u8; security_length];
                            programmer.write_custom_region(security.address, &zeros)?;
                        }

                        // Write high part of code options at 0x1100: all 00s except last byte from defaults
                        let high_part_size = part.option_byte_count.saturating_sub(4);
                        if high_part_size > 0 {
                            let mut high_bytes = vec![0u8; high_part_size];
                            if let Some(&last_default) = part.default_code_options.last() {
                                if let Some(last) = high_bytes.last_mut() {
                                    *last = last_default;
                                }
                            }
                            eprintln!(
                                "Writing code options high bytes at {:#x} ({} bytes)...",
                                0x1100, high_part_size
                            );
                            programmer.write_custom_region(0x1100, &high_bytes)?;
                        }
                    }

                    // Write custom fields if provided (customer_option first to write high bytes first)
                    if let Some(cust_opt_hex) = sub_matches.get_one::<String>("customer_option") {
                        let data = parse_hex(cust_opt_hex)?;
                        programmer.write_customer_option(&data)?;
                    }

                    // Security can only be written after mass erase
                    if let Some(security_hex) = sub_matches.get_one::<String>("security") {
                        if !is_mass_erase {
                            return Err("Security can only be written after mass erase".into());
                        }
                        let data = parse_hex(security_hex)?;
                        programmer.write_security(&data)?;
                    }

                    if let Some(customer_id_hex) = sub_matches.get_one::<String>("customer_id") {
                        let data = parse_hex(customer_id_hex)?;
                        if data.len() != 4 {
                            return Err("Customer ID must be exactly 4 bytes".into());
                        }
                        programmer.write_customer_id(data.as_slice().try_into().unwrap())?;
                    }

                    if let Some(op_num_hex) = sub_matches.get_one::<String>("operation_number") {
                        let data = parse_hex(op_num_hex)?;
                        if data.len() != 2 {
                            return Err("Operation number must be exactly 2 bytes".into());
                        }
                        programmer.write_operation_number(data.as_slice().try_into().unwrap())?;
                    }

                    if let Some(serial_hex) = sub_matches.get_one::<String>("serial_number") {
                        let data = parse_hex(serial_hex)?;
                        if data.len() != 4 {
                            return Err("Serial number must be exactly 4 bytes".into());
                        }
                        programmer.write_serial_number(data.as_slice().try_into().unwrap())?;
                    }

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
                _ => unreachable!(),
            }
        }
        Some(("erase", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let programmer_name = sub_matches
                .get_one::<String>("programmer")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            match programmer_name {
                "sinodude-serial" => {
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

                    let mut programmer =
                        SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                    programmer.erase_init()?;

                    // Use sector-based erase for partial erases, mass erase otherwise
                    let is_partial_erase = start_addr.is_some() || end_addr.is_some();
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

                    // Blank out security region and set higher half of code options on full mass erase
                    if !is_partial_erase {
                        if let Some(ref security) = part.security {
                            let security_length = part.security_length();
                            eprintln!(
                                "Blanking security region at {:#x} ({} bytes)...",
                                security.address, security_length
                            );
                            let zeros = vec![0u8; security_length];
                            programmer.write_custom_region(security.address, &zeros)?;
                        }

                        // Write high part of code options at 0x1100: all 00s except last byte from defaults
                        let high_part_size = part.option_byte_count.saturating_sub(4);
                        if high_part_size > 0 {
                            let mut high_bytes = vec![0u8; high_part_size];
                            if let Some(&last_default) = part.default_code_options.last() {
                                if let Some(last) = high_bytes.last_mut() {
                                    *last = last_default;
                                }
                            }
                            eprintln!(
                                "Writing code options high bytes at {:#x} ({} bytes)...",
                                0x1100, high_part_size
                            );
                            programmer.write_custom_region(0x1100, &high_bytes)?;
                        }
                    }

                    programmer.finish()?;
                }
                _ => unreachable!(),
            }
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
