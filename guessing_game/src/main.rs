use std::io;

// Implement the random number generators method
use rand::Rng;

fn main() {
    println!("Guess the number!");

    //declare the secret variable
    //rand::thread_rng func gives us the particular rndom num
    // generator we are going to use
    // The gen_range method takes a range
    //  expresion as an argument & generates a random num in the range.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    println!("You guessed: {guess}");
}