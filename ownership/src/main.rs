// Ownership rules
//
// - Each value in Rust has a variable that's called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

// In the case of a string literal we know the contents at
// compile time, so the text size is know. But with the String
// type in order to support mutable and growable pieces of text
// we need to allocate an amount of memory on the heap which
// size is unknow at compile time.

fn main() {
    // At this point s is not valid
    let s = "hello"; // s is valid from this point forward

    println!("{}, world", s);

    // String type is allocated on the heap
    // and as such is able to store an amount of text
    // that is unknown at compile time.
    // This kind of string can be mutated
    let mut str = String::from("hi"); // ::from() access the from
                                      // function uder the String type

    str.push_str(", world"); // appends a literal to the String
    println!("{}", str);

    let str_two = str; // We are copying the pointer, the length
                       // and the capacity.
                       //
                       // To avoid the double free error or just
                       // freeing memory twice str is no longer valid.
                       // This is called a move

    println!("{}", str_two); // You can't use str here

    // If we want a deeply copy of the data we can use
    // clone.
    let s1 = String::from("hi");
    let s2 = s1.clone(); // May be very expensive

    println!("s1 = {}, s2 = {}", s1, s2); // s1 is still valid

    // The next code contradicts the above comments.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // works

    // The reason that it works with types such as integers
    // is because integers have a known size at compile time
    // and are stored on the stack and not on the heap.

    takes_ownership(s2); // s2 moves into the function
                         // and stops being valid here

    makes_copy(x); // x would mode into the function
                   // but as a copy so it's okay and x
                   // is still valid here

    let other_str = String::from("Hello");
    let _returned = takes_and_returns(other_str); // other_str is moved
                                                  // and gave back
}
// The scope is over so s or str are no longer valid.

// When a variable goes out of scope Rust calls a special
// function called drop which returns the memory.

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn takes_and_returns(a_string: String) -> String {
    a_string
}
