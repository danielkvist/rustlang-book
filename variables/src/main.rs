fn main() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6; // Error
    // println!("The value of x is: {}", x);

    let mut y = 7; // y is mutable
    println!("The value of y is: {}", y);

    y = 9;
    println!("The value of y is: {}", y);

    // You are unable to use mut with constants because
    // constants will be always immutable.
    // So constants and variables are not the same in Rust
    // Also the type of a constant must be annotated and they
    // can be declared at global scope.
    // The last thing is that a constant cannot be the result
    // of a function call or any other value computed at runtime.
    const MAX_POINTS: u32 = 100_000;
    println!("Max number of points: {}", MAX_POINTS);

    // With variables we can shadow them
    let x = 7; // x was defined above
    println!("The value of x is: {}", x);

    let x = x + 1; // assigns new value using old x value
    println!("The value of x is: {}", x);

    // When we shadow a variable we can change it's type
    // reusing the same name.
    let spaces = "    "; // String
    let _spaces = spaces.len(); // int
}
