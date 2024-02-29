fn main() {
    let mut str_heap = String::from("hello");

    str_heap.push_str(" world!");

    println!("{}", str_heap);

    let s1 = String::from("hello");

    let mut s2 = s1.clone();

    s2.push_str("string");

    println!("s1 = {}, s2 = {}", s1, s2);
}
