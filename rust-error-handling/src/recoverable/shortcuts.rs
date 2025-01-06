use std::fs::File;

#[allow(unused_variables)]
pub fn clean_open_file() {
    let greeting_file: File = File::open("hello.txt").unwrap();
}
