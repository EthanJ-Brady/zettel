use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

pub fn find(search_string: &str, dir: &str) -> Result<(), io::Error> {
    let paths = fs::read_dir(&dir).unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().into_string().unwrap();
        if file_name.contains(search_string) {
            println!("{} {}", &calculate_hash(&file_name)[..7], file_name)
        }
    }

    Ok(())
}

pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let decimal_num = s.finish();
    format!("{:x}", decimal_num)
}
