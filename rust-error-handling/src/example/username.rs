use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let user_data_file: PathBuf = current_dir.join("src").join("user_data.txt");
    let mut username_file_result = File::open(user_data_file)?;

    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}
