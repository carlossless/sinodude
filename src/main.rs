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
                ),
        )
        .subcommand(
            Command::new("erase")
                .short_flag('e')
                .about("Mass erase the chip's flash")
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
                    let mut programmer =
                        SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                    programmer.write_init()?;
                    programmer.write_flash(&firmware)?;
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
                    let mut programmer =
                        SinodudeSerialProgrammer::new(port, part, cancelled.clone())?;
                    programmer.erase_init()?;
                    programmer.mass_erase()?;
                    // TODO: figure out security_levels + chip types and handle this properly
                    // Also erase the custom region from the security address
                    if let Some(ref security) = part.security {
                        // Determine security region length based on security_level
                        let security_length: u16 = match part.security_level {
                            4 => 17,
                            _ => 8, // RANDOM
                        };
                        eprintln!(
                            "Erasing custom region at security address {:#x} ({} bytes)...",
                            security.address, security_length
                        );
                        programmer.erase_custom_region(security.address, security_length)?;
                        eprintln!("Custom region erase complete");
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
