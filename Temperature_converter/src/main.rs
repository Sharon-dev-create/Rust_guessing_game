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
        print!("Enter choice (1/2/3)");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let temp = read_temperature("Enter temperature in Celcius: ");
                let result = celcius_to_fahrenheit(temp);
                println!("{:.2}C = {:.2}F", temp, result);
            }
            "2" => {
                let temp = read_temperature("Enter temperature in Fahrenheit: ");
                let result = fahrenheit_to_celcius(temp);
                println!("{:.2}C = {:.2}F", temp, result);
            }
            "3" => {
                println!("Goodbye.");
                break;
            }
            _ => {
                println!("Invalid choice. Enter 1, 2 or 3");
            }
        }
    }
}

fn read_temperature(prompt: &str) -> f64 {
       loop {
        println!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); //Pauses the program and waits for the
        //user to press enter. Whatever they types goes into input.

        match input.trim().parse::<f64>() {
           Ok(val) => return val,
           Err(_) => println!(" Invalid input. Enter a number (e.g 100 or -50.5)"),
        };
       }
}