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
    #[arg(short = 'o', long = "open", action)]
    pub open: bool,
}

#[derive(Args, Debug)]
pub struct FindArgs {
    pub search_string: String,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    pub search_string: String,
}

#[derive(Args, Debug)]
pub struct AppendArgs {}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New(NewArgs),
    Find(FindArgs),
    Open(OpenArgs),
    Append(AppendArgs),
}
