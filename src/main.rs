mod cli;

use clap::Parser;
use cli::Cli;
use cli::Commands;

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::New(args) => println!("{}", &args.title),
        Commands::Find(_args) => {}
        Commands::Open(_args) => {}
        Commands::Append(_args) => {}
    }
}
