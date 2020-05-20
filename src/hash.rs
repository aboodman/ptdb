use std::fmt;
use sha2::{Sha512, Digest};

pub const BYTE_LENGTH: usize = 20;

// This is kind of lame but I don't want to re-implement base32, and the base32 package doesn't support pluggable libraries.
const CROCKFORD_ALPHABET: &'static [u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const NOMS_ALPHABET: &'static [u8] = b"0123456789abcdefghijklmnopqrstuv";

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
        let mut ss = String::with_capacity(s.len());
        for c in s.bytes() {
            let p = NOMS_ALPHABET.iter().position(|cc| *cc == c)?;
            ss.push(char::from(CROCKFORD_ALPHABET[p]));
        }
        
        let buf = base32::decode(base32::Alphabet::Crockford, ss.as_str())?;
        let mut h = Hash::empty();
        h.sum.copy_from_slice(buf.as_slice());
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

    pub fn is_empty(&self) -> bool {
        return self.sum == [0; BYTE_LENGTH];
    }
}

impl fmt::Display for Hash {
    // TODO: Is it idiomatic to do this in fmt, or should there be a to_string() too?
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Would be better to not allocate here since we know the size of the resulting string, but base32 package doesn't support.
        let mut str = base32::encode(base32::Alphabet::Crockford, &self.sum);

        // TODO: Unsafe :(
        unsafe {
            for c in str.as_mut_vec().iter_mut() {
                // TODO: Unwrap here is sad, but fmt::Result looks like it can't transmit an error message?
                let p = CROCKFORD_ALPHABET.iter().position(|cc| cc == c).unwrap();
                *c = NOMS_ALPHABET[p];
            }
        }
        write!(f, "{}", str)
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
