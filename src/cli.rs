use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn message(&self) -> &str {
        self.command.message()
    }
}

#[derive(Args, Debug)]
struct NewArgs {}

#[derive(Args, Debug)]
struct FindArgs {}

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewArgs),
    Find(FindArgs),
}

impl Commands {
    fn message(&self) -> &str {
        match self {
            Self::New(_args) => "new",
            Self::Find(_args) => "find",
        }
    }
}
