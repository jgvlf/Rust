use std::fs;
use std::io;
use std::path::PathBuf;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let user_data_file: PathBuf = current_dir.join("src").join("user_data.txt");
    fs::read_to_string(user_data_file)
}
