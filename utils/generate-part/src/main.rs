use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GptError {
    #[error("Invalid GPT header")]
    InvalidHeader,
    #[error("Invalid decrypted content")]
    InvalidContent,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

type KeyPair = (u8, u8);
const HEADER_SIZE: usize = 17;

fn decrypt(mut input: impl Iterator<Item = u8>, keys: KeyPair) -> Result<Vec<u8>, GptError> {
    let header: Vec<u8> = input.by_ref().take(HEADER_SIZE).collect();
    if header != b"[Version]\r\n3.00\r\n" {
        return Err(GptError::InvalidHeader);
    }

    let mut result: Vec<u8> = vec![];

    for x in input {
        let mut num: u16 = x as u16;
        if num < keys.0 as u16 {
            num += 256;
        }
        let partial = (num - keys.0 as u16) as u8;
        result.push(partial ^ keys.1);
    }

    if result.len() < HEADER_SIZE + 10 || &result[HEADER_SIZE..(HEADER_SIZE + 10)] != b"[ChipName]" {
        return Err(GptError::InvalidContent);
    }

    Ok(result)
}

fn keypair(filename: &str) -> Result<KeyPair, GptError> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| GptError::Parse("Invalid filename".to_string()))?;
    let len = stem.len();
    if len < 4 {
        return Err(GptError::Parse("Filename too short for key extraction".to_string()));
    }
    let key1 = u8::from_str_radix(&stem[len - 2..len], 16)
        .map_err(|_| GptError::Parse("Invalid key1 in filename".to_string()))?;
    let key2 = u8::from_str_radix(&stem[len - 4..len - 2], 16)
        .map_err(|_| GptError::Parse("Invalid key2 in filename".to_string()))?;
    Ok((key1, key2))
}

#[derive(Debug, Clone)]
pub struct AddressField {
    pub region: u8,
    pub address: u32,
}

#[derive(Debug, Clone)]
pub struct PartDefinition {
    pub chip_name: String,
    pub part_number: String,
    pub chip_type: u8,
    pub custom_block: u8,
    pub product_block: u8,
    pub flash_size: usize,
    pub jtag_id: u16,
    pub sector_size: usize,
    pub external_ram: usize,
    pub eeprom: usize,
    pub initial_option: u64,
    pub option_mask: u64,
    pub option_byte_count: usize,
    pub customer_id: Option<AddressField>,
    pub operation_number: Option<AddressField>,
    pub customer_option: Option<AddressField>,
    pub security: Option<AddressField>,
    pub serial_number: Option<AddressField>,
    pub options: Vec<OptionDefinition>,
}

#[derive(Debug, Clone)]
pub struct OptionDefinition {
    pub name: String,
    pub byte_index: usize,
    pub bits_start: usize,
    pub bits_end: usize,
    pub items: Vec<(u8, String)>,
    pub editable: bool,
}

