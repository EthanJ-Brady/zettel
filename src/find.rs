use colored::Colorize;
use std::fs::{self, DirEntry};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

use crate::cli::FindArgs;

pub fn find(args: &FindArgs, dir: &str) -> Result<(), io::Error> {
    let search_string = &args.search_string;

    let paths = fs::read_dir(&dir).unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().into_string().unwrap();
        if file_name.contains(search_string) {
            println!("{}", get_file_print_string(&file_name));
        }
    }

    Ok(())
}

pub fn find_first(search_string: &str, dir: &str) -> DirEntry {
    let paths = fs::read_dir(&dir).unwrap();

    for path in paths {
        let file_name = path.as_ref().unwrap().file_name().into_string().unwrap();
        let file_name_and_hash = get_file_print_string(&file_name);
        if file_name_and_hash.contains(search_string) {
            return path.unwrap();
        }
    }
    panic!("Could not find file that matched {}", search_string)
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
