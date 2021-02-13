// The rules of references
//
//  - At any given time, you can have either one mutable reference
//      or any number of immutable references.
//  - References must always be valid.

fn main() {
    let s = String::from("Hello, world");
    let len = calculate_length(&s); // we are passing a reference

    println!("The length of '{}' is {}", s, len);

    let mut s2 = s.clone();
    change_ref(&mut s2);

    println!("The string contains: '{}'", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len() // you're not allowed to modify it though
} // s goes out of scope here but because it does
  // not have ownership of what it refers to nothing
  // happens

// Just as variables are immutable by default so are
// references. So we need a mutable reference
fn change_ref(s: &mut String) {
    s.push_str("! How are you?")
}
