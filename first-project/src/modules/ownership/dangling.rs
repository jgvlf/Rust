#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sim_dangling() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
