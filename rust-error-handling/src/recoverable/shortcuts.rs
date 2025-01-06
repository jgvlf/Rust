use std::fs::File;

#[allow(unused_variables)]
pub fn clean_open_file() {
    let greeting_file: File = File::open("hello.txt").unwrap();
}

#[allow(unused_variables)]
pub fn expect_open_file() {
    let greeting_file: File =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
