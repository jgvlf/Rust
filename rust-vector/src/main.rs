pub mod rust_vector;

use std::vec;

use rust_vector::main::RustVector;

fn main() {
    let mut empty_list: RustVector = RustVector {
        vector: &mut Vec::new(),
    };
    let list: RustVector = RustVector {
        vector: &mut vec![-1, 0, 1],
    };

    let small_list: RustVector = RustVector {
        vector: &mut vec![-957],
    };

    println!("Before -> Empty Vector: {:?}", empty_list.vector);
    println!("Before -> Non-Empty Vector: {:?}", list.vector);

    empty_list.add(1);
    empty_list.add(2);
    empty_list.add(3);
    empty_list.add(4);
    empty_list.add(5);
    empty_list.add(6);

    println!("After -> Empty Vector: {:?}", empty_list.vector);
    println!("After -> Non-Empty Vector: {:?}", list.vector);

    // get value by indexing
    let third: &i64 = &empty_list.vector[2];
    println!("The third element is {third}.");

    // get value by the `get` method
    let third: Option<&i64> = small_list.vector.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }
    println!("{:?}", third);

    let outbound_list: RustVector = RustVector {
        vector: &mut vec![1, 2, 3, 4, 5],
    };
    // handle
    let does_not_exist: Option<&i64> = outbound_list.vector.get(100);
    println!("{:?}", does_not_exist);
    // get error
    // let does_not_exist: &i64 = &outbound_list.vector[100];
    // println!("{}", does_not_exist);

    // immutable iteration
    let v: Vec<i32> = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    // mutable iteration
    let mut v: Vec<i32> = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
    println!("{:?}", v);

    // create a enum to use as type in a vector to use diferent data types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
