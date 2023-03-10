use clap::*;
use std::fs::File;
use std::io::Write;

mod sinolink;
use sinolink::*;

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
            .long_flag("read")
            .about("Read the chips flash contents.")
            .arg(arg!(output_file: <OUTPUT_FILE> "file to write flash contents to"))
    );
}

fn main() {
  let matches = cli().get_matches();

  match matches.subcommand() {
    Some(("read", sub_matches)) => {
      let output_file = sub_matches.get_one::<String>("output_file").map(|s| s.as_str()).unwrap();

      let sinolink = Sinolink::new();
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
    _ => unreachable!()
  }
}
