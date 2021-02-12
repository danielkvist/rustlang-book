fn main() {
    println!("Hello, world!");

    for n in 1..26 {
        println!("Fib({}) = {}", n, fib(n));
    }
}

fn fib(n: i32) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
