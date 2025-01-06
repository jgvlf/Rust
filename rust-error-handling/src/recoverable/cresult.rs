use std::fs::File;

pub fn open_file() {
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
