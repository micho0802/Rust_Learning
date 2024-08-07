use std::io;

fn main() {

    println!("Enter a number in Fahrenheit temperature: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let fahrenheit: f64 = input.trim().parse().expect("Please enter a number");

    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

