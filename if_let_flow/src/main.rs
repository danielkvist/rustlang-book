fn main() {
    let some_value = Some(0u8);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // If we want to do something with the Some(3) match
    // but do nothing with any other Some<u8> or None value
    // we can use if let.
    //
    // if let takes a pattern and an expression separated
    // by an =.
    if let Some(3) = some_value {
        println!("three");
    } else {
        println!("not three!"); // we can also use else with if let
    }
}
