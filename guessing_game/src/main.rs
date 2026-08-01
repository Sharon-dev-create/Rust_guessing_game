use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main(){
    println!("guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

    }
}