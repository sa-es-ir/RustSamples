use restaurant::front_of_house::hosting::add_to_waitlist;

fn main() {
    println!("Hello, world!");

    let break_fast = restaurant::back_of_house::Breakfast::summer("yes");

    println!("{:#?}", break_fast);
}
