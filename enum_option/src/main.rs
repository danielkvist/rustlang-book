// The Option enum is defined by the standard library.
// It encodes the very common scenario in which a value
// could be something or it could be nothing.

fn main() {
    let _some_string = Option::Some("hello, world");
    let _some_number = Some(5);

    // If we use None we need to define what type
    // of Option<T> we have.
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let _sum = x + y; // Doesn't work

    // We have to convert an Option<T> to a T before you can
    // perform T operations with it. This helps to catch one
    // of the most common issues with null: assuming that
    // something isn't null when it actually is.
}
