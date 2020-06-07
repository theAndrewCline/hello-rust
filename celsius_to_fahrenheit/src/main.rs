use std::io;

fn celsius_to_farenheit(deg: i32) -> i32 {
    return ((deg as f32 * 1.8) + 32.0) as i32;
}

fn farenheit_to_celsius(deg: i32) -> i32 {
    return ((deg as f32 - 32.0) / 1.8) as i32;
}

fn main() {
    let mut current_units = String::new();
    let mut degrees = String::new();

    println!("input your current units");

    io::stdin()
        .read_line(&mut current_units)
        .expect("Failed to read line");

    let current_units = current_units.trim();

    println!("Input number of degrees");

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    let degrees: i32 = degrees.trim().parse().expect("Please type a number");

    match current_units {
        "C" => println!(
            "you entered {} degrees celsius, which is {} farenheit",
            degrees,
            celsius_to_farenheit(degrees)
        ),
        "F" => println!(
            "you entered {} degrees farenheit, which is {} celsius",
            degrees,
            farenheit_to_celsius(degrees)
        ),
        _ => println!("please enter \"C\" or \"F\", you entered {}", current_units),
    }
}
