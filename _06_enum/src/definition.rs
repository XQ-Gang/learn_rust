use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// 枚举（enumerations, enums）允许你通过列举可能的 成员（variants）来定义一个类型。
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

// std::net::IpAddr
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {
    let four = IpAddrKind::V4;
    println!("four = {:?}", four);
    let six = IpAddrKind::V6;
    println!("six = {:?}", six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(Ipv4Addr::LOCALHOST);
    println!("home = {:?}", home);
    let loopback = IpAddr::V6(Ipv6Addr::LOCALHOST);
    println!("loopback = {:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
}
