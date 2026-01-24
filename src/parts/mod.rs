use indexmap::IndexMap;
use phf::phf_map;

pub mod adc2015;
pub mod ch6935a;
pub mod cm7916;
pub mod gs16lf601;
pub mod gs16lf602a;
pub mod gs16lf603a;
pub mod gs16lf606;
pub mod gs16lf607;
pub mod gs16lf608;
pub mod gs16lf609a;
pub mod gs16lf611;
pub mod gs16lf612;
pub mod gs16lf614;
pub mod gs16lf615;
pub mod gs16lf616;
pub mod gs16lf617;
pub mod hea08f20;
pub mod hea08f28a;
pub mod hp60207;
pub mod hp60307;
pub mod hp60405;
pub mod hp60908;
pub mod jym0532;
pub mod jym0564;
pub mod mcf8014;
pub mod md001;
pub mod ps2024;
pub mod qf9408;
pub mod sh366002;
pub mod sh366006;
pub mod sh39f003;
pub mod sh39f003a;
pub mod sh39f004;
pub mod sh39f005;
pub mod sh39f323;
pub mod sh39f323a;
pub mod sh39f323c;
pub mod sh39f325;
pub mod sh39f325a;
pub mod sh39f325c;
pub mod sh39f3510;
pub mod sh39f5005;
pub mod sh39f5007;
pub mod sh68f091;
pub mod sh68f093;
pub mod sh68f093c;
pub mod sh68f097;
pub mod sh68f097b;
pub mod sh68f1000;
pub mod sh68f1001;
pub mod sh68f83;
pub mod sh68f86;
pub mod sh68f88;
pub mod sh68f881;
pub mod sh68f89;
pub mod sh68f90;
pub mod sh68f902;
pub mod sh68f902a;
pub mod sh68f903a;
pub mod sh68f90a;
pub mod sh68f91;
pub mod sh77p1651;
pub mod sh77p1652;
pub mod sh79e02;
pub mod sh79e021;
pub mod sh79f081;
pub mod sh79f0819;
pub mod sh79f0819a;
pub mod sh79f081a;
pub mod sh79f081b;
pub mod sh79f082;
pub mod sh79f083;
pub mod sh79f083a;
pub mod sh79f083b;
pub mod sh79f084a;
pub mod sh79f084b;
pub mod sh79f085;
pub mod sh79f086;
pub mod sh79f088;
pub mod sh79f088b;
pub mod sh79f089;
pub mod sh79f161;
pub mod sh79f1611;
pub mod sh79f1612;
pub mod sh79f1612a;
pub mod sh79f1612b;
pub mod sh79f1614;
pub mod sh79f1615;
pub mod sh79f1616;
pub mod sh79f1616b;
pub mod sh79f1617;
pub mod sh79f1617a;
pub mod sh79f1618;
pub mod sh79f1618a;
pub mod sh79f1619;
pub mod sh79f1619a;
pub mod sh79f1619b;
pub mod sh79f161a;
pub mod sh79f161b;
pub mod sh79f162;
pub mod sh79f1620;
pub mod sh79f1620b;
pub mod sh79f1621;
pub mod sh79f1621a;
pub mod sh79f1622;
pub mod sh79f1623;
pub mod sh79f1624a;
pub mod sh79f1624b;
pub mod sh79f1625;
pub mod sh79f1627;
pub mod sh79f1627a;
pub mod sh79f1628;
pub mod sh79f163;
pub mod sh79f1630;
pub mod sh79f1631;
pub mod sh79f1633;
pub mod sh79f164;
pub mod sh79f1640;
pub mod sh79f165;
pub mod sh79f166;
pub mod sh79f166a;
pub mod sh79f166b;
pub mod sh79f166c;
pub mod sh79f168;
pub mod sh79f169;
pub mod sh79f169b;
pub mod sh79f2201;
pub mod sh79f2202;
pub mod sh79f2202a;
pub mod sh79f2203;
pub mod sh79f2203a;
pub mod sh79f2204;
pub mod sh79f2206;
pub mod sh79f2206a;
pub mod sh79f2211;
pub mod sh79f2221;
pub mod sh79f2401;
pub mod sh79f2601;
pub mod sh79f2611;
pub mod sh79f32;
pub mod sh79f321;
pub mod sh79f3212;
pub mod sh79f3213;
pub mod sh79f3213a;
pub mod sh79f3214;
pub mod sh79f3215;
pub mod sh79f3218;
pub mod sh79f3221;
pub mod sh79f3252;
pub mod sh79f326;
pub mod sh79f326a;
pub mod sh79f328;
pub mod sh79f3281;
pub mod sh79f3281a;
pub mod sh79f3283;
pub mod sh79f3283a;
pub mod sh79f3284;
pub mod sh79f3285;
pub mod sh79f328a;
pub mod sh79f329;
pub mod sh79f329a;
pub mod sh79f64;
pub mod sh79f6412;
pub mod sh79f6413;
pub mod sh79f642;
pub mod sh79f6421;
pub mod sh79f6428;
pub mod sh79f6428a;
pub mod sh79f642b;
pub mod sh79f6431;
pub mod sh79f6432;
pub mod sh79f6433;
pub mod sh79f6436;
pub mod sh79f6441;
pub mod sh79f6442;
pub mod sh79f6461;
pub mod sh79f6470;
pub mod sh79f6481;
pub mod sh79f6481a;
pub mod sh79f6482;
pub mod sh79f6483;
pub mod sh79f6484;
pub mod sh79f6485;
pub mod sh79f6486;
pub mod sh79f6488;
pub mod sh79f6489;
pub mod sh79f649;
pub mod sh79f7010;
pub mod sh79f7011a;
pub mod sh79f7012;
pub mod sh79f7013a;
pub mod sh79f7015;
pub mod sh79f7016;
pub mod sh79f7017;
pub mod sh79f7019a;
pub mod sh79f7019f;
pub mod sh79f7021a;
pub mod sh79f7022;
pub mod sh79f7099;
pub mod sh79f7416;
pub mod sh79f9010;
pub mod sh79f9202;
pub mod sh79f9203;
pub mod sh79f9204;
pub mod sh79f9206;
pub mod sh79f9209;
pub mod sh79f9211;
pub mod sh79f9212;
pub mod sh79f9219;
pub mod sh79f9230;
pub mod sh79f9258;
pub mod sh79f9259;
pub mod sh79f9260;
pub mod sh79f9260a;
pub mod sh79f9261;
pub mod sh79f9262;
pub mod sh79f9263;
pub mod sh79f9263a;
pub mod sh79f9267;
pub mod sh79f9269;
pub mod sh79f9270;
pub mod sh79f9271;
pub mod sh79f9272;
pub mod sh79f9273;
pub mod sh79f9401;
pub mod sh79f9402;
pub mod sh79f9403;
pub mod sh79f9404;
pub mod sh79f9405;
pub mod sh79f9406;
pub mod sh79f9407;
pub mod sh79f9408;
pub mod sh79f9409;
pub mod sh79f9410;
pub mod sh79f9412;
pub mod sh79f9415;
pub mod sh79f9420;
pub mod sh79f9421;
pub mod sh79f9460;
pub mod sh79f9461;
pub mod sh79f9461a;
pub mod sh79f9462;
pub mod sh79f9462a;
pub mod sh79f9463;
pub mod sh79f9463a;
pub mod sh79f9468;
pub mod sh79f9476;
pub mod sh79f9601;
pub mod sh79f9602;
pub mod sh79f9603;
pub mod sh79f9604;
pub mod sh79f9608;
pub mod sh79f9611;
pub mod sh79f9612;
pub mod sh79f9660;
pub mod sh79f9661;
pub mod sh79f9661a;
pub mod sh79f9662;
pub mod sh79f9670;
pub mod sh79f9801;
pub mod sh79m081a;
pub mod sh79m083a;
pub mod sh79m083b;
pub mod sh79m084b;
pub mod sh79m1612b;
pub mod sh79m1634;
pub mod sh79m9266;
pub mod sh79m9280;
pub mod sh79m9413;
pub mod sh79m9464;
pub mod sh79m9606;
pub mod sh79m9607;
pub mod sh79m9607a;
pub mod sh79m9613;
pub mod sh86295;
pub mod sh86313;
pub mod sh86315;
pub mod sh86331;
pub mod sh86f6601;
pub mod sh86f7061;
pub mod sh86f7066;
pub mod sh86f7086;
pub mod sh86f7088;
pub mod sh87f8941;
pub mod sh88f2049;
pub mod sh88f2051;
pub mod sh88f2051a;
pub mod sh88f2051b;
pub mod sh88f4051;
pub mod sh88f4051a;
pub mod sh88f4051b;
pub mod sh88f48;
pub mod sh88f516;
pub mod sh88f54;
pub mod sh88f6161;
pub mod sh88f6162;
pub mod sh88f6163;
pub mod sh89f52;
pub mod sh99f01;
pub mod sh99f201;
pub mod sh99f201b;
pub mod sh99f221;
pub mod xa2000;

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
    pub eeprom_size: usize,
    pub default_code_options: &'static [u8],
    pub code_option_mask: &'static [u8],
    pub jtag_id: u16,
    pub sector_size: usize,
    pub option_byte_count: usize,
    pub security_level: u8,
    pub bank_type: u8,
    pub customer_id: AddressField,
    pub operation_number: AddressField,
    pub customer_option: AddressField,
    pub security: AddressField,
    pub serial_number: AddressField,
    pub options: fn() -> Options,
}

