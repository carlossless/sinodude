use clap::*;
use log::info;
use simple_logger::SimpleLogger;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{env, fs};

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
                .about("Read the chip's code flash and/or data EEPROM into Intel HEX files")
                .arg(arg!(--flash <FILE> "Write the code flash contents to this file").required(false))
                .arg(arg!(--eeprom <FILE> "Write the data EEPROM contents to this file").required(false))
                .group(ArgGroup::new("memory").args(["flash", "eeprom"]).multiple(true).required(true))
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
                    arg!(--power <MODE> "Target power for the sinolink programmer: 3v3, 5v or external")
                        .required(false),
                )
                .arg(
                    arg!(--supply <RAIL> "Deprecated spelling of --power")
                        .required(false)
                        .hide(true),
                )
                .arg(
                    arg!(--key <KEY> "Customer unlock key for a password-protected part (8 bytes hex)")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("write")
                .short_flag('w')
                .about("Write Intel HEX files to the chip's code flash and/or data EEPROM")
                .arg(arg!(--flash <FILE> "Program the code flash from this file").required(false))
                .arg(arg!(--eeprom <FILE> "Program the data EEPROM from this file (erases the whole EEPROM first)").required(false))
                .group(ArgGroup::new("memory").args(["flash", "eeprom"]).multiple(true).required(true))
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
                    arg!(--power <MODE> "Target power for the sinolink programmer: 3v3, 5v or external")
                        .required(false),
                )
                .arg(
                    arg!(--supply <RAIL> "Deprecated spelling of --power")
                        .required(false)
                        .hide(true),
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
                    arg!(--erase_mode <MODE> "Force a vendor erase-mode index (0-5) instead of the part's own; 3 and 4 are not otherwise reachable")
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
                    arg!(--power <MODE> "Target power for the sinolink programmer: 3v3, 5v or external")
                        .required(false),
                )
                .arg(
                    arg!(--supply <RAIL> "Deprecated spelling of --power")
                        .required(false)
                        .hide(true),
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
                    arg!(--power <MODE> "Target power for the sinolink programmer: 3v3, 5v or external")
                        .required(false),
                )
                .arg(
                    arg!(--supply <RAIL> "Deprecated spelling of --power")
                        .required(false)
                        .hide(true),
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
                )
                .arg(
                    arg!(--password <PASSWORD> "Set the customer password (6 bytes hex); unlock later with --key <password><2 trailing record bytes>")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("sinolink")
                .about("Drive a genuine SinoLink USB dongle directly (diagnostics and raw access)")
                .subcommand_required(true)
                .subcommand(Command::new("probe").about("Identify the dongle and measure the target supply"))
                .subcommand(
                    Command::new("connect")
                        .about("Download the part's config blob, power the target and connect")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode: 1 = ICP, 2 = OCD").required(false)),
                )
                .subcommand(
                    Command::new("dump")
                        .about("Connect and read a range of target memory to stdout")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode: 1 = ICP, 2 = OCD").required(false))
                        .arg(arg!(--addr <ADDR> "Start address").required(false))
                        .arg(arg!(--len <LEN> "Byte count").required(false))
                        .arg(arg!(--region <REGION> "code or custom").required(false))
                        .arg(arg!(--out <FILE> "Write the bytes to this file as Intel HEX").required(false))
                        .arg(arg!(--bin <FILE> "Write the bytes to this file raw").required(false)),
                )
                .subcommand(
                    Command::new("raw")
                        .about("Issue one vendor control-IN and print the reply")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode").required(false))
                        .arg(arg!(--req <N> "bRequest").required(true))
                        .arg(arg!(--value <N> "wValue").required(false))
                        .arg(arg!(--index <N> "wIndex").required(false))
                        .arg(arg!(--len <N> "wLength").required(false)),
                )
                .subcommand(
                    Command::new("recover")
                        .about("Erase then write the upper code options in one ICP session")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode").required(false))
                        .arg(arg!(--erase_mode <MODE> "full, mass or protected").required(false))
                        .arg(arg!(--upper <HEX> "Upper 4 option bytes, default 00000088").required(false)),
                )
                .subcommand(
                    Command::new("writeopts")
                        .about("Stage code options from the blob and trigger the deferred commit")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode").required(false))
                        .arg(arg!(--omode <M> "Option drive mode, 0 or 1").required(false))
                        .arg(arg!(--go <WHICH> "ready, go, both or none").required(false)),
                )
                .subcommand(
                    Command::new("options")
                        .about("Drive code options into the target from the config blob")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode: 1 = ICP, 2 = OCD").required(false))
                        .arg(arg!(--addr <ADDR> "Offset into the config blob (default 0x30)").required(false))
                        .arg(arg!(--len <LEN> "Option byte count (default 8)").required(false))
                        .arg(arg!(--omode <M> "Option drive mode, 0 or 1").required(false)),
                )
                .subcommand(
                    Command::new("erase")
                        .about("Connect and erase the target")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode: 1 = ICP, 2 = OCD").required(false))
                        .arg(arg!(--erase_mode <MODE> "mass, full, protected, sector or page").required(false))
                        .arg(arg!(--addr <ADDR> "Address, for sector and page erases").required(false)),
                )
                .subcommand(
                    Command::new("program")
                        .about("Connect and program a raw binary into the target")
                        .arg(
                            arg!(-p --part <PART>)
                                .value_parser(PARTS.keys().copied().collect::<Vec<_>>())
                                .required(true),
                        )
                        .arg(arg!(--power <MODE> "Target power: 3v3, 5v or external").required(false))
                        .arg(arg!(--supply <RAIL> "Deprecated spelling of --power").required(false).hide(true))
                        .arg(arg!(--key <KEY> "Customer unlock key, 8 bytes hex").required(false))
                        .arg(arg!(--mode <MODE> "Connect mode: 1 = ICP, 2 = OCD").required(false))
                        .arg(arg!(--bin <FILE> "Raw binary to program").required(true))
                        .arg(arg!(--addr <ADDR> "Start address").required(false))
                        .arg(arg!(--region <REGION> "code or custom").required(false))
                        .arg(arg!(--opt_prep <MODE> "Run bRequest 0x11 with this drive mode before programming").required(false)),
                ),
        )
}

