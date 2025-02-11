use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    pub title: String,
    #[arg(short = 'b', long = "body", default_value = "")]
    pub body: String,
}

#[derive(Args, Debug)]
pub struct FindArgs {}

#[derive(Args, Debug)]
pub struct OpenArgs {}

#[derive(Args, Debug)]
pub struct AppendArgs {}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New(NewArgs),
    Find(FindArgs),
    Open(OpenArgs),
    Append(AppendArgs),
}
