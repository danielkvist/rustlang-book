fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rect);

    println!("the area of the rectangle is {} square pixels", rect.area()); // Using the area method of rect

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    let _sq = Rectangle::square(3); // Calls an associated function
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl -> implementation
//
// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    // Rust knows that the type of &self is Rectangle.
    // It uses &self because we don't want to take
    // the ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // You can take more parameters than &self.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Or event don't take the &self parameter. These are called
    // associated functions because they're associated with the struct
    // but because they don't have an instance of the struct
    // they aren't methods.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
