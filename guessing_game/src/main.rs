use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random secret number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop allows the user to keep guessing until they get it right
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read user input from the terminal
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the original 'guess' string into an unsigned 32-bit integer.
        // .trim() removes whitespaces and the trailing newline character (\n or \r\n).
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop and end the program
            }
        }
    }
}