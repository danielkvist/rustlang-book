fn main() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 42, 6];
    println!("The largest number is {}", largest_i32(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest_char(&char_list));

    println!("The largest char is {}", largest(&number_list));
    println!("The largest char is {}", largest_char(&char_list));

    // We've used only one generic type to define Point<T> so
    // the fields x and y are both that same type, whatever
    // that type may be. If we create an instance of a Point<T>
    // that has values of different types, our code won't
    // compile.
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}, p.y = {}", integer.x(), integer.y);
    println!("p.x = {}", float.x());
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// When defining a function that uses generics, we place the
// generics in the signature of the function where we would
// usually specify the data types of the parameters and
// return value.
// We read this definition as: the function largest is generic
// over some type T. This function has one parameter named
// list, which is a slice of values of type T. And it will return
// a reference to a value of the same type T.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We can define structs to use a generic type parameter
// in one or mode fields.
struct Point<T> {
    x: T,
    y: T,
}

// To define a Point struct where x and y are both generics
// but could have different types we can use multiple
// generic type parameters.
//
// struct Point<T, U> {
//      x: T,
//      y: U,
// }
//
// You can use as many generic type paramenters
// as you want, but using more than a few makes your
// code hard to read. When you need lots of generics
// in your code it could indicate that your code
// needs restructuring.

// We can implement methods on struct and enums and
// use generic types in their definitions.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We could implement methods only on Point<f32> instances.
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//    }
// }
// This code means that tye type Point<f32> will have a method
// names distance_from_origin and other instances of Point<T>
// where T is not of type f32 will not have this method defined.
