use std::io;

fn celsius_to_farenheit() {}

fn farenheit_to_celsius() {}

fn main() {
    println!("Type your current units");

    let mut current_units = String::new();

    io::stdin()
        .read_line(&mut current_units)
        .expect("Failed to read line");

    let current_units = current_units.trim();

    match current_units {
        "C" => println!("current units are Celsius"),
        "F" => println!("current units are Farenheit"),
        _ => println!("please enter \"C\" or \"F\", you entered {}", current_units),
    }
}
