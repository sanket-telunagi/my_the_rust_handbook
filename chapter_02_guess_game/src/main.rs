use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Welcome to The Guessing Game!");
    // random number generation
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {}", secret_number);
    
    loop { 
        println!("\nPlease input your guess - ");
        
        let mut guess = String::new();
        
        // take the user input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("You guessed: {}", guess);
    
        // convert the input to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        // match the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                return;
            }
        };
    }


}
