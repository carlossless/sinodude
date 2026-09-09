// Auto-generated from GPT file for CHS008

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
    code_option_mask: &hex!("00fe"),
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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 15,
            bits_end: 15,
            editable: false,
            states: IndexMap::from([
                (0, "Disable Watchdog"),
                (1, "Enable Watchdog"),
            ]),
        }),
        ("OP_LVR", OptionInfo {
            byte_index: 0,
            bits_start: 14,
            bits_end: 14,
            editable: false,
            states: IndexMap::from([
                (0, "Disable LVR"),
                (1, "Enable LVR"),
            ]),
        }),
        ("OP_OAP", OptionInfo {
            byte_index: 0,
            bits_start: 9,
            bits_end: 9,
            editable: false,
            states: IndexMap::from([
                (0, "Disable Operation Amplifier"),
                (1, "Enable Operation Amplifier"),
                (0, "Normal IO"),
                (1, "PPG &CMP"),
                (0, "Disable"),
                (1, "Enable"),
                (0, "Disconnected"),
                (1, "Connected"),
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
    ])
}
