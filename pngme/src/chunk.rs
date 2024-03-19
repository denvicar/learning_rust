#![allow(unused_variables, dead_code)]
use std::fmt;

use crate::{chunk_type::ChunkType, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

const PNG32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    length: u32,
    crc: u32,
    data: Vec<u8>,
}

#[derive(Debug)]
struct ChunkError;

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to process chunk")
    }
}

impl std::error::Error for ChunkError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut value_iter = value.iter();
        let mut bytes: Vec<u8> = value_iter.by_ref().take(4).map(|&b| b).collect();
        let length = u32::from_be_bytes(bytes[..].try_into()?);
        bytes = value_iter.by_ref().take(4).map(|&b| b).collect();

        let bytes: [u8; 4] = bytes[..].try_into()?;
        let chunk_type = ChunkType::try_from(bytes)?;
        if !chunk_type.is_valid() {
            return Err(Box::new(ChunkError));
        }

        let data: Vec<_> = value_iter.map(|&v| v).collect();
        if data.len() < 4 {
            return Err(Box::new(ChunkError));
        }

        let (data, crc) = data.split_at(data.len() - 4);
        if data.len() as u32 != length {
            return Err(Box::new(ChunkError));
        }
        let crc = u32::from_be_bytes(crc.try_into()?);
        let mut digest = PNG32.digest();
        digest.update(&[chunk_type.bytes().as_slice(), data].concat());

        if digest.finalize() != crc {
            return Err(Box::new(ChunkError));
        }

        Ok(Chunk {
            data: data.to_vec(),
            length,
            crc,
            chunk_type,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Length {} Type {} CRC {}",
            self.length, self.chunk_type, self.crc
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut digest = PNG32.digest();
        digest.update(&[chunk_type.bytes().as_slice(), &data].concat());
        Chunk {
            crc: digest.finalize(),
            length: data.len() as u32,
            chunk_type,
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
