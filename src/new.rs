use chrono::Local;
use std::fs::File;
use std::io::prelude::*;

pub fn new(title: &str) -> std::io::Result<()> {
    let file_name = format!("{title}.md");
    let file_name = file_name.replace(" ", "-");
    let file_name = file_name.trim().to_lowercase();
    let zettel_date = get_zettel_datetime();
    let file_name = format!("{zettel_date}-{file_name}");
    let mut file = File::create(file_name)?;

    let header = title;
    let file_text = format!("# {header}");
    write!(file, "{}", file_text)?;

    Ok(())
}

fn get_zettel_datetime() -> String {
    let date = Local::now();
    date.format("%Y%m%d%H%M").to_string()
}
