// Auto-generated from GPT file for SH68F1050

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("68f1050000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("001000c000000000"),
    code_option_mask: &hex!("00e001c000000080"),
    jtag_id: 0xf105,
    sector_size: 1024,
    option_byte_count: 8,
    security_level: 10,
    bank_type: 0,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0xfff8000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 6,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level"),
                (1, "3.7V LVR Level"),
                (2, "2.8V LVR Level"),
                (3, "2.3V LVR Level"),
            ]),
        }),
        ("OP_LVRFEN", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "LVRF Disable"),
                (1, "LVRF Enable. Voltage is defined in OP_LVRLE"),
            ]),
        }),
        ("OP_LVRA", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: false,
            states: IndexMap::from([
                (0, "LVRA takes effect"),
                (1, "LVRF used as LVRA(1.9V LVR Level)"),
            ]),
        }),
        ("OP_NRST", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "PD10 used as NRST pin"),
                (1, "PD10 used as I/O pin"),
            ]),
        }),
        ("OP_LDO_DC", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "LDO Constant Current : 0mA"),
                (1, "LDO Constant Current : 1mA"),
                (2, "LDO Constant Current : 2mA"),
                (3, "LDO Constant Current : 4mA"),
            ]),
        }),
        ("OP_CORE_VOL", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Core 1.55V"),
                (1, "Core 1.40V"),
                (2, "Core 1.65V"),
                (3, "Core 1.60V"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 7,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable ISP Function"),
                (1, "Disable ISP Function"),
            ]),
        }),
    ])
}
