// Created by running cargo new --lib restaurant

// Defines a module. Inside a module
// we can have other modules.
mod front_of_house {
    // Modules can have definitions for other
    // items like structs, enums, constants, etc.
    pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_resurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it. 
    // we're not allowed to see or modify the seasonal
    // fruit that comes with the meal:
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // It goes to the parent module
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // The struct Breakfast needs to provide a public
    // associated function that constructs an instance of
    // Breakfast because the struct has a private field.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast we can make an entire enum public
    // usin the pub keyword before the enum keyword.
    pub enum Appetizer {
        Soup, // public
        Salad, // public
    }
}