const THREE_HOURS_IN_SEC: u32 = 60 * 60 * 3;
fn main() {
    let mut x = 5;
    println!("the value of x is {x}");

    x += 6;

    println!("new value of x is : {x}");

    println!("const value of x is : {THREE_HOURS_IN_SEC}");

    let x = x * 2;

    {
        let x = x * 2;
        println!("local scope of x: {x}");
    }

    println!("outer scope of x : {x}");

    let str = "saeed";

    let str = str.len();

    println!("str value is: {str}");
}
