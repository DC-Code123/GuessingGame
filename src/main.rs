// Import standard I/O module for user input/output
use std::io;
// Import Ordering for comparison operations
// Import utility functions from the utils module
use crate::utils::{ game_loop, gen_rand, end_situation_handler };
// Import the utils module
mod utils;

fn main() {
    let mut attempt_number = 0;
    /*
        1 = true(Play again == another attempt)
        0 = false(Start again)
        2 = ERROR(Start again)
     */
    let mut played_again = 1;
    let mut secret_number: f64 = 0.0;

    if played_again == 0 || played_again == 2 {
        println!("Welcome to my guessing game!");
        println!("This is a guessing game that is pretty fanastic");
        println!("Let's begin");
        secret_number = gen_rand();
        println!("A random number has been generated");
        println!(
            "Before we continue you need to choose whether you want an easy hint or a hard one or none"
        );
        println!(
            "To choose select:\n1. Easy hint\n2. Hard hint(Math equation)\nIf you don't want a hint enter 3 or just click 'enter'"
        );

        // Read hint choice from user
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");

        loop{
            let (guess_correct, attempt_number) = game_loop(&op, secret_number, attempt_number);
            played_again = end_situation_handler(guess_correct, attempt_number);
            if played_again == 2 {
                eprintln!("Error detected!");
                break;
            }
        }
    }

    else if played_again == 1 {
        attempt_number += 1;
        println!(
            "Before we continue you need to choose whether you want an easy hint or a hard one or none"
        );
        println!(
            "To choose select:\n1. Easy hint\n2. Hard hint(Math equation)\nIf you don't want a hint enter 3 or just click 'enter'"
        );

        // Read hint choice from user
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");

        loop{
            let (guess_correct, attempt_number) = game_loop(&op, secret_number, attempt_number);
            played_again = end_situation_handler(guess_correct, attempt_number);
            if played_again == 2 {
                eprintln!("Error detected!");
                break;
            }
        }
    }
    
    else {
        println!("PROGRAMMING ERROR: CONTACT THE DEV for an error like this!!");
    }

}