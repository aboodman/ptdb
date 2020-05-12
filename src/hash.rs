use std::fmt;
use sha2::{Sha512, Digest};

const BYTE_LENGTH: usize = 20;

pub struct Hash {
    sum: [u8; BYTE_LENGTH]
}

impl Hash {
    pub fn empty() -> Hash {
        Hash{sum: [0; BYTE_LENGTH]}
    }

    pub fn new(sum: [u8; BYTE_LENGTH]) -> Hash {
        Hash{sum}
    }

    pub fn parse(s: &str) -> Option<Hash> {
        let buf = base32::decode(base32::Alphabet::Crockford, s)?;
        let mut h = Hash::empty();
        h.sum.copy_from_slice(&buf);
        Some(h)
    }

    pub fn of(data: &[u8]) -> Hash {
        let mut hasher = Sha512::new();
        hasher.input(data);
        let result = hasher.result();
        let mut h = Hash::empty();
        h.sum.copy_from_slice(&result[..BYTE_LENGTH]);
        return h;
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "foo")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_of() {
        let h = Hash::of(b"abc");
        print!("{}", h);
    }
}
