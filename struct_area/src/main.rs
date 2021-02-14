fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_with_tuples(rect)
    );

    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect struct is {:?}", rect_struct); // Debug print
    println!("rect struct is {:#?}", rect_struct); // Debug print
    println!(
        "The area of the rectangle is {} square pixels",
        struct_area(&rect_struct) // using & main retains ownership
    );
}

// The problem with this function is on its signature.
// The parameters are related but that's not expressed
// anywhere.
fn area(w: u32, h: u32) -> u32 {
    w * h
}

// Tuples let us add a bit of structure and relation
// to our program but tuples don't name their elements
// so our calculation has become a little confusing.
fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// By adding #[derive(Debug)] we are opt in
// to make that functionality available for the
// struct Rectangle.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
