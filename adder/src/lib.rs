#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // The tests mod is a regular module that follows the usual
    // visibility rules. Because the test module is an inner module,
    // we need to bring the code under test in the outer module
    // intro scope of the inner module.
    use super::*;

    #[test] // indicates that is a test function
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // Tests fail when something in the test function
    // panics. Each test is run in a new thread and when the
    // main thread sees that a test thread has died, the
    // test is marked as failed.
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // The assert! macro is useful when you want to ensure
    // that some condition in a test evalutes to true.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // You can also add a custom message to be printed with the
        // failure message as optional arguments to the assert!,
        // assert_eq! and assert_ne!
        assert!(
            result.contains("Carol"),
            "Greeting did not container name, value was `{}`",
            result,
        );
    }

    // In addition to checking that our code returns the correct
    // values we expect, it's also important to check that our
    // code handles error conditions as expected.
    // But tests that use should_panic can be imprecise because
    // they only indicate that the code has caused some panic. To
    // make should_panic tests more precise, we can add an optional
    // expected parameter to the should_panic attribute.
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greather_tan_100() {
        Guess::new(200);
    }

    // We can also write tests that use Result<T, E>. We return
    // Ok(()) when the test passes and Err with a String inside when
    // the test fails.
    // Writing tests so they return a Result<T, E> enables you to use
    // the question mark operator in the body of a test, which can be
    // a convenient way to write tests that should fail if any
    // operation within them returns an Err variant.
    // Keep in mind that you can't use #[should_panic] on tests
    // that use the Result<T, E>.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }
}
