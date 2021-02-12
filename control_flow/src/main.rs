fn main() {
    let age = 18;

    // The condition must be a bool.
    if age < 18 {
        println!("You can't enter this site");
    } else {
        println!("Welcome");
    }

    let n = 6;

    // If you have a lot of if...else you may
    // want to use match.
    if n % 3 == 0 {
        println!("Number is odd");
    } else if n % 2 == 0 {
        println!("Number is even");
    } else {
        println!("Number is not divisible by 3 or 2");
    }

    let condition = true;
    let n = if condition { 5 } else { 6 }; // if is an expression
                                           // so you can use it on the rigth
                                           // side of a let statement

    println!("The value of n is: {}", n);
}