fn make_programmer(
    sub: &ArgMatches,
    part: &'static Part,
    cancelled: Arc<AtomicBool>,
) -> Result<Box<dyn Programmer>, Box<dyn std::error::Error>> {
    match sub.get_one::<String>("programmer").map(|s| s.as_str()) {
        Some("sinolink") => {
            let power = sinolink_power(sub);
            Ok(Box::new(SinoLinkProgrammer::new(part, cancelled, power)?))
        }
        _ => {
            let port = sub
                .get_one::<String>("port")
                .ok_or("--port is required for the sinodude-serial programmer")?;
            Ok(Box::new(SinodudeSerialProgrammer::new(
                port, part, cancelled,
            )?))
        }
    }
}

/// --power wins; --supply is kept as the older spelling.
fn sinolink_power(sub: &ArgMatches) -> Power {
    let v = sub
        .get_one::<String>("power")
        .or_else(|| sub.get_one::<String>("supply"));
    Power::parse(v.map(|s| s.as_str()))
}

fn sinolink_cmd(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let link = SinoLink::open()?;

    let (action, sub) = matches.subcommand().unwrap();
    if action == "probe" {
        let info = link.probe()?;
        println!("probe:        {:02x?}", &info[..16]);
        println!("target info:  {:02x?}", &info[0x10..0x20]);
        println!("signature:    {:02x?}", &info[0x32..0x3a]);
        println!("status word:  {:#04x}", link.status_word()?);
        let (code, subres) = link.op_status()?;
        println!(
            "op status:    {:#04x} ({}) sub={:#010x}",
            code,
            decode_status(code),
            subres
        );
        println!("PB7 pull-up:  {}", link.sample_data_pin(1)?);
        println!("PB7 pull-dn:  {}", link.sample_data_pin(2)?);
        return Ok(());
    }

    let part_name = sub.get_one::<String>("part").unwrap().as_str();
    let part = PARTS.get(part_name).unwrap();
    let power = sinolink_power(sub);
    power.check(part)?;
    let mode: u8 = sub
        .get_one::<String>("mode")
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(1);

    println!(
        "part {} chip_type {} flash {:#x}",
        part_name, part.chip_type, part.flash_size
    );
    if !blob_is_verified(part) {
        eprintln!("warning: no verified config blob for {part_name}; using the SH68F90A capture with fields patched, results are not trustworthy");
    }
    let write_mode = matches!(
        action,
        "erase" | "program" | "options" | "writeopts" | "recover" | "raw"
    );
    // Same power-on reset the Programmer path does: a previous run leaves the rail switched off,
    // and re-applying it without this drops the ICP handshake with status 0x09.
    let _ = link.power_reset(0);
    std::thread::sleep(std::time::Duration::from_millis(300));
    let mv = match power {
        Power::V5_0 => link.set_supply(Supply::RailB)?,
        Power::V3_3 => link.set_supply(Supply::RailA)?,
        Power::External => 0,
    };
    std::thread::sleep(std::time::Duration::from_millis(300));
    println!("target power: {power} ({mv} mV)");
    let blob = build_blob(part, power, write_mode);
    link.download_blob(&blob)?;
    let status = link.connect(mode)?;
    std::thread::sleep(std::time::Duration::from_millis(20));
    if let Some(k) = sub.get_one::<String>("key") {
        if let Ok(s) = std::env::var("SINOLINK_PREKEY") {
            for sc in s.split(",").filter(|x| !x.is_empty()) {
                let n: u8 = sc.trim().parse().unwrap_or(0);
                match link.power_reset(n) {
                    Ok(r) => println!("pre-key 0x41 sub {n} -> {:02x?}", r),
                    Err(e) => println!("pre-key 0x41 sub {n} -> {e}"),
                }
            }
        }
        let bytes = parse_hex(k)?;
        let key: [u8; 8] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| "key must be 8 bytes")?;
        let r = link.unlock(&key)?;
        println!("unlock reply: {:02x?}", &r[..r.len().min(8)]);
        let marker = link.read(part.customer_option.address, Region::Custom, 1)?;
        println!(
            "unlock key sent, marker at {:#x} = {:02x?} (0xc0 = accepted)",
            part.customer_option.address, marker
        );
    }
    println!("connected, status {:02x?}", &status[..status.len().min(8)]);

    if action == "raw" {
        let num = |k: &str, d: u64| -> Result<u64, Box<dyn std::error::Error>> {
            Ok(match sub.get_one::<String>(k) {
                Some(v) => parse_addr(v)? as u64,
                None => d,
            })
        };
        let req = num("req", 0)? as u8;
        let value = num("value", 0)? as u16;
        let index = num("index", 0)? as u16;
        let len = num("len", 16)? as u16;
        match link.raw_in(req, value, index, len) {
            Ok(d) => {
                let nonff = d.iter().filter(|&&b| b != 0xff).count();
                println!(
                    "req {req:#04x} val {value:#06x} -> ({}) {:02x?}  non-ff {nonff}",
                    d.len(),
                    d
                );
            }
            Err(e) => println!("req {req:#04x} val {value:#06x} -> {e}"),
        }
        return Ok(());
    }

    if action == "recover" {
        let em = match sub.get_one::<String>("erase_mode").map(|s| s.as_str()) {
            Some("mass") => EraseMode::Mass,
            Some("protected") => EraseMode::ProtectedMass,
            _ => EraseMode::Full,
        };
        let upper = parse_hex(
            sub.get_one::<String>("upper")
                .map(|s| s.as_str())
                .unwrap_or("00000088"),
        )?;
        println!("erase {em:?} then write {upper:02x?} at 0x1100 in the same session");
        link.erase(0, em)?;
        match link.program(0x1100, Region::Custom, &upper) {
            Ok(()) => println!("option write ok"),
            Err(e) => println!("option write failed: {e}"),
        }
        let back = link.read(0x1100, Region::Custom, 8)?;
        println!("0x1100 now: {:02x?}", &back);
        return Ok(());
    }

    if action == "writeopts" {
        let omode: u8 = sub
            .get_one::<String>("omode")
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(0);
        let go = sub
            .get_one::<String>("go")
            .map(|s| s.as_str())
            .unwrap_or("both");
        println!("staging options (mode {omode}) then go={go}");
        link.load_option_bytes(0x30, 8, omode)?;
        if go == "ready" || go == "both" {
            link.set_ready()?;
        }
        if go == "go" || go == "both" {
            link.set_go()?;
        }
        match link.wait_idle(std::time::Duration::from_secs(10)) {
            Ok(()) => println!("commit ok"),
            Err(e) => println!("commit result: {e}"),
        }
        return Ok(());
    }

    if action == "options" {
        let off = sub
            .get_one::<String>("addr")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(0x30) as u32;
        let len = sub
            .get_one::<String>("len")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(8) as u16;
        let omode: u8 = sub
            .get_one::<String>("omode")
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(0);
        println!("staging {len} option byte(s) from blob offset {off:#x}, mode {omode}");
        link.load_option_bytes(off, len, omode)?;
        println!("options ok");
        return Ok(());
    }

    if action == "erase" {
        let em = match sub.get_one::<String>("erase_mode").map(|s| s.as_str()) {
            Some("full") => EraseMode::Full,
            Some("protected") => EraseMode::ProtectedMass,
            Some("sector") => EraseMode::Sector { page: false },
            Some("page") => EraseMode::Sector { page: true },
            _ => EraseMode::Mass,
        };
        let addr = sub
            .get_one::<String>("addr")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(0) as u32;
        println!("erasing {em:?} at {addr:#x}");
        link.erase(addr, em)?;
        println!("erase ok");
        return Ok(());
    }

    if action == "program" {
        let file = sub.get_one::<String>("bin").unwrap();
        let data = fs::read(file)?;
        let addr = sub
            .get_one::<String>("addr")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(0) as u32;
        let region = match sub.get_one::<String>("region").map(|s| s.as_str()) {
            Some("custom") => Region::Custom,
            _ => Region::Flash,
        };
        if let Some(pm) = sub.get_one::<String>("opt_prep") {
            let pm: u8 = pm.parse()?;
            println!("staging options from blob 0x30 len 8, drive mode {pm}");
            link.load_option_bytes(0x30, 8, pm)?;
        }
        println!("programming {} bytes at {:#x}", data.len(), addr);
        for (i, chunk) in data.chunks(0x400).enumerate() {
            link.program(addr + (i * 0x400) as u32, region, chunk)?;
        }
        println!("program ok");
        return Ok(());
    }

    if action == "dump" {
        let addr = sub
            .get_one::<String>("addr")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(0) as u32;
        let len = sub
            .get_one::<String>("len")
            .map(|s| parse_addr(s))
            .transpose()?
            .unwrap_or(0x100);
        let region = match sub.get_one::<String>("region").map(|s| s.as_str()) {
            Some("custom") => Region::Custom,
            _ => Region::Flash,
        };
        let mut out = Vec::with_capacity(len);
        let mut off = 0usize;
        while off < len {
            let chunk = (len - off).min(0x400);
            out.extend_from_slice(&link.read(addr + off as u32, region, chunk as u16)?);
            off += chunk;
        }
        if len <= 0x400 {
            for (i, row) in out.chunks(16).enumerate() {
                let hex: Vec<String> = row.iter().map(|b| format!("{:02x}", b)).collect();
                println!("{:06x}  {}", addr as usize + i * 16, hex.join(" "));
            }
        }
        println!("md5 {:x}", md5::compute(&out));
        if let Some(file) = sub.get_one::<String>("bin") {
            fs::write(file, &out)?;
        }
        if let Some(file) = sub.get_one::<String>("out") {
            fs::write(file, to_ihex(out)?)?;
        }
    }
    Ok(())
}

