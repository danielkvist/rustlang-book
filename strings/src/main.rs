fn main() {
    let mut _s = String::new(); // creates a new empty string

    // On any type that implements the Display trait
    // we can use the to_string method.
    let data = "hello";
    let _data_str = data.to_string();
    let _lit = "hello".to_string();

    // Or use the function String::from() to create a String
    // from a string literal.
    let mut _s = String::from("hello ");

    // We can grow a String by using the push_str method
    _s.push_str("world");

    _s.push('!'); // push takes a single character.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and
                       // cannot be longer used.

    println!("{}", s3);

    // The reason s1 is no logen valid after the addition
    // and the reason we use a reference to s2 has to do with
    // the signature of the method that gets called when we use
    // the + operator.
    // fn add(self, s: &str) -> String {}

    // To more complicated string combination is better to use
    // the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Invalid code!
    // let h = s1[0]; // Rust strings don't support indexing.

    // A String is a wrapper over a Vec<u8>.
    let _hello = String::from("Hola");

    // In the case of hello, the len will be 4. Which means
    // the vector is storing the string "Hola"  in 4 bytes
    // long. But that's not always the case.
    // In the next case the len is 24, and not 12. So an index
    // into the string's bytes will not always correlate to
    // a valid Unicode scalar value.
    let _hello = String::from("Здравствуйте");

    // In Rust in fact there are actually 3 relevant ways
    // to look at strings from Rust's perspective:
    //  * bytes
    //  * scalar values
    //  * grapheme clusters (letters)

    // A final reason of why Rust doesn't allow us to index
    // into a String to get a character is that
    // indexing operations are expected to always take
    // constant time (O(1)). But it isn't possible
    // to guarantee that with a String.

    // If you need to perform operations on individual Unicode
    // scalar values, the best way to do so is to the chars
    // method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // You can also use the bytes methods, which returns
    // each raw byte.
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
