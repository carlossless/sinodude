use std::path::Path;

pub type KeyPair = (u8, u8);
const HEADER_SIZE: usize = 17;

pub struct Decryptor {}

impl Decryptor {
    pub fn decrypt<'a>(mut input: impl Iterator<Item = u8>, keys: KeyPair) -> Vec<u8> {
        let header: Vec<u8> = input.by_ref().take(HEADER_SIZE).collect();
        assert_eq!(header, b"[Version]\r\n3.00\r\n");

        let mut result: Vec<u8> = vec![];

        for x in input {
            let mut num: u16 = x as u16;
            if num < keys.0 as u16 {
                num = num + 256;
            }
            let partial = (num - keys.0 as u16) as u8;
            result.push(partial ^ keys.1);
        }

        assert_eq!(&result[HEADER_SIZE..(HEADER_SIZE + 10)], b"[ChipName]");

        return result;
    }

    pub fn keypair(filename: &str) -> KeyPair {
        let stem = Path::new(filename).file_stem().unwrap();
        let len = stem.len();
        let key1 = Self::parse_key(&filename[len - 2..len]);
        let key2 = Self::parse_key(&filename[len - 4..len - 2]);
        return (key1, key2);
    }

    fn parse_key(str: &str) -> u8 {
        return u8::from_str_radix(str, 16).unwrap();
    }
}
