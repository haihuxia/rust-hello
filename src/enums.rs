fn a() {
    enum IpAddKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }

    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
}

fn b() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

fn c() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}