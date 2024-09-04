#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[allow(dead_code)]
struct Color(u8, u8, u8);

#[allow(dead_code)]
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
#[allow(unused_variables)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn tuple_struct_instances() {
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn unit_like_struct() {
    let subject: AlwaysEqual = AlwaysEqual;
}
