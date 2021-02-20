use std::collections::HashMap; // We need to import them.

fn main() {
    // You can create a hash map with the new keyword.
    let mut scores = HashMap::new();

    // Just like vectors, hash maps store their data
    // in the heap.

    // All the keys must have the same type and all
    // of the values must have the same type.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // Another way of constructing a hash map is by
    // using iterators and the collect method.
    // The zip method creates a vector of typles where Blue
    // is paired with 10 and so forth. The collect method
    // turns that vector of tuples into a hash map.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // If a type implements the Copy trait it's value is
    // copied into the hash map. For owned values like String
    // the value will be moved and the hash map will be
    // the owner.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_vale are invalid at this point. To
    // solve this we can pass reference points but those references
    // must be valid for at least as long as the hash map is valid.

    // To get a value out of a hash map we use the get method.
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name); // Some(&10)

    // If there's no value for that key in the hash map
    // get will return None. That's why it returns Some(&10) in
    // this case.

    // We can iterate over each key/value pair in a hash map
    // using a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Each key can only have one value associated with it
    // at a time. When you want to change the data in a hash
    // map, you have to decide how to handle the case when a key
    // already has a value assigned.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // Will print {"Blue": 25}

    // It's very common to check whether a particular key
    // has a value and if it doesn't, insert a value
    // for it. Hash maps have a special API for this
    // called entry.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // The or_insert method on Entry is defined to return
    // a mutable reference to the value for the corresponding
    // Entry key if that key exists, and if not, insert the
    // parameter as the new value.
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(20);

    println!("{:?}", scores);

    // Another common use case for hash maps is to look up
    // a key's value and then update it based on the old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for w in text.split_whitespace() {
        let count = map.entry(w).or_insert(0); // returns a mutable reference
        *count += 1;
    }

    println!("{:?}", map);
}
