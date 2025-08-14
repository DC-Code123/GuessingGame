//! Main entry point for the number guessing game
//! 
//! This implements a complete guessing game where:
//! - A random number between 1.0-100.0 is generated
//! - Players guess with optional hints
//! - Supports retrying with same or new numbers

use std::io;
use std::process::exit;
use crate::utils::{game_loop, gen_rand, end_situation_handler, choose_hint, game_range_adjuster};
mod utils;

fn main() {
    // Print game introduction
    println!("Welcome to the Fantastic Number Guessing Game!");
    println!("=============================================");
    println!("A random number between 1.0 and 100.0 will be generated.");
    println!("Try to guess it with optional hints to help you!\n");

    // Track the current guessing range
    let mut range_start: f64;
    let mut range_end: f64;

    // Main game loop - runs until player chooses to quit
    'game: loop {
        // Always reset range to default at the start of a new game
        range_start = 1.0;
        range_end = 100.0;

        // Optionally allow user to adjust range before the game starts
        // (Uncomment the next two lines if you want to prompt every time)
        // let (new_start, new_end) = game_range_adjuster();
        // range_start = new_start; range_end = new_end;

        // Generate new secret number for each game session
        let secret_number = gen_rand(range_start, range_end);
        let mut attempts = 0;
        println!("\nNew game started! A secret number has been generated.");   

        // Retry loop - allows playing same number multiple times
        'retry: loop {
            // Get player's hint preference
            let hint_choice = get_hint_choice();
            
            // Show selected hint type
            choose_hint(&hint_choice, secret_number);

            // Run one full game round and get results
            let (guess_correct, new_attempts) = game_loop(secret_number, attempts);
            attempts = new_attempts;

            // Handle post-game choices
            match end_situation_handler(guess_correct, attempts) {
                1 => { // Player wants to continue
                    match get_retry_choice() {
                        1 => { // Retry same number
                            println!("\nContinuing with same number. Attempts reset.");
                            continue 'retry;
                        },
                        2 => { // Get new number (will reset range at top of 'game' loop)
                            println!("\nGenerating new number...");
                            continue 'game;
                        },
                        3 => {
                            println!("\nPreparing the game-range adjuster");
                            let (new_start, new_end) = game_range_adjuster();
                            range_start = new_start;
                            range_end = new_end;
                            println!("New guessing range set: {:.1} to {:.1}", range_start, range_end);
                            // Generate new secret number with new range, but do not reset to default until next 'game'
                            let secret_number = gen_rand(range_start, range_end);
                            attempts = 0;
                            continue 'retry;
                        },
                        0 => { // Quit game
                            println!("\nGame over in {} attempts. -quiting", attempts);
                            exit_game();
                        },
                        _ => { // Invalid input
                            println!("Invalid choice. Starting new game.");
                            continue 'game;
                        }
                    }
                },
                0 => { // Player chose to quit
                    exit_game();
                },
                _ => { // Error case
                    println!("Unexpected error. Exiting.");
                    exit(1);
                }
            }
        }
    }
}

/// Prompts player to select hint type
/// Returns:
///   String containing their choice ("1", "2", or "3")
fn get_hint_choice() -> String {
    println!("\nChoose a hint option:");
    println!("1. Easy hint (simple arithmetic)");
    println!("2. Hard hint (complex equations)");
    println!("3. No hints (I'm feeling lucky!)");
    println!("Your choice (1-3, default 3): ");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    choice
}

/// Gets player's choice after game ends
/// Returns:
///   1 = same number, 2 = new number, 0 = quit
fn get_retry_choice() -> i32 {
    println!("\nWhat would you like to do next?");
    println!("1. Try same number again");
    println!("2. Get a new random number");
    println!("3. Try again with a different guessing range");
    println!("0. Quit game");
    println!("Your choice (0-2): ");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    choice.trim().parse().unwrap_or(0) // Default to 0 (quit) on invalid input
}

/// Cleanly exits the game with farewell message
fn exit_game() {
    println!("\nThank you for playing! Goodbye!");
    exit(0);
}
