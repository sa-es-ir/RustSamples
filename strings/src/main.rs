use std::iter;

fn main() {
    let s = String::from("Hello");
    let s2 = String::from("world!");

    let s3_1 = format!("{s} {s2}");
    let s3 = s + &s2;
    println!("{} {}", s3_1, s2);

    let mut string = String::from("Здравствуйте");

    if string.contains("З") {
        println!("YES");
    }

    for c in string.chars() {
        print!("{c}-");
    }
    println!("");
    string = string.replace("З", "S").replace("драв", "ZXC");
    println!("-----------");
    for c in string.chars() {
        print!("{c}-");
    }
    let ch = &string[0..4];

    println!("\t {ch}");
}
