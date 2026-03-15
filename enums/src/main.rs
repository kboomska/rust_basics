// Перечисление типов IP адресов.
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Значения перечислений.

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {}
