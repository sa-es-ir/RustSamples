fn main() {
    println!("Hello, world!");

    no_params();

    with_params("Saeed", 34);

    let result = with_params_result("Saeed", 34);

    println!("the result is: `{result}`");
}

fn no_params() {
    println!("no param function gets called");
}

fn with_params(name: &str, age: u32) {
    println!("{name} is {age} years old!");
}

fn with_params_result(name: &str, age: u32) -> String {
    format!("{name} is {age} years old!")
}
