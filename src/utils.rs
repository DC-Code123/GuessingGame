// Import random number generation utilities
use rand::Rng;
// Import random selection utilities
use rand::prelude::IndexedRandom;
// Import colored for colored terminal output
use colored::Colorize;
// Import standard I/O module for user input/output
use std::io;
use std::cmp::Ordering;  // Required for `Ordering` (Less, Greater, Equal)

/// Generates a random number between 1.0 and 100.0 (inclusive)
pub fn gen_rand() -> f64 {
    let mut rng = rand::rng();
    return rng.random_range(1.0..=100.0);
}

///Handles multiple ending situations, which are:
///-The player guesses correctly but wants to play again
///-The player guesses correctly but doesn't want to play again
///-The player guesses incorrectly but wants to play again
///-The player guesses incorrectly but doesn't want to play again
pub fn end_situation_handler(is_guess_correct: i32, attempts: i32) -> i32{
    if is_guess_correct == 1 {
        println!("Goodjob you guessed it!");
        println!("You guessed It after: {} attempts", attempts);
        println!("Do you want to play again(y/yes,n/no)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().is_empty(){
            println!("{}","No input provided, please enter a number.".red().bold());
            return 2;
        } else{}
    }
}

/// Provides an easy hint for the secret number
fn easy_hint_chooser(secret_number: f64) {
    // Each tuple: (hint string, closure to compute value)
    /*
     * Define a vector of tuples containing easy hint strings and their corresponding calculations
     * Each tuple contains:
     * - A hint string with a placeholder for the computed value
     * - A closure that computes the hint value based on the secret number
     */
    let expressions: Vec<(&str, Box<dyn Fn(f64) -> f64>)> = vec![
        //every tuples format:
        //(hint(string), Box::new(|x| (the actual calculation logic))),
        ("The secret number is 5 positive steps from {}", Box::new(|x| x - 5.0)),
        ("The secret number is 45 negative steps from {}", Box::new(|x| x + 45.0)),
        ("The secret number is 10 positive steps from {}", Box::new(|x| x - 10.0)),
        ("The secret number is 20 negative steps from {}", Box::new(|x| x + 20.0)),
        ("The secret number is 15 positive steps from {}", Box::new(|x| x - 15.0)),
        ("The secret number is 30 negative steps from {}", Box::new(|x| x + 30.0)),
        ("The secret number is 12 positive steps from {}", Box::new(|x| x - 12.0)),
        ("The secret number is 18 negative steps from {}", Box::new(|x| x + 18.0)),
        ("The secret number is 22 positive steps from {}", Box::new(|x| x - 22.0)),
        ("The secret number is 28 negative steps from {}", Box::new(|x| x + 28.0)),
        ("The secret number is 14 positive steps from {}", Box::new(|x| x - 14.0)),
        (
            "A number x² is 25 positive steps from the secret number(x is {})",
            Box::new(|x| x.sqrt() - 25.0),
        ),
        (
            "A number x³ is 10 negative steps from the secret number(x is {})",
            Box::new(|x| x.cbrt() + 10.0),
        ),
        ("A number x + 3 is 5 positive steps from the secret number", Box::new(|x| x - 5.0 - 3.0)),
        (
            "A number x - 4 is 20 negative steps from the secret number",
            Box::new(|x| x + 20.0 + 4.0),
        ),
        (
            "A number x * 2 is 15 positive steps from the secret number",
            Box::new(|x| x / 2.0 - 15.0),
        ),
        (
            "A number x / 3 is 30 negative steps from the secret_number",
            Box::new(|x| x * 3.0 + 30.0),
        ),
        (
            "The secret number is 8 positive steps from the number x + 1",
            Box::new(|x| x - 8.0 - 1.0),
        ),
        (
            "A number is called p = x - 2^3 and that number is 24 negative steps from the secret_number",
            Box::new(|x| x - 8.0 + 24.0),
        ),
        (
            "The secret number is 16 negative steps from the number x + 4",
            Box::new(|x| x + 16.0 - 4.0),
        ),
        (
            "The secret number is 32 positive steps from the number x * 5",
            Box::new(|x| x / 5.0 - 32.0),
        ),
        (
            "The secret number is 6 negative steps from the number x / 6",
            Box::new(|x| x * 6.0 + 6.0),
        ),
        (
            "The secret number is 10 positive steps from the number x + 2",
            Box::new(|x| x - 10.0 - 2.0),
        ),
        (
            "The secret number is 20 negative steps from the number x - 3",
            Box::new(|x| x + 20.0 + 3.0),
        ),
        (
            "The secret number is 16 positive steps from the number x * 4",
            Box::new(|x| x / 4.0 - 16.0),
        ),
        (
            "The secret number is 32 negative steps from the number x / 5",
            Box::new(|x| x * 5.0 + 32.0),
        ),
        (
            "The secret number is 6 positive steps from the number x + 2",
            Box::new(|x| x - 6.0 - 2.0),
        ),
        (
            "The secret number is 8 negative steps from the number x - 3",
            Box::new(|x| x + 8.0 + 3.0),
        ),
        (
            "The secret number is 4 positive steps from the number x * 6",
            Box::new(|x| x / 6.0 - 4.0),
        ),
        (
            "The secret number is 10 negative steps from the number x / 7",
            Box::new(|x| x * 7.0 + 10.0),
        ),
        (
            "The secret number is 10 positive steps from the number x + 5",
            Box::new(|x| x - 10.0 - 5.0),
        ),
        (
            "The secret number is 20 negative steps from the number x - 3",
            Box::new(|x| x + 20.0 + 3.0),
        ),
        (
            "The secret number is 15 positive steps from the number x * 2",
            Box::new(|x| x / 2.0 - 15.0),
        ),
        (
            "The secret number is 30 negative steps from the number x / 3",
            Box::new(|x| x * 3.0 + 30.0),
        ),
        (
            "The secret number is 12 positive steps from the number x + 4",
            Box::new(|x| x - 12.0 - 4.0),
        ),
        (
            "The secret number is 18 negative steps from the number x - 6",
            Box::new(|x| x + 18.0 + 6.0),
        ),
        (
            "The secret number is 22 positive steps from the number x * 5",
            Box::new(|x| x / 5.0 - 22.0),
        ),
        (
            "The secret number is 28 negative steps from the number x / 4",
            Box::new(|x| x * 4.0 + 28.0),
        ),
        (
            "The secret number is 14 positive steps from the number x + 7",
            Box::new(|x| x - 14.0 - 7.0),
        ),
        (
            "The secret number is 16 negative steps from the number x - 8",
            Box::new(|x| x + 16.0 + 8.0),
        )
    ];

    // Randomly select a hint
    let mut rng = rand::rng();
    let (hint, expr) = expressions.choose(&mut rng).unwrap();

    // Display the selected hint with the computed value
    print!("Hint chosen: ");
    println!("HINT: {} = {:.2}", hint, expr(secret_number));
}

