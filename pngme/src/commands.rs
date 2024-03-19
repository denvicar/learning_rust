use std::fs;
use std::str::FromStr;
use crate::png::Png;
use crate::{png, chunk, chunk_type};

use crate::Result;

fn decode_png(file_path: &str) -> Result<png::Png> {
    let input = fs::read(file_path)?;
    Png::try_from(&input[..])
}

pub fn encode(file_path: String, chunk_type: String, msg: String, output: Option<String>) -> Result<()> {
    let mut png = decode_png(&file_path)?;
    let chunk = chunk::Chunk::new(chunk_type::ChunkType::from_str(&chunk_type)?, msg.into_bytes());
    png.append_chunk(chunk);

    if let Some(output_path) = output {
        fs::write(output_path, png.as_bytes())?;
    } else {
        fs::write(file_path, png.as_bytes())?;
    }

    Ok(())
}

pub fn decode(file_path: String, chunk_type: String) -> Result<String> {
    let png = decode_png(&file_path)?;
    let chunk = png.chunk_by_type(&chunk_type);

    if let Some(ch) = chunk {
        return ch.data_as_string();
    } else {
        return Err(Box::new(png::PngError { msg: "cannot decode given chunk_type" }));
    }
}

pub fn remove(file_path: String, chunk_type: String) -> Result<String> {
    let mut png = decode_png(&file_path)?;
    let chunk = png.remove_chunk(&chunk_type)?;
    fs::write(file_path, png.as_bytes())?;

    chunk.data_as_string()
}

pub fn print(file_path: String) {
    let png = decode_png(&file_path).expect("cannot read provided png file");
    println!("{}", png);
}
