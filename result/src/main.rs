use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let _f = File::open("hello.txt"); // Result<T, E>

    // In the case where File::open succeeds, the value in the
    // variable f will be an instance of Ok that contains
    // a file handle. In the case where it fails, the value
    // in f will be an instance of Err that contains more information
    // about the kind of error that happened.
    // We need to take different actions depending on the value then.
    //     let _f = match f {
    //       Ok(file) => file,
    //      Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // The code above fails no matter why File::open failed. But we
    // can take different actions depending on differents failure
    // reasons.
    //let _f = match f {
    //    Ok(file) => file,
    //   Err(error) => match error.kind() {
    //       ErrorKind::NotFound => match File::create("hello.txt") {
    //          Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating file: {:?}", e),
    //    },
    //    other_error => {
    //        panic!("Problem opening the file: {:?}", other_error)
    //    }
    //  },
    //};

    // The type of the value that File::open returns inside the Err
    // variant is io::Error which is a struct provided by the
    // standard library. This struct has a method king that
    // we can call to get an io::ErrorKind value. This enum
    // is provided by the standard library and has variants
    // representing the different kinds of errors that might
    // result from an io operation.
    // The condition we want to check in the inner math is wheter
    // the value returned by error.kind() is the NotFound variant
    // of the ErrorKind enum. If it is, we try to create the
    // file with File::create but because File::create could
    // also fail, we need a second arm in the inner match expression.

    // A better way might be
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // But using match is a bit verbose and doesn't
    // communicate it's intent well. The Result<T, E> type
    // has many helper methods defined on it to
    // do various tasks and one of those methods is called
    // unwrap, which is a shortcut method. If the Result value
    // is Ok varian, unwrap will return the value inside. If the
    // Result is the Err varian, unwrap will call panic! for us.
    let _f = File::open("hello.txt").unwrap();

    // Another method, expect, which is similar to unwrap lets us
    // alos choose the panic! error message.
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Instead of handling the error within a function you can
// return it to the calling code (Propagating error) so
// that it can decide what to do.
// The code that calls this code will handle getting either an
// Ok value that contains the file's content or an Err value
// that contains an io::Error. This pattern of propagating errors
// is so common in Rust that Rust provides the question mark
// operator ? to make this easier.
// ? placed after a Result value works in almost the same way
// as the match expression. If the value of the Result is an Ok
// the value inside it will get returned from this expression
// and the program will continue. If the value is an Err, the
// Err will be returned from the whole function.
// The ? is only allowed on functions that returns
// Result or Option.
fn read_username_from_file() -> Result<String, io::Error> {
    /*     let f = File::open("user.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } */

    let mut f = File::open("user.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Using the ? we could even thorten the above code.
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("user.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    // Reading a file into a string is a fairly common operation
    // so Rust provides the convenient fs::read_to_string that opens
    // the file, creates a new String, reads the contents of the
    // file, puts the contents into that String and returns it.
    fs::read_to_string("user.text")
}
