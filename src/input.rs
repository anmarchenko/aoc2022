use std::{fs, path::Path};

pub fn get_input(day: u8, prefix: &str) -> String {
    let filename = format!("./inputs/{:02}/{}.txt", day, prefix);
    let file_path = Path::new(&filename);
    fs::read_to_string(file_path).unwrap()
}
