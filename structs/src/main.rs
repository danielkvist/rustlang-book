struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}

fn main() {
    // Creating an instance of the Struct User.
    let user = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
        sing_in_count: 2,
    };

    // To access a specific value from a struct
    // we use dot notation.
    println!("user name is '{}'", user.username);
    println!("user email is '{}'", user.email);
    println!("user is active: '{}'", user.active);
    println!("user sign in count is '{}'", user.sing_in_count);

    // Creates another instance of User using
    // different values to the email and username
    // but the same values for the active and
    // sign_in_count fields from user.
    let another_user = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        ..user
    };

    println!("user name is '{}'", another_user.username);
    println!("user email is '{}'", another_user.email);
    println!("user is active: '{}'", another_user.active);
    println!("user sign in count is '{}'", another_user.sing_in_count);

    // Tuple structs look similar to tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Note that _black and _origin values are
    // different types because they're instances
    // of different tuple structs.
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like structs don't have any fields. They
    // are useful in situations in which you need to
    // implement a trait of some type but don't have
    // any data that you want to store in the type itself.
    struct Hotdog;

    let _food = Hotdog;
}

fn _build_user(email: String, username: String) -> User {
    User {
        email: email,
        username,
        active: true,
        sing_in_count: 1,
    }
}
