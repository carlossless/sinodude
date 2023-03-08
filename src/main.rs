use std::{time::Duration};

use rusb::*;
use hex_literal::*;

mod sinolink;
use sinolink::*;

fn main() {
  let sinolink = Sinolink::new();
  sinolink.init();
  sinolink.read_flash();
}
