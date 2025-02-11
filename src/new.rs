use chrono::Local;
use std::fs::File;
use std::io::prelude::*;

pub fn new(title: &str, path: &str) -> std::io::Result<()> {
    let file_name = get_file_name(title);
    let file_path = format!("{path}/{file_name}");
    let mut file = File::create(file_path)?;

    let header = title;
    let file_text = format!("# {header}");
    write!(file, "{}", file_text)?;

    Ok(())
}

fn get_file_name(title: &str) -> String {
    let file_name = format!("{title}.md");
    let file_name = file_name.replace(" ", "-");
    let file_name = file_name.trim().to_lowercase();
    let zettel_date = get_zettel_datetime();
    format!("{zettel_date}-{file_name}")
}

fn get_zettel_datetime() -> String {
    let date = Local::now();
    date.format("%Y%m%d%H%M").to_string()
}
