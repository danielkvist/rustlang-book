// A trait tells the Rust compiler about functionality a particular
// type has and can share with other types. Traits are similar to
// a feature often called interfaces in other languages like Go.

fn main() {
    let tweet = Tweet {
        username: String::from("danielkvist_"),
        content: String::from("testing traits on Rust!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
    notify(&tweet);
    notify(&article);
}

// We declare a trait using the trait keyword and then
// the trait's name, which is Summary. Inside the brackets
// we declare the method signatures that describe the behaviors
// of the types that implement this trait.
// A trait can have multiple methods in its body: the method
// signatures are liste one per line.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing a trait on a type is similar to implementing regular
// methods. The difference is that after impl we put the trait
// name that we want to implement the trait for.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// One restriction to note with traits implementation is that
// we can implement a trait on a type only if either the trait
// or the type is local to our crate.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// The notify function calls the summarize method on its
// item parameter, which is of some type that implements
// the Summary trait.
// Instead of a concrete type for the item parameter, we specify
// the impl keyword and the trait name. In the body of notify
// we can call any methods on item that come from the Summary
// trait, such as summarize.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax works for straightforward cases but
// is actually syntax sugar for a longer form, which is
// called a trait bound:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// The impl Trait syntax is convenient and makes for more concise
// code in simple cases. The trait bound syntax can express more
// complexity in other cases. For example, we can have
// two parameters that implement Summary.
// pub fn notify(item1: &impl Summary, item2: &impl Summar)
// pub fn notify<T: Summary>(item1: &T, item2: &T)

// We can also specify more than one trait bound using the
// + syntax.
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}
//
// But using too many trait bounds has its downsides. Rust
// has alternative syntax for specifying trait bound inside
// a where clause after a function signature.
// fn some_function<T, U>(t: &T, u: &U) -> i32 {
//     where T: Display + Clone,
//           U: Clone + Debug
// }

// We can also use the impl Trait syntax in the return
// position to return a value of some type that
// implements a trait.
// By using impl Summary for the return type, we specify
// that the function returns some type that implements
// the Summary trait without naming the concrete type.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probable already know, people"),
        reply: false,
        retweet: false,
    } // You can only use impl Trait if you're returning a single
      // type.
}

// We can also conditionally implement a trait for any type
// that implements another trait. Implementations of a trait
// on any type that satisfies the trait bounds are called
// blanket implementations.
// For example, the standard library implements the ToString
// trait on any type type that implements the Display trait.
// impl<T: Display> ToString for T {}
