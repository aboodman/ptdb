use std::fmt;

use data_encoding::decode;
use data_encoding::encode;
use data_encoding::base;
use sha2::{Sha512, Digest};

pub const BYTE_LENGTH: usize = 20;
const NOMS_ALPHABET: &'static [u8] = b"0123456789abcdefghijklmnopqrstuv";

struct Base32 {}

impl base::Base for Base32 {
    fn pad(&self) -> u8 {
        '=' as u8
    }
    fn val(&self, x: u8) -> Option<u8> {
        Some(NOMS_ALPHABET.iter().position(|y| x == *y)? as u8)
    }
}


pub struct Hash {
    pub sum: [u8; BYTE_LENGTH]
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Error {
    InvalidHashSerialization
}

impl Hash {
    pub fn empty() -> Hash {
        Hash{sum: [0; BYTE_LENGTH]}
    }

    pub fn new(sum: [u8; BYTE_LENGTH]) -> Hash {
        Hash{sum}
    }

    pub fn parse(s: &str) -> Result<Hash, Error> {
        let mut h = Hash::empty();
        match decode::decode_mut(&Base32{}, s.as_bytes(), &mut h.sum) {
            Err(_) => Err(Error::InvalidHashSerialization),
            Ok(_) => Ok(h),
        }
    }

    pub fn of(data: &[u8]) -> Hash {
        let mut hasher = Sha512::new();
        hasher.input(data);
        let result = hasher.result();
        let mut h = Hash::empty();
        h.sum.copy_from_slice(&result[..BYTE_LENGTH]);
        return h;
    }

    pub fn is_empty(&self) -> bool {
        return self.sum == [0; BYTE_LENGTH];
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: encode_mut will write to a buffer, can we avoid the
        // heap allocated string somehow that way?
        let s = encode::encode(&Base32{}, &self.sum);
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of() {
        let mut h = Hash::empty();
        assert_eq!(h.is_empty(), true);
        assert_eq!(format!("{}", h), "00000000000000000000000000000000");

        h = Hash::of(b"abc");
        assert_eq!(h.is_empty(), false);
        assert_eq!(format!("{}", h), "rmnjb8cjc5tblj21ed4qs821649eduie");

        let h2 = Hash::parse("rmnjb8cjc5tblj21ed4qs821649eduie").unwrap();
        assert_eq!(h2.to_string(), h.to_string());
    }
}