fn parse_gpt_content(content: &str) -> Result<PartDefinition, GptError> {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut options: Vec<OptionDefinition> = Vec::new();
    let mut in_options_section = false;
    let mut current_option: Option<OptionDefinition> = None;
    let mut current_items: Vec<(u8, String)> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line == "------ Options Begin ------" {
            in_options_section = true;
            continue;
        }
        if line == "------ Options End ------" {
            if let Some(mut opt) = current_option.take() {
                opt.items = std::mem::take(&mut current_items);
                options.push(opt);
            }
            in_options_section = false;
            continue;
        }

        if in_options_section {
            if line.starts_with("[OP_") && line.ends_with("]") {
                if let Some(mut opt) = current_option.take() {
                    opt.items = std::mem::take(&mut current_items);
                    options.push(opt);
                }
                let name = line[1..line.len() - 1].to_string();
                current_option = Some(OptionDefinition {
                    name,
                    byte_index: 0,
                    bits_start: 0,
                    bits_end: 0,
                    items: Vec::new(),
                    editable: false, // Will be set based on option_mask later
                });
                current_items.clear();
            } else if let Some(ref mut opt) = current_option {
                if line.starts_with("Byte ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        opt.byte_index = parts[1].parse().unwrap_or(0);
                    }
                } else if line.starts_with("Bits ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        opt.bits_start = parts[1].parse().unwrap_or(0);
                        opt.bits_end = parts[2].parse().unwrap_or(0);
                    }
                } else if line.starts_with("Items ") {
                    // Just the count, items follow
                } else if !line.is_empty() {
                    // Parse item line: "0	description"
                    let parts: Vec<&str> = line.splitn(2, '\t').collect();
                    if parts.len() == 2 {
                        if let Ok(value) = parts[0].parse::<u8>() {
                            current_items.push((value, parts[1].to_string()));
                        }
                    }
                }
            }
        } else if line.starts_with('[') && line.ends_with(']') {
            // Section header
        } else if !line.is_empty() && !line.starts_with('[') {
            // Value line - associate with the most recent section
        }
    }

    // Parse key-value pairs outside options section
    // Some fields have multiple lines (size + address), store as Vec
    let mut multi_fields: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: Option<String> = None;
    for line in content.lines() {
        let line = line.trim();
        if line == "------ Options Begin ------" {
            break;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current_key = Some(line[1..line.len() - 1].to_string());
        } else if !line.is_empty() {
            if let Some(ref key) = current_key {
                // Single-value fields go to fields map (first value only)
                if !fields.contains_key(key) {
                    fields.insert(key.clone(), line.to_string());
                }
                // All values go to multi_fields
                multi_fields.entry(key.clone()).or_default().push(line.to_string());
            }
        }
    }

    // Helper to parse address fields (region + hex address)
    let parse_address_field = |key: &str| -> Option<AddressField> {
        let values = multi_fields.get(key)?;
        if values.len() >= 2 {
            let region = values[0].parse::<u8>().ok()?;
            let addr_str = values[1].trim_start_matches("0x").trim_start_matches("0X");
            let address = u32::from_str_radix(addr_str, 16).ok()?;
            Some(AddressField { region, address })
        } else {
            None
        }
    };

    let chip_name = fields
        .get("ChipName")
        .cloned()
        .ok_or_else(|| GptError::Parse("Missing ChipName".to_string()))?;

    let part_number = fields.get("PartNumber").cloned().unwrap_or_default();

    let chip_type = fields
        .get("ChipType")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let custom_block = fields
        .get("CustomBlock")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let product_block = fields
        .get("ProductBlock")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let flash_size = fields
        .get("FlashSize")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let jtag_id = fields
        .get("JTAG ID")
        .and_then(|s| u16::from_str_radix(s, 16).ok())
        .unwrap_or(0);

    let sector_size = fields
        .get("SectorSize")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let external_ram = fields
        .get("ExternalRAM")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let eeprom = fields
        .get("EEPROM")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let initial_option: u64 = fields
        .get("InitialOption")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let option_mask: u64 = fields
        .get("OptionMask")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let option_byte_count = fields
        .get("OptionByteCount")
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    // Parse address fields
    let customer_id = parse_address_field("CustomerID");
    let operation_number = parse_address_field("OperationNumber");
    let customer_option = parse_address_field("CustomerOption");
    let security = parse_address_field("Security");
    let serial_number = parse_address_field("SerialNumber");

    // Determine editability of each option based on option_mask
    let option_mask_bytes = option_mask.to_le_bytes();
    for opt in &mut options {
        let bit_width = opt.bits_end - opt.bits_start + 1;
        let bits_mask = ((1u64 << bit_width) - 1) << opt.bits_start;
        let byte_mask = if opt.byte_index < 8 {
            option_mask_bytes[opt.byte_index] as u64
        } else {
            0
        };
        // Option is editable if all its bits are set in the mask
        opt.editable = (byte_mask & bits_mask) == bits_mask;
    }

    Ok(PartDefinition {
        chip_name,
        part_number,
        chip_type,
        custom_block,
        product_block,
        flash_size,
        jtag_id,
        sector_size,
        external_ram,
        eeprom,
        initial_option,
        option_mask,
        option_byte_count,
        customer_id,
        operation_number,
        customer_option,
        security,
        serial_number,
        options,
    })
}

