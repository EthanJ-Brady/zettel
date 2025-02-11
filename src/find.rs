use std::fs;
use std::io;

pub fn find(search_string: &str, dir: &str) -> Result<(), io::Error> {
    let paths = fs::read_dir(&dir).unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().into_string().unwrap();
        if file_name.contains(search_string) {
            println!("{}", file_name)
        }
    }

    Ok(())
}
