use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let user_data_file: PathBuf = current_dir.join("src").join("user_data.txt");
    fs::read_to_string(user_data_file)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn get_last_char_of_username(username: &str) -> char {
    last_char_of_first_line(username).unwrap()
}

pub fn read_file_allow_question_operator() -> Result<(), Box<dyn Error>> {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let user_data_file: PathBuf = current_dir.join("src").join("user_data.txt");
    File::open(user_data_file)?;

    Ok(())
}
