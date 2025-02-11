use crate::cli::OpenArgs;
use crate::find::find_first;
use std::io;
use std::path::PathBuf;
use std::{env::var, fs::File, io::Read, process::Command};

pub fn open(args: &OpenArgs, dir: &str) -> Result<(), io::Error> {
    let search_string = &args.search_string;

    let dir_entry = find_first(search_string, dir);
    open_file(dir_entry.path())
}

pub fn open_file(path: PathBuf) -> Result<(), io::Error> {
    let editor = var("EDITOR").unwrap();
    Command::new(editor)
        .arg(&path)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();
    File::open(&path)
        .expect("Could not open file")
        .read_to_string(&mut editable)?;

    Ok(())
}
