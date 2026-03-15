// Перечисление типов IP адресов.
enum IpAddrKind {
    V4,
    V6,
}

// Хранение значения IP адреса вместе с его типом с использованием структур.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Значения перечислений.

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn route(ip_kind: IpAddrKind) {}
