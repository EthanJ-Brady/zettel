use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The title of the new file being created
    pub title: String,

    /// The body of the new file being created
    #[arg(short = 'b', long = "body", default_value = "")]
    pub body: String,

    /// A tag to include in the file being created
    #[arg(short = 't', long = "tag")]
    pub tags: Option<Vec<String>>,

    /// A source to include in the file being created
    #[arg(short = 's', long = "short")]
    pub sources: Option<Vec<String>>,

    /// Whether or not to open the program in the default editor
    #[arg(short = 'o', long = "open", action)]
    pub open: bool,
}

#[derive(Args, Debug)]
pub struct FindArgs {
    /// The string to search for in the name of the file
    pub search_string: String,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    /// The string or hash found in the file you want to open
    pub search_string: String,
}

#[derive(Args, Debug)]
pub struct AppendArgs {}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a new file with a given title
    New(NewArgs),

    /// Finds a file with a given name
    Find(FindArgs),

    /// Opens the first file with a substring of its name or hash matching the search
    Open(OpenArgs),

    /// Appends the first file with a substring of its name or hash matching the search with
    /// details
    Append(AppendArgs),
}
