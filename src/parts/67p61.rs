// Auto-generated from GPT file for 67P61

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
    code_option_mask: &hex!("80ff"),
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
            bits_start: 12,
            bits_end: 15,
            editable: false,
            states: IndexMap::from([
                (0, "Internal RC 32.768kHz + Internal RC 10MHz"),
                (5, "Internal RC 32.768kHz + Ceramic / Crystal 400kHz - 10MHz(Normal)"),
                (6, "Internal RC 32.768kHz + Ceramic / Crystal 400kHz - 10MHz(Strong)"),
                (7, "Crystal 32.768kHz + Internal RC 10MHz"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 11,
            bits_end: 11,
            editable: false,
            states: IndexMap::from([
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
        ("OP_WDT_STOP", OptionInfo {
            byte_index: 0,
            bits_start: 9,
            bits_end: 10,
            editable: false,
            states: IndexMap::from([
                (0, "Disable"),
                (2, "Enable(WDT Enable,CPU Reset When WDT overflow)"),
                (3, "Enable(WDT Enable,Wake up STOP When WDT overflow)"),
            ]),
        }),
        ("OP_LVR", OptionInfo {
            byte_index: 0,
            bits_start: 8,
            bits_end: 8,
            editable: false,
            states: IndexMap::from([
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
        ("OP_LVR0", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "LVR High(4.0V)"),
                (1, "LVR Low(2.5V)"),
            ]),
        }),
    ])
}
