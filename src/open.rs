use crate::find::find_first;
use std::io;
use std::{env::var, fs::File, io::Read, process::Command};

pub fn open(search_string: &str, dir: &str) -> Result<(), io::Error> {
    let dir_entry = find_first(search_string, dir);

    let editor = var("EDITOR").unwrap();
    Command::new(editor)
        .arg(&dir_entry.path())
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();
    File::open(&dir_entry.path())
        .expect("Could not open file")
        .read_to_string(&mut editable)?;

    Ok(())
}
