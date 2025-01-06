pub mod cpanic;
pub mod recoverable;

use cpanic::ccall;

fn main() {
    // ccall::run_panic();
    ccall::acess_index();
}
