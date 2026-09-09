// Auto-generated from GPT file for 69P21A

use super::{SecurityRecordFormat, AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("0000000000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 0,
    eeprom_size: 0,
    default_code_options: &hex!("0000"),
    code_option_mask: &hex!("ffff"),
    jtag_id: 0x0000,
    sector_size: 0,
    option_byte_count: 2,
    security_level: 0,
    bank_type: 0,
    single_wire: false,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0x0000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[],
    options,
    erase_full_mode: 5,
    option_high_addr: None,
    security_record_format: SecurityRecordFormat::Record19,
    isp_password_addr: None,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 13,
            bits_end: 15,
            editable: false,
            states: IndexMap::from([
                (3, "Ineternal 8M"),
                (4, "External RC 400k-8MHz"),
                (5, "Crystal/Ceramic 400k-8MHz(Normal)"),
                (6, "Crystal/Ceramic 400k-8MHz(Strong)"),
                (7, "Crystal 32.768kHz"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 11,
            bits_end: 12,
            editable: false,
            states: IndexMap::from([
                (0, "WDT Disable"),
                (1, "WDT Enable"),
            ]),
        }),
        ("OP_LVR", OptionInfo {
            byte_index: 0,
            bits_start: 10,
            bits_end: 10,
            editable: false,
            states: IndexMap::from([
                (0, "LVR Disable"),
                (1, "LVR Enable"),
            ]),
        }),
        ("OP_LVR0", OptionInfo {
            byte_index: 0,
            bits_start: 9,
            bits_end: 9,
            editable: false,
            states: IndexMap::from([
                (0, "LVR High(4V)"),
                (1, "LVR Low (2.5V)"),
            ]),
        }),
        ("OP_Reset", OptionInfo {
            byte_index: 0,
            bits_start: 8,
            bits_end: 8,
            editable: false,
            states: IndexMap::from([
                (0, "PORTA.3 used as Reset"),
                (1, "PORTA,3 used as IO"),
            ]),
        }),
    ])
}
