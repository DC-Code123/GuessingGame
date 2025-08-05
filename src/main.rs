// Import standard I/O module for user input/output
use std::io;
// Import Ordering for comparison operations
use std::cmp::Ordering;
// Import colored for colored terminal output
use colored::Colorize;
// Import utility functions from the utils module
use crate::utils::{ gen_rand, choose_hint,game_loop };
// Import the utils module
mod utils;

/// Main function that runs the number guessing game
fn main() {
    // Welcome message
    println!("Welcome to my guessing game!");
    println!("Please enter the maximum number for the guessing range:1-100");

    // Generate a random secret number
    // Note: gen_rand() now returns a f64 instead of i32
    let secret_number = gen_rand();
    println!("A secret number has been generated between 1 and 100.");

    // Main game loop
    loop {
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

        game_loop(&op, secret_number);

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