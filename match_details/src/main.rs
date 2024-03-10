fn main() {
    let coin = Coin::Penny;

    println!("coin in cents: {}", value_in_cents(coin));

    let q_coin = Coin::Quarter(UsState::Alaska);
    println!("coin in cents: {}", value_in_cents(q_coin));
}

#[derive(Debug)]
enum UsState {
    NY,
    Alabama,
    Alaska,
    //others
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky 1 cent");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("us state {:?}", state);
            25
        }
        _ => 2, // discard, for covering any other match arms
    }
}