impl Part {
    /// Returns true if options are stored in flash memory.
    /// custom_block values 2, 3, 4 are NOT in flash; all others ARE in flash.
    pub fn options_in_flash(&self) -> bool {
        !matches!(self.custom_block, 2..=4)
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
            let size = self.serial_number.address as usize - self.security.address as usize;
            eprintln!(
                "Warning: exact security length for this part is unknown, using the full {} bytes",
                size
            );
            size
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
        for (i, byte) in upper.iter_mut().enumerate() {
            let idx = 4 + i;
            if idx < self.default_code_options.len() && idx < self.code_option_mask.len() {
                // Non-editable bits: defaults & !mask
                *byte = self.default_code_options[idx] & !self.code_option_mask[idx];
            }
        }

        Some(upper)
    }
}

pub static PARTS: phf::Map<&'static str, &'static Part> = phf_map! {
    "adc2015" => &adc2015::PART,
    "ch6935a" => &ch6935a::PART,
    "cm7916" => &cm7916::PART,
    "gs16lf601" => &gs16lf601::PART,
    "gs16lf602a" => &gs16lf602a::PART,
    "gs16lf603a" => &gs16lf603a::PART,
    "gs16lf606" => &gs16lf606::PART,
    "gs16lf607" => &gs16lf607::PART,
    "gs16lf608" => &gs16lf608::PART,
    "gs16lf609a" => &gs16lf609a::PART,
    "gs16lf611" => &gs16lf611::PART,
    "gs16lf612" => &gs16lf612::PART,
    "gs16lf614" => &gs16lf614::PART,
    "gs16lf615" => &gs16lf615::PART,
    "gs16lf616" => &gs16lf616::PART,
    "gs16lf617" => &gs16lf617::PART,
    "hea08f20" => &hea08f20::PART,
    "hea08f28a" => &hea08f28a::PART,
    "hp60207" => &hp60207::PART,
    "hp60307" => &hp60307::PART,
    "hp60405" => &hp60405::PART,
    "hp60908" => &hp60908::PART,
    "jym0532" => &jym0532::PART,
    "jym0564" => &jym0564::PART,
    "mcf8014" => &mcf8014::PART,
    "md001" => &md001::PART,
    "ps2024" => &ps2024::PART,
    "qf9408" => &qf9408::PART,
    "sh366002" => &sh366002::PART,
    "sh366006" => &sh366006::PART,
    "sh39f003" => &sh39f003::PART,
    "sh39f003a" => &sh39f003a::PART,
    "sh39f004" => &sh39f004::PART,
    "sh39f005" => &sh39f005::PART,
    "sh39f323" => &sh39f323::PART,
    "sh39f323a" => &sh39f323a::PART,
    "sh39f323c" => &sh39f323c::PART,
    "sh39f325" => &sh39f325::PART,
    "sh39f325a" => &sh39f325a::PART,
    "sh39f325c" => &sh39f325c::PART,
    "sh39f3510" => &sh39f3510::PART,
    "sh39f5005" => &sh39f5005::PART,
    "sh39f5007" => &sh39f5007::PART,
    "sh68f091" => &sh68f091::PART,
    "sh68f093" => &sh68f093::PART,
    "sh68f093c" => &sh68f093c::PART,
    "sh68f097" => &sh68f097::PART,
    "sh68f097b" => &sh68f097b::PART,
    "sh68f1000" => &sh68f1000::PART,
    "sh68f1001" => &sh68f1001::PART,
    "sh68f83" => &sh68f83::PART,
    "sh68f86" => &sh68f86::PART,
    "sh68f88" => &sh68f88::PART,
    "sh68f881" => &sh68f881::PART,
    "sh68f89" => &sh68f89::PART,
    "sh68f90" => &sh68f90::PART,
    "sh68f90a" => &sh68f90a::PART,
    "sh68f902" => &sh68f902::PART,
    "sh68f902a" => &sh68f902a::PART,
    "sh68f903a" => &sh68f903a::PART,
    "sh68f91" => &sh68f91::PART,
    "sh77p1651" => &sh77p1651::PART,
    "sh77p1652" => &sh77p1652::PART,
    "sh79e02" => &sh79e02::PART,
    "sh79e021" => &sh79e021::PART,
    "sh79f081" => &sh79f081::PART,
    "sh79f0819" => &sh79f0819::PART,
    "sh79f0819a" => &sh79f0819a::PART,
    "sh79f081a" => &sh79f081a::PART,
    "sh79f081b" => &sh79f081b::PART,
    "sh79f082" => &sh79f082::PART,
    "sh79f083" => &sh79f083::PART,
    "sh79f083a" => &sh79f083a::PART,
    "sh79f083b" => &sh79f083b::PART,
    "sh79f084a" => &sh79f084a::PART,
    "sh79f084b" => &sh79f084b::PART,
    "sh79f085" => &sh79f085::PART,
    "sh79f086" => &sh79f086::PART,
    "sh79f088" => &sh79f088::PART,
    "sh79f088b" => &sh79f088b::PART,
    "sh79f089" => &sh79f089::PART,
    "sh79f161" => &sh79f161::PART,
    "sh79f1611" => &sh79f1611::PART,
    "sh79f1612" => &sh79f1612::PART,
    "sh79f1612a" => &sh79f1612a::PART,
    "sh79f1612b" => &sh79f1612b::PART,
    "sh79f1614" => &sh79f1614::PART,
    "sh79f1615" => &sh79f1615::PART,
    "sh79f1616" => &sh79f1616::PART,
    "sh79f1616b" => &sh79f1616b::PART,
    "sh79f1617" => &sh79f1617::PART,
    "sh79f1617a" => &sh79f1617a::PART,
    "sh79f1618" => &sh79f1618::PART,
    "sh79f1618a" => &sh79f1618a::PART,
    "sh79f1619" => &sh79f1619::PART,
    "sh79f1619a" => &sh79f1619a::PART,
    "sh79f1619b" => &sh79f1619b::PART,
    "sh79f161a" => &sh79f161a::PART,
    "sh79f161b" => &sh79f161b::PART,
    "sh79f162" => &sh79f162::PART,
    "sh79f1620" => &sh79f1620::PART,
    "sh79f1620b" => &sh79f1620b::PART,
    "sh79f1621" => &sh79f1621::PART,
    "sh79f1621a" => &sh79f1621a::PART,
    "sh79f1622" => &sh79f1622::PART,
    "sh79f1623" => &sh79f1623::PART,
    "sh79f1624a" => &sh79f1624a::PART,
    "sh79f1624b" => &sh79f1624b::PART,
    "sh79f1625" => &sh79f1625::PART,
    "sh79f1627" => &sh79f1627::PART,
    "sh79f1627a" => &sh79f1627a::PART,
    "sh79f1628" => &sh79f1628::PART,
    "sh79f163" => &sh79f163::PART,
    "sh79f1630" => &sh79f1630::PART,
    "sh79f1631" => &sh79f1631::PART,
    "sh79f1633" => &sh79f1633::PART,
    "sh79f164" => &sh79f164::PART,
    "sh79f1640" => &sh79f1640::PART,
    "sh79f165" => &sh79f165::PART,
    "sh79f166" => &sh79f166::PART,
    "sh79f166a" => &sh79f166a::PART,
    "sh79f166b" => &sh79f166b::PART,
    "sh79f166c" => &sh79f166c::PART,
    "sh79f168" => &sh79f168::PART,
    "sh79f169" => &sh79f169::PART,
    "sh79f169b" => &sh79f169b::PART,
    "sh79f2201" => &sh79f2201::PART,
    "sh79f2202" => &sh79f2202::PART,
    "sh79f2202a" => &sh79f2202a::PART,
    "sh79f2203" => &sh79f2203::PART,
    "sh79f2203a" => &sh79f2203a::PART,
    "sh79f2204" => &sh79f2204::PART,
    "sh79f2206" => &sh79f2206::PART,
    "sh79f2206a" => &sh79f2206a::PART,
    "sh79f2211" => &sh79f2211::PART,
    "sh79f2221" => &sh79f2221::PART,
    "sh79f2401" => &sh79f2401::PART,
    "sh79f2601" => &sh79f2601::PART,
    "sh79f2611" => &sh79f2611::PART,
    "sh79f32" => &sh79f32::PART,
    "sh79f321" => &sh79f321::PART,
    "sh79f3212" => &sh79f3212::PART,
    "sh79f3213" => &sh79f3213::PART,
    "sh79f3213a" => &sh79f3213a::PART,
    "sh79f3214" => &sh79f3214::PART,
    "sh79f3215" => &sh79f3215::PART,
    "sh79f3218" => &sh79f3218::PART,
    "sh79f3221" => &sh79f3221::PART,
    "sh79f3252" => &sh79f3252::PART,
    "sh79f326" => &sh79f326::PART,
    "sh79f326a" => &sh79f326a::PART,
    "sh79f328" => &sh79f328::PART,
    "sh79f3281" => &sh79f3281::PART,
    "sh79f3281a" => &sh79f3281a::PART,
    "sh79f3283" => &sh79f3283::PART,
    "sh79f3283a" => &sh79f3283a::PART,
    "sh79f3284" => &sh79f3284::PART,
    "sh79f3285" => &sh79f3285::PART,
    "sh79f328a" => &sh79f328a::PART,
    "sh79f329" => &sh79f329::PART,
    "sh79f329a" => &sh79f329a::PART,
    "sh79f64" => &sh79f64::PART,
    "sh79f6412" => &sh79f6412::PART,
    "sh79f6413" => &sh79f6413::PART,
    "sh79f642" => &sh79f642::PART,
    "sh79f6421" => &sh79f6421::PART,
    "sh79f6428" => &sh79f6428::PART,
    "sh79f6428a" => &sh79f6428a::PART,
    "sh79f642b" => &sh79f642b::PART,
    "sh79f6431" => &sh79f6431::PART,
    "sh79f6432" => &sh79f6432::PART,
    "sh79f6433" => &sh79f6433::PART,
    "sh79f6436" => &sh79f6436::PART,
    "sh79f6441" => &sh79f6441::PART,
    "sh79f6442" => &sh79f6442::PART,
    "sh79f6461" => &sh79f6461::PART,
    "sh79f6470" => &sh79f6470::PART,
    "sh79f6481" => &sh79f6481::PART,
    "sh79f6481a" => &sh79f6481a::PART,
    "sh79f6482" => &sh79f6482::PART,
    "sh79f6483" => &sh79f6483::PART,
    "sh79f6484" => &sh79f6484::PART,
    "sh79f6485" => &sh79f6485::PART,
    "sh79f6486" => &sh79f6486::PART,
    "sh79f6488" => &sh79f6488::PART,
    "sh79f6489" => &sh79f6489::PART,
    "sh79f649" => &sh79f649::PART,
    "sh79f7010" => &sh79f7010::PART,
    "sh79f7011a" => &sh79f7011a::PART,
    "sh79f7012" => &sh79f7012::PART,
    "sh79f7013a" => &sh79f7013a::PART,
    "sh79f7015" => &sh79f7015::PART,
    "sh79f7016" => &sh79f7016::PART,
    "sh79f7017" => &sh79f7017::PART,
    "sh79f7019a" => &sh79f7019a::PART,
    "sh79f7019f" => &sh79f7019f::PART,
    "sh79f7021a" => &sh79f7021a::PART,
    "sh79f7022" => &sh79f7022::PART,
    "sh79f7099" => &sh79f7099::PART,
    "sh79f7416" => &sh79f7416::PART,
    "sh79f9010" => &sh79f9010::PART,
    "sh79f9202" => &sh79f9202::PART,
    "sh79f9203" => &sh79f9203::PART,
    "sh79f9204" => &sh79f9204::PART,
    "sh79f9206" => &sh79f9206::PART,
    "sh79f9209" => &sh79f9209::PART,
    "sh79f9211" => &sh79f9211::PART,
    "sh79f9212" => &sh79f9212::PART,
    "sh79f9219" => &sh79f9219::PART,
    "sh79f9230" => &sh79f9230::PART,
    "sh79f9258" => &sh79f9258::PART,
    "sh79f9259" => &sh79f9259::PART,
    "sh79f9260" => &sh79f9260::PART,
    "sh79f9260a" => &sh79f9260a::PART,
    "sh79f9261" => &sh79f9261::PART,
    "sh79f9262" => &sh79f9262::PART,
    "sh79f9263" => &sh79f9263::PART,
    "sh79f9263a" => &sh79f9263a::PART,
    "sh79f9267" => &sh79f9267::PART,
    "sh79f9269" => &sh79f9269::PART,
    "sh79f9270" => &sh79f9270::PART,
    "sh79f9271" => &sh79f9271::PART,
    "sh79f9272" => &sh79f9272::PART,
    "sh79f9273" => &sh79f9273::PART,
    "sh79f9401" => &sh79f9401::PART,
    "sh79f9402" => &sh79f9402::PART,
    "sh79f9403" => &sh79f9403::PART,
    "sh79f9404" => &sh79f9404::PART,
    "sh79f9405" => &sh79f9405::PART,
    "sh79f9406" => &sh79f9406::PART,
    "sh79f9407" => &sh79f9407::PART,
    "sh79f9408" => &sh79f9408::PART,
    "sh79f9409" => &sh79f9409::PART,
    "sh79f9410" => &sh79f9410::PART,
    "sh79f9412" => &sh79f9412::PART,
    "sh79f9415" => &sh79f9415::PART,
    "sh79f9420" => &sh79f9420::PART,
    "sh79f9421" => &sh79f9421::PART,
    "sh79f9460" => &sh79f9460::PART,
    "sh79f9461" => &sh79f9461::PART,
    "sh79f9461a" => &sh79f9461a::PART,
    "sh79f9462" => &sh79f9462::PART,
    "sh79f9462a" => &sh79f9462a::PART,
    "sh79f9463" => &sh79f9463::PART,
    "sh79f9463a" => &sh79f9463a::PART,
    "sh79f9468" => &sh79f9468::PART,
    "sh79f9476" => &sh79f9476::PART,
    "sh79f9601" => &sh79f9601::PART,
    "sh79f9602" => &sh79f9602::PART,
    "sh79f9603" => &sh79f9603::PART,
    "sh79f9604" => &sh79f9604::PART,
    "sh79f9608" => &sh79f9608::PART,
    "sh79f9611" => &sh79f9611::PART,
    "sh79f9612" => &sh79f9612::PART,
    "sh79f9660" => &sh79f9660::PART,
    "sh79f9661" => &sh79f9661::PART,
    "sh79f9661a" => &sh79f9661a::PART,
    "sh79f9662" => &sh79f9662::PART,
    "sh79f9670" => &sh79f9670::PART,
    "sh79f9801" => &sh79f9801::PART,
    "sh79m081a" => &sh79m081a::PART,
    "sh79m083a" => &sh79m083a::PART,
    "sh79m083b" => &sh79m083b::PART,
    "sh79m084b" => &sh79m084b::PART,
    "sh79m1612b" => &sh79m1612b::PART,
    "sh79m1634" => &sh79m1634::PART,
    "sh79m9266" => &sh79m9266::PART,
    "sh79m9280" => &sh79m9280::PART,
    "sh79m9413" => &sh79m9413::PART,
    "sh79m9464" => &sh79m9464::PART,
    "sh79m9606" => &sh79m9606::PART,
    "sh79m9607" => &sh79m9607::PART,
    "sh79m9607a" => &sh79m9607a::PART,
    "sh79m9613" => &sh79m9613::PART,
    "sh86295" => &sh86295::PART,
    "sh86313" => &sh86313::PART,
    "sh86315" => &sh86315::PART,
    "sh86331" => &sh86331::PART,
    "sh86f6601" => &sh86f6601::PART,
    "sh86f7061" => &sh86f7061::PART,
    "sh86f7066" => &sh86f7066::PART,
    "sh86f7086" => &sh86f7086::PART,
    "sh86f7088" => &sh86f7088::PART,
    "sh87f8941" => &sh87f8941::PART,
    "sh88f2049" => &sh88f2049::PART,
    "sh88f2051" => &sh88f2051::PART,
    "sh88f2051a" => &sh88f2051a::PART,
    "sh88f2051b" => &sh88f2051b::PART,
    "sh88f4051" => &sh88f4051::PART,
    "sh88f4051a" => &sh88f4051a::PART,
    "sh88f4051b" => &sh88f4051b::PART,
    "sh88f48" => &sh88f48::PART,
    "sh88f516" => &sh88f516::PART,
    "sh88f54" => &sh88f54::PART,
    "sh88f6161" => &sh88f6161::PART,
    "sh88f6162" => &sh88f6162::PART,
    "sh88f6163" => &sh88f6163::PART,
    "sh89f52" => &sh89f52::PART,
    "sh99f01" => &sh99f01::PART,
    "sh99f201" => &sh99f201::PART,
    "sh99f201b" => &sh99f201b::PART,
    "sh99f221" => &sh99f221::PART,
    "xa2000" => &xa2000::PART,
};

/// Find all part names that match a given JTAG ID
pub fn find_parts_by_jtag_id(jtag_id: u16) -> Vec<&'static str> {
    PARTS
        .entries()
        .filter(|(_, part)| part.jtag_id == jtag_id)
        .map(|(name, _)| *name)
        .collect()
}

/// Find all part names that match a given part number
pub fn find_parts_by_part_number(part_number: &[u8; 5]) -> Vec<&'static str> {
    PARTS
        .entries()
        .filter(|(_, part)| &part.part_number == part_number)
        .map(|(name, _)| *name)
        .collect()
}
