fn main() {
    let s = String::from("hello world!");

    // These are references to a portion of
    // the String
    let hello = &s[0..5]; // You could also write &s[..5]
    let world = &s[6..11]; // Or &s[6..]
    let whole = &s[..];

    println!("{}, {}", hello, world);
    println!("{}", whole);

    let first = first_word(&s);
    println!("{}", first);

    let arr = [1, 2, 3, 4, 5];
    let _slice = &arr[1..3];
}

// The signature first_word(s: &str) -> &str
// will allow you to use the same function on
// &String values and &str values
fn first_word(s: &String) -> &str {
    // We need to go through the String element
    // by element and check whether a value is a space
    // so we convert the String to an array of bytes.
    let bytes = s.as_bytes();

    // Then we create an iterator over the array
    // using the iter method. enumerate wrapes the result
    // of iter and returns each element as a tuple
    // with the index and a referente to the element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
