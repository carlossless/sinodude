use indexmap::IndexMap;
use phf::phf_map;

pub mod sh68f1000;
pub mod sh68f1001;
pub mod sh68f881;
pub mod sh68f89;
pub mod sh68f90;
pub mod sh68f90a;
pub mod sh68f91;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Flash,
    Custom,
}

#[derive(Debug, Clone, Copy)]
pub struct AddressField {
    pub address: u32,
}

#[derive(Debug, Clone)]
pub struct OptionInfo {
    pub byte_index: usize,
    pub bits_start: usize,
    pub bits_end: usize,
    pub editable: bool,
    pub states: IndexMap<u8, &'static str>,
}

pub type Options = IndexMap<&'static str, OptionInfo>;

/// Parsed option with its current value and description
#[derive(Debug, Clone)]
pub struct ParsedOption {
    pub name: &'static str,
    pub byte_index: usize,
    pub bits_start: usize,
    pub bits_end: usize,
    pub editable: bool,
    pub raw_value: u8,
    pub description: Option<&'static str>,
}

/// Parse code options bytes using the provided options metadata
pub fn parse_code_options(code_options: &[u8], options: &Options) -> Vec<ParsedOption> {
    let mut parsed = Vec::new();

    for (name, info) in options {
        if info.byte_index >= code_options.len() {
            continue;
        }

        let byte = code_options[info.byte_index];
        let bit_count = info.bits_end - info.bits_start + 1;
        let mask = (1u8 << bit_count) - 1;
        let raw_value = (byte >> info.bits_start) & mask;

        let description = info.states.get(&raw_value).copied();

        parsed.push(ParsedOption {
            name,
            byte_index: info.byte_index,
            bits_start: info.bits_start,
            bits_end: info.bits_end,
            editable: info.editable,
            raw_value,
            description,
        });
    }

    parsed
}

/// Format parsed options for user-friendly display as a table
pub fn format_parsed_options(parsed: &[ParsedOption]) -> String {
    if parsed.is_empty() {
        return String::new();
    }

    // Calculate column widths
    let name_width = parsed
        .iter()
        .map(|o| o.name.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let desc_width = parsed
        .iter()
        .map(|o| o.description.unwrap_or("(unknown)").len())
        .max()
        .unwrap_or(11)
        .max(11);

    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "{:<name_width$}  {:>5}  {:>8}  {:<desc_width$}\n",
        "Name",
        "Value",
        "Editable",
        "Description",
        name_width = name_width,
        desc_width = desc_width
    ));

    // Separator
    output.push_str(&format!(
        "{:-<name_width$}  {:->5}  {:->8}  {:-<desc_width$}\n",
        "",
        "",
        "",
        "",
        name_width = name_width,
        desc_width = desc_width
    ));

    // Rows
    for opt in parsed {
        let desc = opt.description.unwrap_or("(unknown)");
        let editable = if opt.editable { "yes" } else { "no" };
        output.push_str(&format!(
            "{:<name_width$}  {:>5}  {:>8}  {}\n",
            opt.name,
            opt.raw_value,
            editable,
            desc,
            name_width = name_width
        ));
    }

    output
}

pub struct Part {
    pub part_number: [u8; 5],
    pub chip_type: u8,
    pub custom_block: u8,
    pub product_block: u8,
    pub flash_size: usize,
    pub default_code_options: &'static [u8],
    pub code_option_mask: &'static [u8],
    pub jtag_id: u16,
    pub sector_size: usize,
    pub option_byte_count: usize,
    pub security_level: u8,
    pub customer_id: Option<AddressField>,
    pub operation_number: Option<AddressField>,
    pub customer_option: Option<AddressField>,
    pub security: Option<AddressField>,
    pub serial_number: Option<AddressField>,
    pub options: fn() -> Options,
}

impl Part {
    /// Returns true if options are stored in flash memory.
    /// custom_block values 2, 3, 4 are NOT in flash; all others ARE in flash.
    pub fn options_in_flash(&self) -> bool {
        !matches!(self.custom_block, 2 | 3 | 4)
    }

    /// Returns the region for reading options.
    pub fn options_region(&self) -> Region {
        if self.options_in_flash() {
            Region::Flash
        } else {
            Region::Custom
        }
    }

    /// Returns the security region length for this part.
    pub fn security_length(&self) -> usize {
        if self.part_number == sh68f90::PART.part_number
            || self.part_number == sh68f90a::PART.part_number
        {
            17
        } else {
            panic!(
                "Security length not defined for this part (part_number: {:02x?})",
                self.part_number
            )
        }
    }

    /// Returns the non-editable default bits for the upper code options (bytes 4+).
    /// Returns None if option_byte_count <= 4.
    pub fn upper_code_option_defaults(&self) -> Option<Vec<u8>> {
        if self.option_byte_count <= 4 {
            return None;
        }

        let upper_len = self.option_byte_count - 4;
        let mut upper = vec![0u8; upper_len];

        // For each byte, use non-editable bits from defaults (mask 0 = not editable)
        for i in 0..upper_len {
            let idx = 4 + i;
            if idx < self.default_code_options.len() && idx < self.code_option_mask.len() {
                // Non-editable bits: defaults & !mask
                upper[i] = self.default_code_options[idx] & !self.code_option_mask[idx];
            }
        }

        Some(upper)
    }
}

pub static PARTS: phf::Map<&'static str, &'static Part> = phf_map! {
    "sh68f881" => &sh68f881::PART,
    "sh68f89" => &sh68f89::PART,
    "sh68f90" => &sh68f90::PART,
    "sh68f90a" => &sh68f90a::PART,
    "sh68f91" => &sh68f91::PART,
    "sh68f1000" => &sh68f1000::PART,
    "sh68f1001" => &sh68f1001::PART,
};
