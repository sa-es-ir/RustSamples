use restaurant::{back_of_house, front_of_house::hosting};

fn main() {
    println!("Hello, world!");

    hosting::add_to_waitlist();
    let break_fast = back_of_house::Breakfast::summer("yes");

    println!("{:#?}", break_fast);
}
