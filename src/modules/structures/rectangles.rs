#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn exe() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    show_rect_structure()
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn show_rect_structure() {
    let scale: u32 = 2;
    let rect: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect); // inline
                 // println!("{rectangle:#?}"); //prety
}
