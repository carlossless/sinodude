use clap::*;
use log::info;
use simple_logger::SimpleLogger;
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
;
}

fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Off)
        .env()
        .init()
        .unwrap();

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
                    let sinolink = match SinolinkProgrammer::new(part, power_setting) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    };
                    if let Err(e) = sinolink.read_init() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    match sinolink.read_flash() {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    }
                }
                "sinodude-serial" => {
                    let port = sub_matches
                        .get_one::<String>("port")
                        .expect("--port is required for sinodude-serial programmer");
                    let mut programmer = match SinodudeSerialProgrammer::new(port, part) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    };
                    if let Err(e) = programmer.read_init() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    let result = match programmer.read_flash() {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    };
                    if let Err(e) = programmer.finish() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    result
                }
                _ => unreachable!(),
            };

            let digest = md5::compute(&result);
            info!("MD5: {:x}", digest);

            let ihex = to_ihex(result).unwrap();
            fs::write(output_file, ihex).unwrap();
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

            let mut file = fs::File::open(input_file).unwrap();
            let mut file_buf = Vec::new();
            file.read_to_end(&mut file_buf).unwrap();
            let file_str = String::from_utf8_lossy(&file_buf[..]);
            let mut firmware = from_ihex(&file_str, part.flash_size).unwrap();

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
                    let sinolink = match SinolinkProgrammer::new(part, power_setting) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    };
                    if let Err(e) = sinolink.write_init() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    if let Err(e) = sinolink.write_flash(&firmware[0..65536]) {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
                "sinodude-serial" => {
                    let port = sub_matches
                        .get_one::<String>("port")
                        .expect("--port is required for sinodude-serial programmer");
                    let mut programmer = match SinodudeSerialProgrammer::new(port, part) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Error: {e}");
                            std::process::exit(1);
                        }
                    };
                    if let Err(e) = programmer.write_init() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    if let Err(e) = programmer.write_flash(&firmware) {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                    if let Err(e) = programmer.finish() {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
