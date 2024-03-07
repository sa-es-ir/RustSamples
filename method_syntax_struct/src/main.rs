fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Rect1 is {:#?}", rect1);

    println!("Rect1 area is {}", rect1.area());

    println!("Rect1 with is {}", rect1.width());

    println!("New rect1 {:#?}", rect1.make_new());

    let rect2 = Rectangle {
        width: 5,
        height: 19,
    };

    println!("Rect1 can hold Rect2: {}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn make_new(&self) -> Self {
        Self {
            height: self.height,
            width: self.width,
        }
    }

    fn width(&self) -> u32 {
        self.width
    }
}
