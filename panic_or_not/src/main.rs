// When code panics, there's no way to recover. When
// you choose to return a Result value, you give
// the calling code options rather than making
// the decision for it. Therefore, returning
// Result is normally a better choice when you're defining
// a function that might fail. But in rare situations is better
// to panic.
// For example, when you're writing an example having
// robust error-handling code in the example can make
// it less clear.
// Or when you're prototyping.
// Or if a method call fails in a test, you'd want the whole
// test to fail.

use std::net::IpAddr;

fn main() {
    // You can use something like unwrap when you have some
    // logic that ensures the Result will have an Ok value, but
    // the logic isn't something the compiler understands.
    // Here 127.0.0.1 is a valid IP address, so it's acceptable
    // to use unwrap here.
    let _home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("Hello, world!");
}

// It's recommended to have your code panic when it's possible
// that your code could end up in a bad state. A bad state
// is when some assumption, guarantee, contract or invariant
// has been broken, such as when invalid values, contradictory
// values or missing ones are passed to your code.
// If someone calls your code and passes in values that don't
// make sense, the best choice is to panic! and alert
// the person using your library about the bug in their code.
// Similarly, panic! is often appropiate if you're calling
// external code that is out of your control and it returns an
// invalid state that you have no way of fixing.
// You should also use panic! when your code performs operations
// on values if the values aren't valid. This is due to safety reasons.
// Functions often have contracts: their behavior is only guaranteed
// if the inputs meet particular requirements. Panicking when a contract
// is violated makes sense. But this should be explained in the API
// documentation for the function.

// We can make a new type and put the validations in a function
// to create an instance of the type rather than repeating the
// validations everywhere.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    // getter
    pub fn value(&self) -> i32 {
        self.value()
    }
}
