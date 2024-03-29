#![allow(unused_variables, dead_code)]
use crate::{Error, Result};
use core::fmt;
use std::convert;

fn is_in_valid_range(b: u8) -> bool {
    (b >= 65 && b <= 90) || (b >= 97 && b <= 122)
}

fn is_fifth_bit_set(b: u8) -> bool {
    (b & 32) == 32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType {
    repr: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.repr
    }

    pub fn is_valid(&self) -> bool {
        // to be valid each byte has to be between 65-90 and 97-122
        // also bit 5 of byte 3 has to be set to 0 (reserved bit)
        for (i, byte) in self.repr.into_iter().enumerate() {
            if !is_in_valid_range(byte) {
                return false;
            }
            if i == 2 && is_fifth_bit_set(byte) {
                return false;
            }
        }

        true
    }

    pub fn is_critical(&self) -> bool {
        !is_fifth_bit_set(self.repr[0])
    }

    pub fn is_public(&self) -> bool {
        !is_fifth_bit_set(self.repr[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        !is_fifth_bit_set(self.repr[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        is_fifth_bit_set(self.repr[3])
    }
}

#[derive(Debug)]
struct ChunkTypeError;

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to process chunk type")
    }
}

impl std::error::Error for ChunkTypeError {}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes: [u8; 4] = s.as_bytes().try_into()?;
        for b in bytes {
            if !is_in_valid_range(b) {
                return Err(Box::new(ChunkTypeError));
            }
        }
        Ok(ChunkType { repr: bytes })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.repr.as_slice()).unwrap())
    }
}

impl convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(ChunkType { repr: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
