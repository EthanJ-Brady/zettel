mod cli;
mod find;
mod new;
mod open;

use clap::Parser;
use cli::Cli;
use cli::Commands;
use find::find;
use new::new;
use open::open;
use std::io;

static NOTES_DIR: &str = "./notes";

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::New(args) => new(&args, NOTES_DIR).expect("Failed to create file"),
        Commands::Find(args) => find(&args, NOTES_DIR).expect("Failed to find files"),
        Commands::Open(args) => open(&args, NOTES_DIR).expect("Failed to find file"),
        Commands::Append(_args) => dummy().expect("Failed to append"),
    }
}

fn dummy() -> Result<(), io::Error> {
    Ok(())
}
