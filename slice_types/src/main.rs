fn main() {
    let s = String::from("Hello world");

    let word = first_word(&s);

    let (part1, part2) = s.split_at(word);

    println!("{} --- {}", part1, part2);

    //using string slice
    let hello = &s[0..word]; // same as &s[..word]

    let world = &s[word..]; // same as &s[word..s.len()]

    println!("using string slice {} -- {}", hello, world);
}

fn first_word(s: &String) -> usize {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
