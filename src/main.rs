use clap::*;
use std::fs::*;
use std::io::{Read, Write};

pub mod part;
mod gpt;
mod programmer;

use part::*;
use gpt::*;
use programmer::{sinolink::*, PowerSetting};

use crc::*;

const CUSTOM_ALG: Algorithm<u8> = Algorithm {
    width: 8,
    poly: 0x00,
    init: 0x00,
    refin: false,
    refout: true,
    xorout: 0x00,
    check: 0x00,
    residue: 0x00,
};

pub const CDUDE: Crc<u8> = Crc::<u8>::new(&CUSTOM_ALG);

fn cli() -> Command {
    return Command::new("sinodude")
        .about("programming tool for sinowealth devices")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Karolis Stasaitis")
        .subcommand(
            Command::new("read")
                .short_flag('r')
                .about("Read the chips flash contents.")
                .arg(arg!(output_file: <OUTPUT_FILE> "file to write flash contents to"))
                .arg(
                    arg!(-c --programmer <PART>)
                        .value_parser(["sinolink"])
                        .required(true),
                )
                .arg(
                    arg!(-p --part <PART>)
                        .value_parser(PARTS.keys().map(|&s| s).collect::<Vec<_>>())
                        .required(true),
                )
                .arg(
                    arg!(-t --power <POWER_SETTING>)
                        .value_parser(["3v3", "5v", "external"])
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .long_flag("decrypt")
                .about("Decrypt GPT file")
                .arg(arg!(gpt_file: <GPT_FILE> "gpt file to decrypt")),
        );
}

fn main() {
    // let s = hex!("
    //   07 // Chip Type
    //   00
    //   02 // Power setting - 0x02 - 5V, 0x01 - 3.3V, 0x03 - External (3.3V or 5V seems to not matter)
    //   0402040000050000
    //   0301 // Custom Block / Product Block (???)
    //   0620000000000000000800000000000000000000000000000000000000000008
    //   a4e063c00f000088 // code options
    //   00000000000000000000010040ff0000fd8f3600000000000000000000000100b36300000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c11
    //   06080f09000a // looks like chip model
    //   ff000000000000091200000500
    //   68f90a // compressed model number
    //   00000000000000040000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000012000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    //   230308203607 // current date 2023-03-08 20:36:07
    //   05500000000000000000
    // ");

    // assert_eq!(CDUDE.checksum(&s), 0xbd);

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read", sub_matches)) => {
            let output_file = sub_matches
                .get_one::<String>("output_file")
                .map(|s| s.as_str())
                .unwrap();

            let power_setting_name = sub_matches
                .get_one::<String>("power")
                .map(|s| s.as_str())
                .unwrap();

            let power_setting = PowerSetting::from_option(power_setting_name);

            let part_name = sub_matches
                .get_one::<String>("part")
                .map(|s| s.as_str())
                .unwrap();

            let part = PARTS.get(part_name).unwrap();
            let sinolink = Sinolink::new(part, power_setting);
            sinolink.init();

            let buf = sinolink.read_flash();
            let mut file = File::create(output_file).unwrap();
            for chunk in buf.chunks(16) {
                for x in &chunk[0..16] {
                    write!(file, "{:02X}", x).unwrap();
                }
                write!(file, "\n").unwrap();
            }
        }
        Some(("decrypt", sub_matches)) => {
            let output_file = sub_matches
                .get_one::<String>("gpt_file")
                .map(|s| s.as_str())
                .unwrap();
            let keypair = GPTDecryptor::keypair(output_file);

            let file = File::open(output_file).unwrap();
            let decrypted = GPTDecryptor::decrypt(file.bytes().scan((), |_, x| x.ok()), keypair);

            write(format!("{}.decrypted", output_file), decrypted).unwrap();
        }
        _ => unreachable!(),
    }
}
