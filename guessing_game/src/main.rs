use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let mut number_of_guesses = 8;
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess. (You have {number_of_guesses} number of guesses left.)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
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

        number_of_guesses -= 1;

        if number_of_guesses <= 0 {
            println!("No guesses left! The number was: {secret_number}");
            break;
        }
    }
}
