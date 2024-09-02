#[allow(dead_code)]
pub fn branches() {
    let number: i8 = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

#[allow(dead_code)]
pub fn diff_zero() {
    let number: i8 = 3;

    if number != 0 {
        println!("Number was something other than zero.");
    }
}

#[allow(dead_code)]
pub fn mult_cond_handler() {
    let number: i8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[allow(dead_code)]
pub fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

#[allow(dead_code)]
pub fn infinity_loop() {
    loop {
        println!("again!");
    }
}

#[allow(dead_code)]
pub fn return_loop_value() {
    let mut counter: i8 = 0;

    let result: i8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

#[allow(dead_code)]
pub fn labeled_loops() {
    let mut count: i8 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

#[allow(dead_code)]
pub fn cond_while_loop() {
    let mut number: i8 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

#[allow(dead_code)]
pub fn loop_in_collection() {
    let a: [i8; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

#[allow(dead_code)]
pub fn for_in_collection() {
    let a: [i8; 5] = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}")
    }
}

#[allow(dead_code)]
pub fn for_in_rev_range() {
    for element in (1..4).rev() {
        println!("The value is: {element}")
    }
}
