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
        io::stdout().flush().unwrap();

        let mut choice.trim(){
            "1" => {
                let temp = read_temperature("Enter temperature in Celcius: ");
                let result = celcius_to_fahrenheit(temp);
                println!("{:.2}C = {:.2}F", temp, result);
            }
            "1" => {
                let temp = read_temperature("Enter temperature in Celcius: ");
                let result = fahrenheit_to_celcius(temp);
                println!("{:.2}C = {:.2}F", temp, result);
            }
        }
    }
}

fn read_temperature(prompt: &str) -> f64 {

}