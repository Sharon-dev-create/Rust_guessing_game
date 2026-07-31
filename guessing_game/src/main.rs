use std::cmp::Ordering;
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

    loop {
        println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };    
        
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    }
}