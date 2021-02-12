fn main() {
    print_hi(); // function call
    multiply_by_two(4);
    add(4, 2);

    // In Rust functions bdies are made up of statements
    // optionally ending in an expression. Rust is an
    // expression-based language.
    // Functions definitions are also statements.
    // Statements do not return values so in Rust you
    // can't do something like x = y = 6.
    // Expressions evaluate to something, like 5 + 6 or
    // calling a function.

    // this block {} is also an expression
    let y = {
        let x = 3;
        x + 1 // expressions doesn't end on a ;
              // if you add a ; it will no return 4
              // because it will convert this line into a
              // statement and statements doesn't return a value
    };

    println!("The value of y is: {}", y);

    let r = add_and_return(4, 5);
    println!("{}", r);
}

// function declaration
fn print_hi() {
    println!("Hello, world!");
}

// x is a parameter with an i32 type.
// In Rust you are required to declare the type
// of each parameter.
fn multiply_by_two(x: i32) {
    println!("The value of x by 2 is: {}", x * 2);
}

fn add(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// Returns the result of the operation which is an i32
fn add_and_return(x: i32, y: i32) -> i32 {
    x + y
}
