mod cli;
mod new;

use clap::Parser;
use cli::Cli;
use cli::Commands;
use new::new;
use std::io;

static NOTES_DIR: &str = "./notes";

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::New(args) => {
            new(&args.title, &args.body, NOTES_DIR).expect("Failed to create file")
        }
        Commands::Find(_args) => dummy().expect("Failed to find"),
        Commands::Open(_args) => dummy().expect("Failed to open"),
        Commands::Append(_args) => dummy().expect("Failed to append"),
    }
}

fn dummy() -> Result<(), io::Error> {
    Ok(())
}
