use std::io::{self, Write};


fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0;
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0;
}

fn main() {
    loop {
        println("Choose convertion")
    }
}