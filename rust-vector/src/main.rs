
pub mod rust_vector;

use rust_vector::main::RustVector;

fn main() {
    let mut empty_list: RustVector = RustVector {
        vector: &mut Vec::new()
    };
    let list: RustVector = RustVector {
        vector: &mut vec![-1, 0 , 1]
    };

    println!("Before -> Empty Vector: {:?}", empty_list.vector);
    println!("Before -> Non-Empty Vector: {:?}", list.vector);

    empty_list.add(1);
    empty_list.add(2);
    empty_list.add(3);

    println!("After -> Empty Vector: {:?}", empty_list.vector);
    println!("After -> Non-Empty Vector: {:?}", list.vector);

}
