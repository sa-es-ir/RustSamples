fn main() {
    let s = String::from("Hello world");

    let word = first_word_no_slice(&s);

    let (part1, part2) = s.split_at(word);

    println!("{} --- {}", part1, part2);

    //using string slice
    let hello = &s[0..word]; // same as &s[..word]

    let world = &s[word..]; // same as &s[word..s.len()]

    println!("using string slice {} -- {}", hello, world);

    let s = String::from("singleword");
    let first_word = first_word_with_slice(&s);

    println!("Got first word with slice: {}", first_word);
}

fn first_word_no_slice(s: &String) -> usize {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
