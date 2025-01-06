pub mod cpanic;
pub mod recoverable;

#[allow(unused_imports)]
use cpanic::ccall;

use recoverable::cresult;

fn main() {
    // ccall::run_panic();
    // ccall::acess_index();
    cresult::open_file();
}
