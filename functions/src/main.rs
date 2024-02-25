fn main() {
    println!("Hello, world!");

    no_params();

    with_params("Saeed", 34);

    let result = with_params_result("Saeed", 34);

    println!("the result is: `{result}`");

    loop_with_limit(10);

    while_with_limit(20);
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

fn loop_with_limit(max: u32) {
    let mut counter = 0;
    loop {
        counter += 1;

        println!("counter: {counter} but max: {max}");

        if counter == max {
            break;
        }
    }
}

fn while_with_limit(max: u32) -> String {
    let mut counter = 0;
    while counter < max {
        counter += 1;
        println!("while loop, counter: {counter} and max: {max}");

        if counter % 21 == 0 {
            return "Done at {counter}".to_owned();
        }
    }

    "fadf".to_owned()
}
