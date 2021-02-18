// Vectors allow you to store more than one value
// in a single data structure. They can only store
// values of the same type and are useful when
// you have a list of items.

fn main() {
    // Vectors are implemented using generics.
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3]; // The integer type is i32 by default.

    // To add elements to a vector we can use the push method
    // but as we any variable we need to make it mutable
    // first.
    v.push(5);
    v.push(6);
    v.push(7);

    // We can access a value with the indexing syntax or
    // with the get method.
    let third: &i32 = &v[2]; // gives us a reference.
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // To iterate over a vector we can use a for loop
    for i in &v {
        println!("{}", i);
    }

    // Or iterate over mutable references to each element
    // in order to make changes.
    for i in &mut v {
        *i += 50;
    }

    // We can use enums variants to store elements
    // of a different type in a vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// When the vector gets dropped all of its contents
// are also dropped.
