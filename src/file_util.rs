use std::fs;
use log::debug;

pub fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    // debug!("content: {}", content);
    content
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}
