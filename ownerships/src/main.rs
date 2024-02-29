fn main() {
    let mut str_heap = String::from("hello");

    str_heap.push_str(" world!");

    println!("{}", str_heap);
}
