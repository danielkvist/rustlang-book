// Integer types
//
// length - signed - unsigned
// 8bit - i8 - u8
// 16bit - i16 - u16
// 32bit - i32 - u32
// 64bit - i64 - u64
// 128bit - i128 - u128
// arch - isize - usize

// Floating points
//
// 32bit - f32
// 64bit - f64

// Boolean
//
// tue
// false

fn main() {
    // Integers
    let _decimal = 98_222; // _ is a separator between numbers
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte: u8 = b'A';

    // Floating point numbers
    let _fx = 2.0; // f64
    let _fy: f32 = 3.0; // f32

    // Booleans
    let _t = true;
    let _f: bool = false; // explicit type annotation

    // Chars
    let _char = 'z'; // '' instead of ""
    let _heart_eyed_cat = '😻';

    // Tuples
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup; // destructuring
    let _first_item = _tup.0; // index 0
    let _second_item = _tup.1; // index 1

    // Arrays
    let _arr = [1, 2, 3, 4, 5]; // every item must have the same type
    let _arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32 is the type and 5 the number of elements
    let _arr = [3; 5]; // [3, 3, 3, 3, 3]
    let _first_item = _arr[0]; // index 0
    let _second_item = _arr[1]; // index 1

    // Numerical operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;
}
