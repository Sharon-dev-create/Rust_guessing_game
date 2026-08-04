use std::io::{self, Write};


fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    loop {
        println!("Choose convertion:");
        println!("1. Celcius -> Fahrenheit");
        println!("2. Fahrenheit -> Celcius");
        println!("3. Exit");
        print!("Enter choise (1/2/3)");

    }
}