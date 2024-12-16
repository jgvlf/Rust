#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn exe() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );

    // show_rect_structure()
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[allow(dead_code)]
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[allow(dead_code)]
fn show_rect_structure() {
    let scale: u32 = 2;
    let rect: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect); // inline
                 // println!("{rectangle:#?}"); //prety
}
