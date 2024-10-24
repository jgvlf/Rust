#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main() {
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));

}
