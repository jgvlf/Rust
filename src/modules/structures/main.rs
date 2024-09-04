#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
#[allow(unused_variables)]
pub fn create_user() {
    let mut user: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2: User = User {
        email: String::from("another@example.com"),
        ..user
    };
    println!("{}", user.email);
    user.email = String::from("anotheremail@example.com");
    println!("{}", user.email);

    let mut adm: User = build_user(
        String::from("joaogabrielvlf@gmail.com"),
        String::from("jgvlf"),
    );
    println!("{}", adm.email);
    adm.email = String::from("Joao.Fernandes7@br.bosch.com");
    println!("{}", adm.email);
}

pub fn tuple_struct_instances() {
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}
