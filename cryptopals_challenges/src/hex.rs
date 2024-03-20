use base64::{prelude::BASE64_STANDARD, Engine};
use hex;
use crate::{Result};

pub fn hex_to_base64(hex_string: &str) -> Result<String> {
    let bytes = hex::decode(hex_string)?;
    Ok(BASE64_STANDARD.encode(bytes))
}

pub fn fixed_xor(hex_1: &str, hex_2: &str) -> Result<String> {
    let bytes_1 = hex::decode(hex_1)?;
    let bytes_2 = hex::decode(hex_2)?;

    if bytes_1.len() != bytes_2.len() {
        return Err(Box::new(std::fmt::Error));
    }

    let res : Vec<_>= bytes_1.iter()
        .zip(bytes_2.iter())
        .map(|(byte1, byte2)| *byte1 ^ *byte2)
        .collect();

    Ok(hex::encode(res))
}

#[cfg(test)]
mod tests {
    use crate::hex::{fixed_xor, hex_to_base64};
    #[test]
    pub fn test_conversion() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(hex_to_base64(hex_string).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    pub fn test_conversion_error() {
        let hex_string = "497d20gb";
        let res = hex_to_base64(hex_string);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_fixed_xor() {
        let hex_1 = "1c0111001f010100061a024b53535009181c";
        let hex_2 = "686974207468652062756c6c277320657965";

        assert_eq!(fixed_xor(hex_1, hex_2).unwrap(), "746865206b696420646f6e277420706c6179");
    }

    #[test]
    pub fn test_fixed_xor_invalid() {
        let hex_1 = "1g0111001f010100061a024b53535009181c";
        let hex_2 = "686974207468652062756c6c277320657965";
        let res = fixed_xor(hex_1, hex_2);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_fixed_xor_diff_len() {
        let hex_1 = "1111001f010100061a024b53535009181c";
        let hex_2 = "686974207468652062756c6c277320657965";
        let res = fixed_xor(hex_1, hex_2);
        assert!(res.is_err());
    }

    #[test]
    pub fn test_single_xor_cypher() {
        let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        println!("{}",String::from_utf8(hex::decode(hex_string).unwrap()).unwrap())
    }
}