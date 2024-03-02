fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("string value {} and len {}", s1, len);

    change(&mut s1);

    println!("new value after change string is {}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world!");
}
