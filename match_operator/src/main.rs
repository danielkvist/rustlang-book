// math allows you to compare a value against
// a series of patterns and then execute
// code based on which pattern matches.

fn main() {
    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penni!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // We need to cover all possible cases
        Some(i) => Some(i + 1),
    }
}
