use clap::Parser;
use commands::{decode, encode, print, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Args::parse();

    match args.command {
        args::Commands::Encode { file_path, chunk_type, message, output } => {
            encode(file_path, chunk_type, message, output).expect("cannot encode given message");
            println!("encoded correctly");
        }
        args::Commands::Decode { file_path, chunk_type } => {
            let msg = decode(file_path, chunk_type)?;
            println!("decoded message: {}", msg);
        }
        args::Commands::Remove { file_path, chunk_type } => {
            remove(file_path, chunk_type).expect("cannot remove chunk specified");
            println!("chunk removed successfully");
        }
        args::Commands::Print { file_path } => {
            print(file_path);
        }
    }

    Ok(())
}
