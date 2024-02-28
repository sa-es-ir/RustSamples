use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100).to_string();

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    println!("Your guessed number is {guess}");

    let compare_result = match guess.cmp(&secret_number) {
        Ordering::Equal => "Equal",
        Ordering::Greater => "Greater",
        Ordering::Less => "Less",
    };

    println!("Result is {compare_result}");
}
