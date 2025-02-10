use std::fs::File;
use std::io::prelude::*;

pub fn new(title: &str) -> std::io::Result<()> {
    let file_name = format!("{title}.md");
    let mut file = File::create(file_name)?;

    let file_text = format!("# {title}");
    write!(file, "{}", file_text)?;

    Ok(())
}
