use std::ops::Add;

fn main() {
    let mut str_heap = String::from("hello");

    str_heap.push_str(" world!");

    println!("{}", str_heap);

    let s1 = String::from("hello");

    let mut s2 = s1.clone();

    s2.push_str("string");

    println!("s1 = {}, s2 = {}", s1, s2);

    let str_heap = String::from("hi hi");

    takes_ownership(str_heap);

    //str_heap.push('c'); error because it's moved!

    let str_heap = String::from("hi2");

    let str_heap = takes_gives_ownership(str_heap);

    println!("takes ownership again yay {}", str_heap);

    let x = 5;
    only_copy(x);

    println!("after calling function {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn only_copy(some_thing: u32) {
    println!("{}", some_thing);
}

fn takes_gives_ownership(some_string: String) -> String {
    some_string.add("rhs")
}
