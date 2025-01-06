pub fn run_panic() {
    panic!("crash and burn");
}

pub fn acess_index() {
    let v: Vec<i32> = vec![1, 2, 3];
    v[99];
}
