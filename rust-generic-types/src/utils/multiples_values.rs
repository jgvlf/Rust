pub fn get_largest_list_number() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let mut largest: &i32 = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}.");
}
