pub mod cpanic;
pub mod example;
pub mod recoverable;

#[allow(unused_imports)]
use cpanic::ccall;

#[allow(unused_imports)]
use recoverable::cresult;

#[allow(unused_imports)]
use recoverable::shortcuts;

use example::username;

fn main() {
    // ccall::run_panic();
    // ccall::acess_index();
    // cresult::open_file();
    // shortcuts::clean_open_file();
    // shortcuts::expect_open_file();
    let user: String = username::read_username_from_file().unwrap();
    println!("Username: {user:?}");
}
