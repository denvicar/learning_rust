use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// encode a given message as a new chunk in the file specified
    Encode { file_path: String, chunk_type: String, message: String, output: Option<String> },
    /// decodes the message in the given chunk for the specified file
    Decode { file_path: String, chunk_type: String },
    /// removes the given chunk from the specified file
    Remove { file_path: String, chunk_type: String },
    /// prints all chunks
    Print { file_path: String }
}
