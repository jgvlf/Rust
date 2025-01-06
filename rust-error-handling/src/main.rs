pub mod cpanic;
pub mod recoverable;

#[allow(unused_imports)]
use cpanic::ccall;

#[allow(unused_imports)]
use recoverable::cresult;

use recoverable::shortcuts;

fn main() {
    // ccall::run_panic();
    // ccall::acess_index();
    // cresult::open_file();
    // shortcuts::clean_open_file();
    shortcuts::expect_open_file();
}
