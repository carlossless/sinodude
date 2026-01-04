use hex_literal::*;
use phf::*;

pub struct Part {
    pub part_number: [u8; 5],
    pub model: [u8; 6],
    pub chip_type: u8,
    pub custom_block: u8,
    pub product_block: u8,
    pub flash_size: usize,
    pub default_code_options: [u8; 8],
    pub jtag_id: u16,
    pub sector_size: usize,
}

pub const PART_68F90A: Part = Part {
    part_number: hex!("68f90a0000"),
    model: hex!("06080f09000a"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    default_code_options: hex!("a4e063c00f000088"),
    jtag_id: 0xf690,
    sector_size: 512,
};

pub static PARTS: phf::Map<&'static str, Part> = phf_map! {
    "68f90a" => PART_68F90A
};
