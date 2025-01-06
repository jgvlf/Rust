use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let user_data_file: PathBuf = current_dir.join("src").join("user_data.txt");
    File::open(user_data_file)?.read_to_string(&mut username)?;
    Ok(username)
}
