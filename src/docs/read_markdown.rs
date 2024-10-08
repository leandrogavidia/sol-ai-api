use std::{env, fs};

pub fn read_markdown(file_path: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    let file = fs::read_to_string(format!("{}{}", current_dir.to_str().unwrap(), file_path))
        .expect("Error reading file");
    file
}
