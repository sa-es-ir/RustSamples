fn main() {
    let width = 10;
    let height = 20;

    let area = area(width, height);

    println!(
        "Area of rectangle with height {} and width {} is {}",
        height, width, area
    );

    let dimension = (10, 20);

    let area = area_tuple(dimension);

    println!(
        "Area tuple of rectangle with height {} and width {} is {}",
        dimension.0, dimension.1, area
    );

    let dim_struct = Rectangle {
        width: 10,
        height: 20,
    };

    let area = area_struct(&dim_struct);

    println!(
        "Area struct of rectangle with height {} and width {} is {}",
        dim_struct.height, dim_struct.width, area
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
