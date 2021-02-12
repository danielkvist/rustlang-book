fn main() {
    // Rust has three kinds of loops:
    //  * loop
    //  * while
    //  * for

    loop {
        println!("again");
        break; // You have to exit manually
    }

    // You can use loops to retry an operation
    // adding the value you want returned
    // after the break expression
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // You can use a loop or a while loop to iterate
    // over each item in a collection but the best way
    // is to use a for loop
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    // If you want to run a loop a certain number of times
    // as in the countdown example with a for loop you can use
    // a Range
    for number in (1..4).rev() {
        println!("{}", number); // .rev() reverses the range
    }
}
