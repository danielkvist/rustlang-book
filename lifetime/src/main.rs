// Every reference in Rust has a lifetime, which is the scope
// for which that reference is valid. Most of the time
// lifetimes are implicit and inferred, just like types.
// We must annotate types when multiple types are possible. In
// a similar way, we must annotate lifetimes when the lifetimes
// when the lifetimes of references could be realted in a few
// different ways.

fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }

    // The variable x doesn't live long enough. The reason is that x
    // will be out of scope when the inner scope ends on line 16.
    // But r is still valid for the outer scope; because its scope
    // is larger ("it lives longer"). If Rust allowed this code
    // r would be referencing memory that was deallocated when x
    // went out of scope.

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), &string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // One special lifetime is static, which means that this reference
    // can live for the entire duration of the program.
    let _s: &'static str = "I have a static lifetime";
}

// We don't know the concrete values that will be passed
// into this function, so we don't know whether the if case
// of the else case will execute. We also don't know
// the concrete lifetimes of the references that will be passed in
// so we can't look at the scopes as we did before to determine
// whether the reference we return will always be valid. The borrow
// checker can't determine this either.
//
// To fix this error we'll add generic lifetime parameters
// that define the relationship between the references so the
// borrow checker can perform its analysis.
//
// Lifetime annotations don't change how long any of the references
// live. Lifetime annoations describe the relationships of the
// lifetimes of multiple references to each other without
// affecting the lifetimes.
// Examples:
// &i32 -> A reference
// &'a i32 -> A references with an explicit lifetime
// &'a mut i32 -> a mutable reference with an explicit lifetime
//
// The function signature now tells Rust that for some lifetime 'a
// the function thakes two parameters, both of which are string slices
// that live at least that long as lifetime 'a. In practice it means
// that the lifetime of the reference returned by the longest
// function is the same as the smaller of the lifetimes of the
// references passed in.
// Note that the longest function doesn't need to know exactly
// how long x and y will live, only that some scope can be
// substituted for 'a.
//
// Ultimately, lifetime syntax is about connecting the lifetimes of
// various parameters and return values of functions.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// It's possible for structs to hold references, but in that
// case we would need to add a lifetime annoation on
// each one.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
