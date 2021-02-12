fn main() {
    println!("Enter a temperature in Celsius:");

    let mut c = String::new();
    std::io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

    let c: i32 = match c.trim().parse() {
        Ok(temp) => temp,
        Err(_) => -1,
    };

    println!("{}Cº = {}Fº", c, celsius_to_fahrenheit(c));
}

fn celsius_to_fahrenheit(c: i32) -> i32 {
    // C to F = C * (9 / 5) + 32
    c * (9 / 5) + 32
}