/// Provides a hard mathematical hint for the secret number
fn hard_hint_chooser(secret_number: f64) {
    // Each tuple: (hint string, closure to compute value)
    /*
     * Define a vector of tuples containing hard hint strings and their corresponding calculations
     * Each tuple contains:
     * - A complex mathematical expression as a hint string
     * - A closure that computes the hint value based on the secret number
     */
    let expressions: Vec<(&str, Box<dyn Fn(f64) -> f64>)> = vec![
        (
            "(x^2 - 3)×4 + (x^3÷2 - 7) = {}",
            Box::new(|x| (x.powi(2) - 3.0) * 4.0 + (x.powi(3) / 2.0 - 7.0)),
        ),
        (
            "(2x^3 + 5)×3 - (x^2÷4 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 5.0) * 3.0 - (x.powi(2) / 4.0 + 8.0)),
        ),
        (
            "(x^4 - 2x)×2 + (3x÷5 - 12) = {}",
            Box::new(|x| (x.powi(4) - 2.0 * x) * 2.0 + ((3.0 * x) / 5.0 - 12.0)),
        ),
        (
            "(5x^2 + 1)×6 - (x^3÷3 + 9) = {}",
            Box::new(|x| (5.0 * x.powi(2) + 1.0) * 6.0 - (x.powi(3) / 3.0 + 9.0)),
        ),
        (
            "(x^3 - 4x^2)×5 + (2x÷7 - 11) = {}",
            Box::new(|x| (x.powi(3) - 4.0 * x.powi(2)) * 5.0 + ((2.0 * x) / 7.0 - 11.0)),
        ),
        (
            "(3x^2 + 2x)×2 - (x^4÷6 + 10) = {}",
            Box::new(|x| (3.0 * x.powi(2) + 2.0 * x) * 2.0 - (x.powi(4) / 6.0 + 10.0)),
        ),
        (
            "(x^5 - x^2)×4 + (5x÷3 - 13) = {}",
            Box::new(|x| (x.powi(5) - x.powi(2)) * 4.0 + ((5.0 * x) / 3.0 - 13.0)),
        ),
        (
            "(2x^3 + 7)×3 - (x^2÷2 + 6) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 7.0) * 3.0 - (x.powi(2) / 2.0 + 6.0)),
        ),
        (
            "(x^4 - 5x)×2 + (4x÷5 - 8) = {}",
            Box::new(|x| (x.powi(4) - 5.0 * x) * 2.0 + ((4.0 * x) / 5.0 - 8.0)),
        ),
        (
            "(4x^2 + 3x)×5 - (x^3÷4 + 7) = {}",
            Box::new(|x| (4.0 * x.powi(2) + 3.0 * x) * 5.0 - (x.powi(3) / 4.0 + 7.0)),
        ),
        (
            "(x^3 - 2x^2)×6 + (3x÷2 - 9) = {}",
            Box::new(|x| (x.powi(3) - 2.0 * x.powi(2)) * 6.0 + ((3.0 * x) / 2.0 - 9.0)),
        ),
        (
            "(2x^4 + x)×2 - (x^2÷3 + 12) = {}",
            Box::new(|x| (2.0 * x.powi(4) + x) * 2.0 - (x.powi(2) / 3.0 + 12.0)),
        ),
        (
            "(x^2 + 6x)×3 + (2x^3÷5 - 14) = {}",
            Box::new(|x| (x.powi(2) + 6.0 * x) * 3.0 + ((2.0 * x.powi(3)) / 5.0 - 14.0)),
        ),
        (
            "(5x^3 - x)×4 - (x^2÷6 + 11) = {}",
            Box::new(|x| (5.0 * x.powi(3) - x) * 4.0 - (x.powi(2) / 6.0 + 11.0)),
        ),
        (
            "(x^4 + 2x^2)×2 + (3x÷7 - 10) = {}",
            Box::new(|x| (x.powi(4) + 2.0 * x.powi(2)) * 2.0 + ((3.0 * x) / 7.0 - 10.0)),
        ),
        (
            "(3x^2 - 4x)×5 - (x^3÷2 + 13) = {}",
            Box::new(|x| (3.0 * x.powi(2) - 4.0 * x) * 5.0 - (x.powi(3) / 2.0 + 13.0)),
        ),
        (
            "(x^5 + x^2)×3 + (4x÷4 - 15) = {}",
            Box::new(|x| (x.powi(5) + x.powi(2)) * 3.0 + ((4.0 * x) / 4.0 - 15.0)),
        ),
        (
            "(2x^3 - 3x)×2 - (x^2÷5 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) - 3.0 * x) * 2.0 - (x.powi(2) / 5.0 + 8.0)),
        ),
        (
            "(x^4 + 5x)×4 + (2x^3÷3 - 7) = {}",
            Box::new(|x| (x.powi(4) + 5.0 * x) * 4.0 + ((2.0 * x.powi(3)) / 3.0 - 7.0)),
        ),
        (
            "(4x^2 - x)×6 - (x^4÷2 + 9) = {}",
            Box::new(|x| (4.0 * x.powi(2) - x) * 6.0 - (x.powi(4) / 2.0 + 9.0)),
        ),
        (
            "3(x^2 - 4) + 2x - (x^3÷5) = {}",
            Box::new(|x| 3.0 * (x.powi(2) - 4.0) + 2.0 * x - x.powi(3) / 5.0),
        ),
        (
            "(2x^3 + 7x - 1)×2 - (x^2 - 3) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 7.0 * x - 1.0) * 2.0 - (x.powi(2) - 3.0)),
        ),
        (
            "(4x^4 - 2x^2)÷3 + 5x - 8 = {}",
            Box::new(|x| (4.0 * x.powi(4) - 2.0 * x.powi(2)) / 3.0 + 5.0 * x - 8.0),
        ),
        ("(x^2 + 2x)(x - 1) + 6 = {}", Box::new(|x| (x.powi(2) + 2.0 * x) * (x - 1.0) + 6.0)),
        (
            "7x^3 - 2(x^2 - 5x) + (x÷2) = {}",
            Box::new(|x| 7.0 * x.powi(3) - 2.0 * (x.powi(2) - 5.0 * x) + x / 2.0),
        ),
        (
            "(x^4 - 3x^2 + 2)÷2 + 4x = {}",
            Box::new(|x| (x.powi(4) - 3.0 * x.powi(2) + 2.0) / 2.0 + 4.0 * x),
        ),
        (
            "5(x^2 - x) - (2x^3 + 3) = {}",
            Box::new(|x| 5.0 * (x.powi(2) - x) - (2.0 * x.powi(3) + 3.0)),
        ),
        (
            "(x^3 + 4x^2)(x - 2) + 9 = {}",
            Box::new(|x| (x.powi(3) + 4.0 * x.powi(2)) * (x - 2.0) + 9.0),
        ),
        (
            "(3x^2 - 2x + 1)÷(x + 1) - 7 = {}",
            Box::new(|x| (3.0 * x.powi(2) - 2.0 * x + 1.0) / (x + 1.0) - 7.0),
        ),
        (
            "(2x^4 - x^2) + (3x - 5)^2 = {}",
            Box::new(|x| 2.0 * x.powi(4) - x.powi(2) + (3.0 * x - 5.0).powi(2)),
        ),
        (
            "6x^2 - 4x + (x^3 - 2x^2) = {}",
            Box::new(|x| 6.0 * x.powi(2) - 4.0 * x + (x.powi(3) - 2.0 * x.powi(2))),
        ),
        (
            "(x^2 + 5x + 6)÷(x + 2) + 3x = {}",
            Box::new(|x| (x.powi(2) + 5.0 * x + 6.0) / (x + 2.0) + 3.0 * x),
        ),
        (
            "2(x^3 - x^2) - (x^2 + 4x) + 10 = {}",
            Box::new(|x| 2.0 * (x.powi(3) - x.powi(2)) - (x.powi(2) + 4.0 * x) + 10.0),
        ),
        (
            "(x^4 - 2x^2 + x)×3 - 8 = {}",
            Box::new(|x| (x.powi(4) - 2.0 * x.powi(2) + x) * 3.0 - 8.0),
        ),
        ("(x^2 - 3x + 2)^2 + x = {}", Box::new(|x| (x.powi(2) - 3.0 * x + 2.0).powi(2) + x)),
        (
            "4x^3 - (x^2 + 2x - 1) + (x^4÷2) = {}",
            Box::new(|x| 4.0 * x.powi(3) - (x.powi(2) + 2.0 * x - 1.0) + x.powi(4) / 2.0),
        ),
        ("(2x^2 - x + 5)(x - 3) = {}", Box::new(|x| (2.0 * x.powi(2) - x + 5.0) * (x - 3.0))),
        (
            "(x^3 + 2x^2 - x)÷2 + 7 = {}",
            Box::new(|x| (x.powi(3) + 2.0 * x.powi(2) - x) / 2.0 + 7.0),
        ),
        (
            "(x^2 - 4x + 4) + (3x^3 - x) = {}",
            Box::new(|x| x.powi(2) - 4.0 * x + 4.0 + (3.0 * x.powi(3) - x)),
        ),
        (
            "(x^4 - x^2) - 2(x^3 + x) + 6 = {}",
            Box::new(|x| x.powi(4) - x.powi(2) - 2.0 * (x.powi(3) + x) + 6.0),
        ),
        (
            "2(x^3 - 2x^2 + 5) + 3x - 7 = {}",
            Box::new(|x| 2.0 * (x.powi(3) - 2.0 * x.powi(2) + 5.0) + 3.0 * x - 7.0),
        ),
        (
            "(x^2 + 4x + 4) - (2x^3 - x) = {}",
            Box::new(|x| x.powi(2) + 4.0 * x + 4.0 - (2.0 * x.powi(3) - x)),
        ),
        (
            "(5x^4 - 3x^2)÷2 + x - 9 = {}",
            Box::new(|x| (5.0 * x.powi(4) - 3.0 * x.powi(2)) / 2.0 + x - 9.0),
        ),
        (
            "(3x^2 - x + 1)(x - 2) + 8 = {}",
            Box::new(|x| (3.0 * x.powi(2) - x + 1.0) * (x - 2.0) + 8.0),
        ),
        (
            "4x^3 - 2(x^2 + 3x) + 5 = {}",
            Box::new(|x| 4.0 * x.powi(3) - 2.0 * (x.powi(2) + 3.0 * x) + 5.0),
        ),
        (
            "(x^4 + 2x^2 - 3)÷3 + 6x = {}",
            Box::new(|x| (x.powi(4) + 2.0 * x.powi(2) - 3.0) / 3.0 + 6.0 * x),
        ),
        (
            "7(x^2 - 2x) - (x^3 + 4) = {}",
            Box::new(|x| 7.0 * (x.powi(2) - 2.0 * x) - (x.powi(3) + 4.0)),
        ),
        (
            "(2x^3 + 5x^2)(x + 1) - 10 = {}",
            Box::new(|x| (2.0 * x.powi(3) + 5.0 * x.powi(2)) * (x + 1.0) - 10.0),
        ),
        (
            "(4x^2 - x + 2)÷(x + 2) + 3 = {}",
            Box::new(|x| (4.0 * x.powi(2) - x + 2.0) / (x + 2.0) + 3.0),
        ),
        (
            "(x^4 - 2x^2) + (2x - 3)^2 = {}",
            Box::new(|x| x.powi(4) - 2.0 * x.powi(2) + (2.0 * x - 3.0).powi(2)),
        ),
        (
            "5x^2 - 3x + (2x^3 - x^2) = {}",
            Box::new(|x| 5.0 * x.powi(2) - 3.0 * x + (2.0 * x.powi(3) - x.powi(2))),
        ),
        (
            "(x^2 + 3x + 2)÷(x + 1) + 4x = {}",
            Box::new(|x| (x.powi(2) + 3.0 * x + 2.0) / (x + 1.0) + 4.0 * x),
        ),
        (
            "3(x^3 - 2x^2) - (x^2 + 5x) + 12 = {}",
            Box::new(|x| 3.0 * (x.powi(3) - 2.0 * x.powi(2)) - (x.powi(2) + 5.0 * x) + 12.0),
        ),
        (
            "(x^4 - x^2 + 2x)×2 - 7 = {}",
            Box::new(|x| (x.powi(4) - x.powi(2) + 2.0 * x) * 2.0 - 7.0),
        ),
        ("(x^2 - 4x + 3)^2 + 2x = {}", Box::new(|x| (x.powi(2) - 4.0 * x + 3.0).powi(2) + 2.0 * x)),
        (
            "6x^3 - (2x^2 + x - 2) + (x^4÷3) = {}",
            Box::new(|x| 6.0 * x.powi(3) - (2.0 * x.powi(2) + x - 2.0) + x.powi(4) / 3.0),
        ),
        (
            "(3x^2 - 2x + 4)(x - 1) = {}",
            Box::new(|x| (3.0 * x.powi(2) - 2.0 * x + 4.0) * (x - 1.0)),
        ),
        (
            "(x^3 + x^2 - 2x)÷3 + 9 = {}",
            Box::new(|x| (x.powi(3) + x.powi(2) - 2.0 * x) / 3.0 + 9.0),
        ),
        (
            "(x^2 - 3x + 9) + (4x^3 - 2x) = {}",
            Box::new(|x| x.powi(2) - 3.0 * x + 9.0 + (4.0 * x.powi(3) - 2.0 * x)),
        ),
        (
            "(x^4 - 2x^2) - 3(x^3 + 2x) + 8 = {}",
            Box::new(|x| x.powi(4) - 2.0 * x.powi(2) - 3.0 * (x.powi(3) + 2.0 * x) + 8.0),
        ),
        (
            "2(x^2 + 5x - 1) + x^3 - 6 = {}",
            Box::new(|x| 2.0 * (x.powi(2) + 5.0 * x - 1.0) + x.powi(3) - 6.0),
        ),
        (
            "(x^3 - 4x^2 + 2)×4 - (x^2 - 1) = {}",
            Box::new(|x| (x.powi(3) - 4.0 * x.powi(2) + 2.0) * 4.0 - (x.powi(2) - 1.0)),
        ),
        (
            "(3x^4 - 2x^2)÷4 + 2x - 5 = {}",
            Box::new(|x| (3.0 * x.powi(4) - 2.0 * x.powi(2)) / 4.0 + 2.0 * x - 5.0),
        ),
        (
            "(2x^2 - x + 3)(x + 2) + 7 = {}",
            Box::new(|x| (2.0 * x.powi(2) - x + 3.0) * (x + 2.0) + 7.0),
        ),
        ("5x^3 - (x^2 + 2x) + 4 = {}", Box::new(|x| 5.0 * x.powi(3) - (x.powi(2) + 2.0 * x) + 4.0)),
        (
            "(x^4 + 3x^2 - 2)÷2 + 5x = {}",
            Box::new(|x| (x.powi(4) + 3.0 * x.powi(2) - 2.0) / 2.0 + 5.0 * x),
        ),
        (
            "8(x^2 - x) - (2x^3 + 6) = {}",
            Box::new(|x| 8.0 * (x.powi(2) - x) - (2.0 * x.powi(3) + 6.0)),
        ),
        (
            "(3x^3 + 2x^2)(x - 1) - 11 = {}",
            Box::new(|x| (3.0 * x.powi(3) + 2.0 * x.powi(2)) * (x - 1.0) - 11.0),
        ),
        (
            "(2x^2 - 3x + 1)÷(x + 3) + 6 = {}",
            Box::new(|x| (2.0 * x.powi(2) - 3.0 * x + 1.0) / (x + 3.0) + 6.0),
        ),
        (
            "(x^4 - 3x^2) + (x - 4)^2 = {}",
            Box::new(|x: f64| x.powi(4) - 3.0 * x.powi(2) + (x - 4.0).powi(2)),
        ),
        (
            "(x² - 3)×4 + (x³÷2 - 7) = {}",
            Box::new(|x| (x.powi(2) - 3.0) * 4.0 + (x.powi(3) / 2.0 - 7.0)),
        ),
        (
            "(2x³ + 5)×3 - (x²÷4 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 5.0) * 3.0 - (x.powi(2) / 4.0 + 8.0)),
        ),
        (
            "(5x² + 1)×6 - (x³÷3 + 9) = {}",
            Box::new(|x| (5.0 * x.powi(2) + 1.0) * 6.0 - (x.powi(3) / 3.0 + 9.0)),
        ),
        (
            "(3x² + 2x)×2 - (x⁴÷6 + 10) = {}",
            Box::new(|x| (3.0 * x.powi(2) + 2.0 * x) * 2.0 - (x.powi(4) / 6.0 + 10.0)),
        ),
        (
            "(x³ - 2x²)×6 + (3x÷2 - 9) = {}",
            Box::new(|x| (x.powi(3) - 2.0 * x.powi(2)) * 6.0 + ((3.0 * x) / 2.0 - 9.0)),
        ),
        (
            "3(x² - 4) + 2x - (x³÷5) = {}",
            Box::new(|x| 3.0 * (x.powi(2) - 4.0) + 2.0 * x - x.powi(3) / 5.0),
        ),
        ("(x² + 2x)(x - 1) + 6 = {}", Box::new(|x| (x.powi(2) + 2.0 * x) * (x - 1.0) + 6.0)),
        (
            "7x³ - 2(x² - 5x) + (x÷2) = {}",
            Box::new(|x| 7.0 * x.powi(3) - 2.0 * (x.powi(2) - 5.0 * x) + x / 2.0),
        ),
        // Basic linear
        ("2x + 5 = {}", Box::new(|x| 2.0 * x + 5.0)),
        ("3x - 7 = {}", Box::new(|x| 3.0 * x - 7.0)),
        // Quadratic
        ("x² + 3x - 2 = {}", Box::new(|x| x.powi(2) + 3.0 * x - 2.0)),
        ("2x² - 5x + 1 = {}", Box::new(|x| 2.0 * x.powi(2) - 5.0 * x + 1.0)),
        // Cubic
        ("x³ + 2x - 1 = {}", Box::new(|x| x.powi(3) + 2.0 * x - 1.0)),
        ("2x³ - x² + 5 = {}", Box::new(|x| 2.0 * x.powi(3) - x.powi(2) + 5.0)),
        // Mixed operations
        ("x(x + 1) = {}", Box::new(|x| x * (x + 1.0))),
        ("(x + 2)(x - 3) = {}", Box::new(|x| (x + 2.0) * (x - 3.0))),
        ("2(x + 3) = {}", Box::new(|x| 2.0 * (x + 3.0))),
        ("3(x² - 2x) = {}", Box::new(|x| 3.0 * (x.powi(2) - 2.0 * x))),
        // Simple fractions
        ("x/2 + 3 = {}", Box::new(|x| x / 2.0 + 3.0)),
        ("(x + 1)/3 = {}", Box::new(|x| (x + 1.0) / 3.0))
    ];

    // Randomly select a hint
    let mut rng = rand::rng();
    let (hint, expr) = expressions.choose(&mut rng).unwrap();

    // Display the selected hint with the computed value
    print!("Hint chosen: ");
    println!("HINT: {} = {:.2}", hint, expr(secret_number));
}

