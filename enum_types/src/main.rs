use std::fmt::format;

fn main() {
    let four = IpAddrKind::V4;

    println!("Ip kind {:?}", four);

    let four = IpKindWithValue::V6 { x: 123, y: 124 };

    println!("Ip kind {:?}", four);

    let four = IpKindWithValue::V4Numbers(127, 0, 0, 1);

    println!("Ip kind {:?}", four.V4Numbers());
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpKindWithValue {
    V4(String),
    V6 { x: u32, y: u32 },
    V4Numbers(u32, u32, u32, u32),
}

impl IpKindWithValue {
    fn V4Numbers(&self) {
        println!("----- {:?}", self);
    }
}
