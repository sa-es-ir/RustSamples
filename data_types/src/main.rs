fn main() {
    let str_num: u32 = "25".parse().expect("Not a number");
    println!("Coverted string is {str_num}");

    let tuple = (134, 9.056, "tuple is a compound type");

    let one = tuple.1;
    println!("tuple displays as {one}");

    let (x, y, z) = tuple;

    println!("tuple splitted in {x} - {y} - {z}");

    let arr = [1, 2, 3, 4, 5, 5];

    let xx = arr[0];
    println!("fdsafds {xx} fdfd");

    //let arr_str: [&str; 3] = ["fasdf", "fff", "2ed"];
}