fn run(cancelled: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("sinolink", sub_matches)) => return sinolink_cmd(sub_matches),
        Some(("read", sub_matches)) => {
            let flash_file = sub_matches.get_one::<String>("flash");
            let eeprom_file = sub_matches.get_one::<String>("eeprom");

            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();
            if eeprom_file.is_some() && part.eeprom_size == 0 {
                return Err(format!("{} has no data EEPROM", part_name).into());
            }

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
            programmer.read_init()?;
            let flash = flash_file.map(|_| programmer.read_flash()).transpose()?;
            let eeprom = eeprom_file.map(|_| programmer.read_eeprom()).transpose()?;
            programmer.finish()?;

            for (file, data) in [(flash_file, flash), (eeprom_file, eeprom)] {
                if let (Some(file), Some(data)) = (file, data) {
                    info!("{} MD5: {:x}", file, md5::compute(&data));
                    fs::write(file, to_ihex(data)?)?;
                }
            }
        }
        Some(("write", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();

            let firmware = sub_matches
                .get_one::<String>("flash")
                .map(|file| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                    let mut data = from_ihex(&fs::read_to_string(file)?, part.flash_size)?;
                    data.resize(part.flash_size, 0);
                    Ok(data)
                })
                .transpose()?;

            let eeprom = sub_matches
                .get_one::<String>("eeprom")
                .map(|file| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                    if part.eeprom_size == 0 {
                        return Err(format!("{} has no data EEPROM", part_name).into());
                    }
                    let mut data = from_ihex(&fs::read_to_string(file)?, part.eeprom_size)?;
                    data.resize(part.eeprom_size, 0);
                    Ok(data)
                })
                .transpose()?;

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

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
            if let Some(key) = sub_matches.get_one::<String>("key") {
                let bytes = parse_hex(key)?;
                let key: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "Unlock key must be exactly 8 bytes")?;
                programmer.set_unlock_key(key);
            }
            programmer.write_init()?;

            // Flash goes first: its mass erase may take the EEPROM with it.
            if let Some(firmware) = &firmware {
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
                        programmer.write_flash_range(firmware, start, end)?;
                    }
                    (Some(start), None) => {
                        programmer.write_flash_range(firmware, start, firmware.len())?;
                    }
                    (None, Some(end)) => {
                        programmer.write_flash_range(firmware, 0, end)?;
                    }
                    (None, None) => {
                        programmer.write_flash(firmware)?;
                    }
                }
            }

            if let Some(eeprom) = &eeprom {
                programmer.write_eeprom(eeprom)?;
            }

            programmer.finish()?;
        }
        Some(("security", sub_matches)) => {
            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();
            let part = PARTS.get(part_name).unwrap();

            let mut programmer = make_programmer(sub_matches, part, cancelled.clone())?;
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
            let new_password: Option<[u8; PROTECTION_PASSWORD_LEN]> =
                match sub_matches.get_one::<String>("password") {
                    Some(p) => Some(
                        parse_hex(p)?
                            .as_slice()
                            .try_into()
                            .map_err(|_| "Password must be exactly 6 bytes")?,
                    ),
                    None => None,
                };

            if read_spec.is_none() && write_spec.is_none() && new_password.is_none() {
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
                if new_password.is_some() {
                    eprintln!("Setting a customer password.");
                }
                eprintln!("This cannot be undone without a full mass erase.");
                programmer.apply_protection(&read, &write, new_password.as_ref())?;
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
            if sub_matches.get_flag("eeprom") && part.eeprom_size == 0 {
                return Err(format!("{} has no data EEPROM", part_name).into());
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
