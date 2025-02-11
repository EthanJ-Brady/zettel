use colored::Colorize;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

pub fn find(search_string: &str, dir: &str) -> Result<(), io::Error> {
    let paths = fs::read_dir(&dir).unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().into_string().unwrap();
        if file_name.contains(search_string) {
            println!("{}", get_file_print_string(&file_name));
        }
    }

    Ok(())
}

fn get_file_print_string(file_name: &str) -> String {
    let hash = (&calculate_hash(&file_name)[..7]).yellow();
    format!("{hash} {file_name}")
}

pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let decimal_num = s.finish();
    format!("{:x}", decimal_num)
}