fn generate_rust_part_definition(part: &PartDefinition) -> String {
    let part_number_bytes = format_part_number(&part.part_number);
    let default_code_options = format_initial_options(part.initial_option, part.option_byte_count);
    let code_option_mask = format_initial_options(part.option_mask, part.option_byte_count);

    let mut output = String::new();

    output.push_str(&format!("// Auto-generated from GPT file for {}\n\n", part.chip_name));
    output.push_str("use super::{AddressField, OptionInfo, Options, Part};\n");
    output.push_str("use hex_literal::hex;\n");
    output.push_str("use indexmap::IndexMap;\n\n");

    // Part constant
    output.push_str("pub const PART: Part = Part {\n");
    output.push_str(&format!("    part_number: hex!(\"{}\"),\n", part_number_bytes));
    output.push_str(&format!("    chip_type: 0x{:02x},\n", part.chip_type));
    output.push_str(&format!("    custom_block: 0x{:02x},\n", part.custom_block));
    output.push_str(&format!("    product_block: 0x{:02x},\n", part.product_block));
    output.push_str(&format!("    flash_size: {},\n", part.flash_size));
    output.push_str(&format!("    default_code_options: &hex!(\"{}\"),\n", default_code_options));
    output.push_str(&format!("    code_option_mask: &hex!(\"{}\"),\n", code_option_mask));
    output.push_str(&format!("    jtag_id: 0x{:04x},\n", part.jtag_id));
    output.push_str(&format!("    sector_size: {},\n", part.sector_size));
    output.push_str(&format!("    option_byte_count: {},\n", part.option_byte_count));

    // Address fields
    fn format_address_field(addr: &Option<AddressField>) -> String {
        match addr {
            Some(a) => format!("Some(AddressField {{ region: {}, address: 0x{:04x} }})", a.region, a.address),
            None => "None".to_string(),
        }
    }
    output.push_str(&format!("    customer_id: {},\n", format_address_field(&part.customer_id)));
    output.push_str(&format!("    operation_number: {},\n", format_address_field(&part.operation_number)));
    output.push_str(&format!("    customer_option: {},\n", format_address_field(&part.customer_option)));
    output.push_str(&format!("    security: {},\n", format_address_field(&part.security)));
    output.push_str(&format!("    serial_number: {},\n", format_address_field(&part.serial_number)));
    output.push_str("    options,\n");

    output.push_str("};\n\n");

    // Generate options function with all options
    if !part.options.is_empty() {
        output.push_str("/// Get all code options metadata\n");
        output.push_str("pub fn options() -> Options {\n");
        output.push_str("    IndexMap::from([\n");
        for opt in &part.options {
            let name = opt.name.trim_end_matches(':');
            output.push_str(&format!(
                "        (\"{}\", OptionInfo {{\n",
                name
            ));
            output.push_str(&format!(
                "            byte_index: {},\n",
                opt.byte_index
            ));
            output.push_str(&format!(
                "            bits_start: {},\n",
                opt.bits_start
            ));
            output.push_str(&format!(
                "            bits_end: {},\n",
                opt.bits_end
            ));
            output.push_str(&format!(
                "            editable: {},\n",
                opt.editable
            ));
            output.push_str("            states: IndexMap::from([\n");
            for (value, desc) in &opt.items {
                let escaped_desc = desc.replace('\\', "\\\\").replace('"', "\\\"");
                output.push_str(&format!(
                    "                ({}, \"{}\"),\n",
                    value, escaped_desc
                ));
            }
            output.push_str("            ]),\n");
            output.push_str("        }),\n");
        }
        output.push_str("    ])\n");
        output.push_str("}\n");
    }

    output
}

fn format_part_number(part_number: &str) -> String {
    // Pad to 10 hex chars (5 bytes)
    let padded = format!("{:0<10}", part_number.to_lowercase());
    padded
}

fn format_initial_options(initial_option: u64, byte_count: usize) -> String {
    let bytes = initial_option.to_le_bytes();
    bytes.iter().take(byte_count).map(|b| format!("{:02x}", b)).collect()
}

#[derive(Parser)]
#[command(name = "generate-part")]
#[command(about = "Decrypt GPT files and generate part definitions for Sinowealth microcontrollers")]
struct Cli {
    /// GPT file(s) to process
    #[arg(required = true)]
    files: Vec<String>,

    /// Output directory for generated part definitions
    #[arg(short, long, default_value = "parts")]
    output_dir: String,

    /// Also write decrypted GPT files
    #[arg(short, long)]
    decrypt_only: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create output directory
    if !cli.decrypt_only {
        fs::create_dir_all(&cli.output_dir)?;
    }

    for file_path in &cli.files {
        println!("Processing: {}", file_path);

        let path = Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?;

        // Read and decrypt
        let file_content = fs::read(file_path)?;
        let keys = keypair(filename)?;
        println!("  Keys: ({:#04x}, {:#04x})", keys.0, keys.1);

        let decrypted = decrypt(file_content.iter().copied(), keys)?;

        // Write decrypted file
        let decrypted_path = format!("{}.decrypted", file_path);
        fs::write(&decrypted_path, &decrypted)?;
        println!("  Decrypted: {}", decrypted_path);

        if cli.decrypt_only {
            continue;
        }

        // Parse and generate part definition
        let content = String::from_utf8_lossy(&decrypted);
        let part = parse_gpt_content(&content)?;

        println!("  Chip: {}", part.chip_name);
        println!("  Flash: {} bytes", part.flash_size);
        println!("  Sector: {} bytes", part.sector_size);
        println!("  JTAG ID: 0x{:04x}", part.jtag_id);
        if let Some(ref addr) = part.customer_id {
            println!("  Customer ID: region={}, 0x{:04X}", addr.region, addr.address);
        }
        if let Some(ref addr) = part.operation_number {
            println!("  Operation Number: region={}, 0x{:04X}", addr.region, addr.address);
        }
        if let Some(ref addr) = part.customer_option {
            println!("  Customer Option: region={}, 0x{:04X}", addr.region, addr.address);
        }
        if let Some(ref addr) = part.security {
            println!("  Security: region={}, 0x{:04X}", addr.region, addr.address);
        }
        if let Some(ref addr) = part.serial_number {
            println!("  Serial Number: region={}, 0x{:04X}", addr.region, addr.address);
        }

        let rust_code = generate_rust_part_definition(&part);
        let output_filename = format!(
            "{}/{}.rs",
            cli.output_dir,
            part.chip_name.to_lowercase().replace("-", "_")
        );
        fs::write(&output_filename, &rust_code)?;
        println!("  Generated: {}", output_filename);
    }

    println!("\nDone!");
    Ok(())
}
