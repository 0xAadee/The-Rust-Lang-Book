// Enums allow us to enumerate a list of variants
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Hello World!")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Variants are namespaced under their identifiers
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}
