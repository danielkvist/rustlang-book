use rand::Rng;
use std::cmp::Ordering;
use std::io; // Imports the io library from the standard library.

// Entry point of the program.
fn main() {
    println!("Guess the number!");

    // Creates a random number generator and uses it.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess:");

    // In Rust variables are immutable by default so
    // we use mut before the variable name to make
    // the variable mutable.
    let mut guess = String::new(); // Returns a new instance of String.
                                   // ::new() indicates that new is an associated function
                                   // of the String type.

    // Now we call the stdin() function from the io library.
    io::stdin()
        .read_line(&mut guess) // & indicates a reference.
        .expect("Failed to read line"); // handles a possible error.

    // We shadow the prev guess variable parsing the previous
    // value in guess into an u32 int.
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    // Here the {} is a placeholder
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
