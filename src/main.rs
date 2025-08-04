// Import standard I/O module for user input/output
use std::io;
// Import Ordering for comparison operations
use std::cmp::Ordering;
// Import colored for colored terminal output
use colored::Colorize;
// Import utility functions from the utils module
use crate::utils::{ gen_rand, choose_hint };
// Import the utils module
mod utils;

/// Main function that runs the number guessing game
fn main() {
    // Welcome message
    println!("Welcome to my guessing game!");
    println!("Please enter the maximum number for the guessing range:1-100");

    let secret_number = gen_rand();

    // Main game loop
    loop {
        // Generate a random secret number
        // Note: gen_rand() now returns a f64 instead of i32

        println!("A secret number has been generated between 1 and 100.");

        // Prompt for hint selection
        println!(
            "Before we continue you need to choose whether you want an easy hint or a hard one"
        );
        println!(
            "To choose select:\n1.Easy hint\n2.Hard hint(Math equation)\nIf you don't want a hint enter 3 or just click 'enter'"
        );

        // Read hint choice from user
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");

        // Provide hint based on user choice
        choose_hint(&op, secret_number);

        // Prompt for user guess
        println!("Please enter your guess: ");

        // Read user's guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Handle empty input
        if guess.trim().is_empty() {
            println!({},"No input provided, please enter a number.".red().bold());
            continue;
        }

        // Parse guess into a float
        let guess: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!({},"Invalid input, please enter a number.".red().bold());
                continue;
            }
        };

        // Validate guess is within range
        if guess < 1.0 || guess > 100.0 {
            println!({},"Please enter a number between 1 and 100.".red().bold());
            continue;
        }

        // Display user's guess
        println!("You inputted: {}", guess);

        // Check if guess is correct
        if (guess - secret_number).abs() < f64::EPSILON {
            println!("Congratulations! You guessed the number!");
        } else {
            println!({},"Sorry, the secret number was: {}", secret_number.red().bold());
        }

        // Compare guess with secret number and provide feedback
        match guess.partial_cmp(&secret_number).unwrap() {
            Ordering::Less => {
                println!(
                    "{} {}",
                    "Too small!".red(),
                    format!("The number was: {}", secret_number).yellow(),
                );
            }
            Ordering::Greater => {
                println!(
                    "{} {}",
                    "Too big!".red().bold(),
                    format!("The number was: {}", secret_number).yellow(),
                );
            }
            Ordering::Equal => {
                println!("{}", "You guessed it!".green().bold());
                println!("The secret number was: {}", secret_number);
                break;
            }
        }

        // Ask if the user wants to play again
        println!("Do you want to play again? (yes=y/no=n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");

        // Exit game if user doesn't want to continue
        if play_again.trim().to_lowercase() != "yes" && play_again.trim().to_lowercase() != "y" {
            println!("Thank you for playing!");
            break;
        }
    }
}
