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
struct NewArgs {
    title: String,
}

#[derive(Args, Debug)]
struct FindArgs {}

#[derive(Args, Debug)]
struct OpenArgs {}

#[derive(Args, Debug)]
struct AppendArgs {}

#[derive(Subcommand, Debug)]
enum Commands {
    New(NewArgs),
    Find(FindArgs),
    Open(OpenArgs),
    Append(AppendArgs),
}

impl Commands {
    fn message(&self) -> &str {
        match self {
            Self::New(args) => &args.title,
            Self::Find(_args) => "find",
            Self::Open(_args) => "open",
            Self::Append(_args) => "append",
        }
    }
}
