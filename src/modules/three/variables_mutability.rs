pub mod variables {
    pub fn error_bind() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
    pub fn shadowing() {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x is: {x}")
        }

        println!("The value of x is: {x}")
    }
    pub fn mut_error() {
        let spaces = "    ";
        let spaces: usize = spaces.len();
        println!("{spaces}");
    }
}