/// Handles hint selection based on user input
/// # Arguments
/// * `op` - The user's input for hint selection (1 for easy, 2 for hard, 3 or empty for none)
/// * `secret_number` - The secret number to generate hints for
pub fn choose_hint(op: &str, secret_number: f64) {
    match op.trim().parse::<i32>() {
        Ok(num) =>
            match num {
                1 => {
                    println!("You chose an easy hint!!");
                    println!("Here you go!!");
                    easy_hint_chooser(secret_number);
                }
                2 => {
                    println!(
                        "You chose a hard hint!!\nThe hard hints are mathematical, so be prepared with a calculator"
                    );
                    println!("Here you go!!");
                    hard_hint_chooser(secret_number);
                }
                3 => {
                    println!("Good luck!!\nProceeding without any hints");
                }
                _ => {
                    println!("Invalid choice: {}. Please enter 1, 2, or 3.", num);
                    println!("Proceeding without hints.");
                }
            }
        Err(_) => {
            println!("Invalid input. Please enter a number (1, 2, or 3).");
            println!("Proceeding without hints.");
        }
    }
}

pub fn game_loop(op: &str,secret: f64, mut attempts: i32) -> (i32,i32) {
    loop{
        attempts += 1;
        // Provide hint based on user choice
        choose_hint(&op, secret);

        // Prompt for user guess
        println!("Please enter the maximum number for the guessing range:1-100");
        println!("Please enter your guess: ");

        // Read user's guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Handle empty input
        if guess.trim().is_empty() {
            println!("{}","No input provided, please enter a number.".red().bold());
            continue;
        }

        // Parse guess into a float
        let guess: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{:?}", "Invalid input, please enter a number.".red().bold());
                continue;
            }
        };

        // Validate guess is within range
        if guess < 1.0 || guess > 100.0 {
            println!("{}","Please enter a number between 1 and 100.".red().bold());
            continue;
        }

        // Display user's guess
        println!("You inputted: {}", guess);

        // Check if guess is correct
        if (guess - secret).abs() < f64::EPSILON {
            println!("Congratulations! You guessed the number!");
        } else {
            println!("Sorry. Try again!");
        }

        // Compare guess with secret number and provide feedback
        match guess.partial_cmp(&secret).unwrap() {
            Ordering::Less => {
                println!(
                    "{}",
                    "Too small!".red(),
                );
                let guess_correct = 0;
                return (guess_correct, attempts);
            }
            Ordering::Greater => {
                println!(
                    "{}",
                    "Too big!".red().bold(),
                );
                let guess_correct = 0;
                return (guess_correct, attempts);
            }
            Ordering::Equal => {
                println!("{}", "You guessed it!".green().bold());
                let guess_correct = 1;
                return (guess_correct, attempts);
            }
        }
    }
}

