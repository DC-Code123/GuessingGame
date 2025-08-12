//! Game utilities module
//!
//! Contains all core game logic components:
//! - Number generation
//! - Hint systems
//! - Game state management

use rand::Rng;
use rand::seq::IndexedRandom;
use colored::Colorize;
use std::io;
use std::cmp::Ordering;

/// Generates random number between 1.0 and 100.0
/// Uses thread-local random number generator
pub fn gen_rand() -> f64 {
    rand::rng().random_range(1.0..=100.0)
}

/// Handles game end scenarios
/// Parameters:
///   is_guess_correct: bool - whether player guessed correctly
///   attempts: i32 - number of attempts made
/// Returns:
///   1 to continue, 0 to quit
pub fn end_situation_handler(is_guess_correct: bool, attempts: i32) -> i32 {
    // Show appropriate win/lose message
    if is_guess_correct {
        println!("{}", format!("You won in {} attempts!", attempts).green().bold());
    } else {
        println!("{}", format!("Unfortunately, you lost on the {} attempt(s).", attempts).red().bold());
    }
    
    // Prompt for next action
    println!("\nWould you like to play again?");
    println!("1 = Yes, 0 = No: ");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    // Parse input, default to quit on error
    choice.trim().parse().unwrap_or(0)
}

/// Provides easy hints using simple arithmetic
/// Parameters:
///   secret_number: f64 - the target number to hint toward
fn easy_hint_chooser(secret_number: f64) {
    // Collection of possible hint formats and their calculations
    let expressions: Vec<(&str, Box<dyn Fn(f64) -> f64>)> =  vec![
    //every tuples format:
    //(hint(string), Box::new(|x| (the actual calculation logic))),
    // Each tuple: (hint string, closure to compute value)
    /*
     * Define a vector of tuples containing easy hint strings and their corresponding calculations
     * Each tuple contains:
     * - A hint string with a placeholder for the computed value
     * - A closure that computes the hint value based on the secret number
     */
        ("If you add 7 to the secret number, you get {:.1}", Box::new(|x| x + 7.0)), // Real world: Simple addition
        ("The secret number is your age if you were born in the year {:.0}", Box::new(|x| 2024.0 - x)), // Real world: Age calculation
        ("A pizza costs $12 and you have ${:.1}, the secret number is how many pizzas you can buy", Box::new(|x| x / 12.0)), // Real world: Division problem
        ("The secret number is the temperature in Celsius when it's {:.1}°F", Box::new(|x| (x - 32.0) * 5.0 / 9.0)), // Real world: Temperature conversion
        ("You have {:.1} quarters, the secret number is how many dollars that equals", Box::new(|x| x / 4.0)), // Real world: Money conversion
        ("The secret number is 15 positive steps from {:.1}", Box::new(|x| x - 15.0)), // Keep original: Direct steps
        ("If you walk {:.1} miles at 3 mph, the secret number is how many hours it takes", Box::new(|x| x / 3.0)), // Real world: Speed/distance/time
        ("The secret number is 18 negative steps from {:.1}", Box::new(|x| x + 18.0)), // Keep original: Direct steps
        ("A rectangle has area {:.1} and width 4, the secret number is its length", Box::new(|x| x / 4.0)), // Real world: Geometry
        ("The secret number is how many dozens are in {:.1} items", Box::new(|x| x / 12.0)), // Real world: Counting
        ("If gas costs $3 per gallon and you spend ${:.1}, the secret number is gallons bought", Box::new(|x| x / 3.0)), // Real world: Gas purchase
        (
            "The secret number squared equals the number of days in {:.1} years (non-leap)",
            Box::new(|x| (x * 365.0).sqrt()), // Real world: Time calculation with squares
        ),
        (
            "The secret number cubed is the volume of a cube with side length {:.1}",
            Box::new(|x| x.cbrt()), // Real world: Volume calculation
        ),
        ("You score the secret number points per game and play 3 games for {:.1} total points", Box::new(|x| x / 3.0)), // Real world: Sports scoring
        (
            "The secret number is your hourly wage if you earn ${:.1} for 4 hours of work",
            Box::new(|x| x / 4.0), // Real world: Wage calculation
        ),
        (
            "A car travels the secret number mph for 2 hours to go {:.1} miles",
            Box::new(|x| x / 2.0), // Real world: Speed calculation
        ),
        (
            "The secret number is 30 negative steps from {:.1}",
            Box::new(|x| x + 30.0), // Keep original: Direct steps
        ),
        (
            "You buy the secret number apples at $0.50 each and spend ${:.1}",
            Box::new(|x| x / 0.5), // Real world: Shopping
        ),
        (
            "The secret number is the side length of a square with perimeter {:.1}",
            Box::new(|x| x / 4.0), // Real world: Geometry
        ),
        (
            "The secret number is 16 negative steps from the number S + 4",
            Box::new(|x| x + 16.0 - 4.0), // Simplified formula: S = N - 12
        ),
        (
            "The secret number is 32 positive steps from the number S * 5",
            Box::new(|x| x / 5.0 - 32.0), // Simplified formula: S = 5(N + 32)
        ),
        (
            "The secret number is 6 negative steps from the number S / 6",
            Box::new(|x| x * 6.0 + 6.0), // Simplified formula: S = (N - 6) / 6
        ),
        (
            "The secret number is 10 positive steps from the number S + 2",
            Box::new(|x| x - 10.0 - 2.0), // Simplified formula: S = N + 12
        ),
        (
            "The secret number is 20 negative steps from the number S - 3",
            Box::new(|x| x + 20.0 + 3.0), // Simplified formula: S = N - 23
        ),
        (
            "The secret number is 16 positive steps from the number S * 4",
            Box::new(|x| x / 4.0 - 16.0), // Simplified formula: S = 4(N + 16)
        ),
        (
            "The secret number is 32 negative steps from the number S / 5",
            Box::new(|x| x * 5.0 + 32.0), // Simplified formula: S = (N - 32) / 5
        ),
        (
            "The secret number is 6 positive steps from the number S + 2",
            Box::new(|x| x - 6.0 - 2.0), // Simplified formula: S = N + 8
        ),
        (
            "The secret number is 8 negative steps from the number S - 3",
            Box::new(|x| x + 8.0 + 3.0), // Simplified formula: S = N - 11
        ),
        (
            "The secret number is 4 positive steps from the number S * 6",
            Box::new(|x| x / 6.0 - 4.0), // Simplified formula: S = 6(N + 4)
        ),
        (
            "The secret number is 10 negative steps from the number S / 7",
            Box::new(|x| x * 7.0 + 10.0), // Simplified formula: S = (N - 10) / 7
        ),
        (
            "The secret number is 10 positive steps from the number S + 5",
            Box::new(|x| x - 10.0 - 5.0), // Simplified formula: S = N + 15
        ),
        (
            "The secret number is 20 negative steps from the number S - 3",
            Box::new(|x| x + 20.0 + 3.0), // Simplified formula: S = N - 23
        ),
        (
            "The secret number is 15 positive steps from the number S * 2",
            Box::new(|x| x / 2.0 - 15.0), // Simplified formula: S = 2(N + 15)
        ),
        (
            "The secret number is 30 negative steps from the number S / 3",
            Box::new(|x| x * 3.0 + 30.0), // Simplified formula: S = (N - 30) / 3
        ),
        (
            "The secret number is 12 positive steps from the number S + 4",
            Box::new(|x| x - 12.0 - 4.0), // Simplified formula: S = N + 16
        ),
        (
            "The secret number is 18 negative steps from the number S - 6",
            Box::new(|x| x + 18.0 + 6.0), // Simplified formula: S = N - 24
        ),
        (
            "The secret number is 22 positive steps from the number S * 5",
            Box::new(|x| x / 5.0 - 22.0), // Simplified formula: S = 5(N + 22)
        ),
        (
            "The secret number is 28 negative steps from the number S / 4",
            Box::new(|x| x * 4.0 + 28.0), // Simplified formula: S = (N - 28) / 4
        ),
        (
            "The secret number is 14 positive steps from the number S + 7",
            Box::new(|x| x - 14.0 - 7.0), // Simplified formula: S = N + 21
        ),
        (
            "The secret number is 16 negative steps from the number S - 8",
            Box::new(|x| x + 16.0 + 8.0), // Simplified formula: S = N - 24
        )
    ];
    
    // Randomly select and display one hint
    let mut rng = rand::rng();
    let (hint, expr) = expressions.choose(&mut rng).unwrap();
    println!("{}: {} = {:.2}", "Easy Hint".blue(), hint, expr(secret_number));
}

/// Provides complex mathematical hints
/// Parameters:
///   secret_number: f64 - the target number to hint toward
fn hard_hint_chooser(secret_number: f64) {
      // Each tuple: (hint string, closure to compute value)
    /*
     * Define a vector of tuples containing hard hint strings and their corresponding calculations
     * Each tuple contains:
     * - A complex mathematical expression as a hint string
     * - A closure that computes the hint value based on the secret number
     */
    // Collection of complex equation hints
    let expressions: Vec<(&str, Box<dyn Fn(f64) -> f64>)> = vec![
        (
            "(S^2 - 3)×4 + (S^3÷2 - 7) = {}",
            Box::new(|x| (x.powi(2) - 3.0) * 4.0 + (x.powi(3) / 2.0 - 7.0)), // Simplified formula: N = 4(S² - 3) + (S³ / 2) - 7
        ),
        (
            "(2S^3 + 5)×3 - (S^2÷4 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 5.0) * 3.0 - (x.powi(2) / 4.0 + 8.0)), // Simplified formula: N = 3(2S³ + 5) - (S² / 4 + 8)
        ),
        (
            "(S^4 - 2S)×2 + (3S÷5 - 12) = {}",
            Box::new(|x| (x.powi(4) - 2.0 * x) * 2.0 + ((3.0 * x) / 5.0 - 12.0)), // Simplified formula: N = 2(S⁴ - 2S) + (3S / 5 - 12)
        ),
        (
            "(5S^2 + 1)×6 - (S^3÷3 + 9) = {}",
            Box::new(|x| (5.0 * x.powi(2) + 1.0) * 6.0 - (x.powi(3) / 3.0 + 9.0)), // Simplified formula: N = 6(5S² + 1) - (S³ / 3 + 9)
        ),
        (
            "(S^3 - 4S^2)×5 + (2S÷7 - 11) = {}",
            Box::new(|x| (x.powi(3) - 4.0 * x.powi(2)) * 5.0 + ((2.0 * x) / 7.0 - 11.0)), // Simplified formula: N = 5(S³ - 4S²) + (2S / 7 - 11)
        ),
        (
            "(3S^2 + 2S)×2 - (S^4÷6 + 10) = {}",
            Box::new(|x| (3.0 * x.powi(2) + 2.0 * x) * 2.0 - (x.powi(4) / 6.0 + 10.0)), // Simplified formula: N = 2(3S² + 2S) - (S⁴ / 6 + 10)
        ),
        (
            "(S^5 - S^2)×4 + (5S÷3 - 13) = {}",
            Box::new(|x| (x.powi(5) - x.powi(2)) * 4.0 + ((5.0 * x) / 3.0 - 13.0)), // Simplified formula: N = 4(S⁵ - S²) + (5S / 3 - 13)
        ),
        (
            "(2S^3 + 7)×3 - (S^2÷2 + 6) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 7.0) * 3.0 - (x.powi(2) / 2.0 + 6.0)), // Simplified formula: N = 3(2S³ + 7) - (S² / 2 + 6)
        ),
        (
            "(S^4 - 5S)×2 + (4S÷5 - 8) = {}",
            Box::new(|x| (x.powi(4) - 5.0 * x) * 2.0 + ((4.0 * x) / 5.0 - 8.0)), // Simplified formula: N = 2(S⁴ - 5S) + (4S / 5 - 8)
        ),
        (
            "(4S^2 + 3S)×5 - (S^3÷4 + 7) = {}",
            Box::new(|x| (4.0 * x.powi(2) + 3.0 * x) * 5.0 - (x.powi(3) / 4.0 + 7.0)), // Simplified formula: N = 5(4S² + 3S) - (S³ / 4 + 7)
        ),
        (
            "(S^3 - 2S^2)×6 + (3S÷2 - 9) = {}",
            Box::new(|x| (x.powi(3) - 2.0 * x.powi(2)) * 6.0 + ((3.0 * x) / 2.0 - 9.0)), // Simplified formula: N = 6(S³ - 2S²) + (3S / 2 - 9)
        ),
        (
            "(2S^4 + S)×2 - (S^2÷3 + 12) = {}",
            Box::new(|x| (2.0 * x.powi(4) + x) * 2.0 - (x.powi(2) / 3.0 + 12.0)), // Simplified formula: N = 2(2S⁴ + S) - (S² / 3 + 12)
        ),
        (
            "(S^2 + 6S)×3 + (2S^3÷5 - 14) = {}",
            Box::new(|x| (x.powi(2) + 6.0 * x) * 3.0 + ((2.0 * x.powi(3)) / 5.0 - 14.0)), // Simplified formula: N = 3(S² + 6S) + (2S³ / 5 - 14)
        ),
        (
            "(5S^3 - S)×4 - (S^2÷6 + 11) = {}",
            Box::new(|x| (5.0 * x.powi(3) - x) * 4.0 - (x.powi(2) / 6.0 + 11.0)), // Simplified formula: N = 4(5S³ - S) - (S² / 6 + 11)
        ),
        (
            "(S^4 + 2S^2)×2 + (3S÷7 - 10) = {}",
            Box::new(|x| (x.powi(4) + 2.0 * x.powi(2)) * 2.0 + ((3.0 * x) / 7.0 - 10.0)), // Simplified formula: N = 2(S⁴ + 2S²) + (3S / 7 - 10)
        ),
        (
            "(3S^2 - 4S)×5 - (S^3÷2 + 13) = {}",
            Box::new(|x| (3.0 * x.powi(2) - 4.0 * x) * 5.0 - (x.powi(3) / 2.0 + 13.0)), // Simplified formula: N = 5(3S² - 4S) - (S³ / 2 + 13)
        ),
        (
            "(S^5 + S^2)×3 + (4S÷4 - 15) = {}",
            Box::new(|x| (x.powi(5) + x.powi(2)) * 3.0 + ((4.0 * x) / 4.0 - 15.0)), // Simplified formula: N = 3(S⁵ + S²) + (S - 15)
        ),
        (
            "(2S^3 - 3S)×2 - (S^2÷5 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) - 3.0 * x) * 2.0 - (x.powi(2) / 5.0 + 8.0)), // Simplified formula: N = 2(2S³ - 3S) - (S² / 5 + 8)
        ),
        (
            "(S^4 + 5S)×4 + (2S^3÷3 - 7) = {}",
            Box::new(|x| (x.powi(4) + 5.0 * x) * 4.0 + ((2.0 * x.powi(3)) / 3.0 - 7.0)), // Simplified formula: N = 4(S⁴ + 5S) + (2S³ / 3 - 7)
        ),
        (
            "(4S^2 - S)×6 - (S^4÷2 + 9) = {}",
            Box::new(|x| (4.0 * x.powi(2) - x) * 6.0 - (x.powi(4) / 2.0 + 9.0)), // Simplified formula: N = 6(4S² - S) - (S⁴ / 2 + 9)
        ),
        (
            "3(S^2 - 4) + 2S - (S^3÷5) = {}",
            Box::new(|x| 3.0 * (x.powi(2) - 4.0) + 2.0 * x - x.powi(3) / 5.0), // Simplified formula: N = 3(S² - 4) + 2S - S³ / 5
        ),
        (
            "(2S^3 + 7S - 1)×2 - (S^2 - 3) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 7.0 * x - 1.0) * 2.0 - (x.powi(2) - 3.0)), // Simplified formula: N = 2(2S³ + 7S - 1) - (S² - 3)
        ),
        (
            "(4S^4 - 2S^2)÷3 + 5S - 8 = {}",
            Box::new(|x| (4.0 * x.powi(4) - 2.0 * x.powi(2)) / 3.0 + 5.0 * x - 8.0), // Simplified formula: N = (4S⁴ - 2S²) / 3 + 5S - 8
        ),
        ("(S^2 + 2S)(S - 1) + 6 = {}", Box::new(|x| (x.powi(2) + 2.0 * x) * (x - 1.0) + 6.0)), // Simplified formula: N = (S² + 2S)(S - 1) + 6
        (
            "7S^3 - 2(S^2 - 5S) + (S÷2) = {}",
            Box::new(|x| 7.0 * x.powi(3) - 2.0 * (x.powi(2) - 5.0 * x) + x / 2.0), // Simplified formula: N = 7S³ - 2(S² - 5S) + S/2
        ),
        (
            "(S^4 - 3S^2 + 2)÷2 + 4S = {}",
            Box::new(|x| (x.powi(4) - 3.0 * x.powi(2) + 2.0) / 2.0 + 4.0 * x), // Simplified formula: N = (S⁴ - 3S² + 2) / 2 + 4S
        ),
        (
            "5(S^2 - S) - (2S^3 + 3) = {}",
            Box::new(|x| 5.0 * (x.powi(2) - x) - (2.0 * x.powi(3) + 3.0)), // Simplified formula: N = 5(S² - S) - (2S³ + 3)
        ),
        (
            "(S^3 + 4S^2)(S - 2) + 9 = {}",
            Box::new(|x| (x.powi(3) + 4.0 * x.powi(2)) * (x - 2.0) + 9.0), // Simplified formula: N = (S³ + 4S²)(S - 2) + 9
        ),
        (
            "(3S^2 - 2S + 1)÷(S + 1) - 7 = {}",
            Box::new(|x| (3.0 * x.powi(2) - 2.0 * x + 1.0) / (x + 1.0) - 7.0), // Simplified formula: N = (3S² - 2S + 1) / (S + 1) - 7
        ),
        (
            "(2S^4 - S^2) + (3S - 5)^2 = {}",
            Box::new(|x| 2.0 * x.powi(4) - x.powi(2) + (3.0 * x - 5.0).powi(2)), // Simplified formula: N = (2S⁴ - S²) + (3S - 5)²
        ),
        (
            "6S^2 - 4S + (S^3 - 2S^2) = {}",
            Box::new(|x| 6.0 * x.powi(2) - 4.0 * x + (x.powi(3) - 2.0 * x.powi(2))), // Simplified formula: N = 6S² - 4S + (S³ - 2S²)
        ),
        (
            "(S^2 + 5S + 6)÷(S + 2) + 3S = {}",
            Box::new(|x| (x.powi(2) + 5.0 * x + 6.0) / (x + 2.0) + 3.0 * x), // Simplified formula: N = (S² + 5S + 6) / (S + 2) + 3S
        ),
        (
            "2(S^3 - S^2) - (S^2 + 4S) + 10 = {}",
            Box::new(|x| 2.0 * (x.powi(3) - x.powi(2)) - (x.powi(2) + 4.0 * x) + 10.0), // Simplified formula: N = 2(S³ - S²) - (S² + 4S) + 10
        ),
        (
            "(S^4 - 2S^2 + S)×3 - 8 = {}",
            Box::new(|x| (x.powi(4) - 2.0 * x.powi(2) + x) * 3.0 - 8.0), // Simplified formula: N = 3(S⁴ - 2S² + S) - 8
        ),
        ("(S^2 - 3S + 2)^2 + S = {}", Box::new(|x| (x.powi(2) - 3.0 * x + 2.0).powi(2) + x)), // Simplified formula: N = (S² - 3S + 2)² + S
        (
            "4S^3 - (S^2 + 2S - 1) + (S^4÷2) = {}",
            Box::new(|x| 4.0 * x.powi(3) - (x.powi(2) + 2.0 * x - 1.0) + x.powi(4) / 2.0), // Simplified formula: N = 4S³ - (S² + 2S - 1) + S⁴ / 2
        ),
        ("(2S^2 - S + 5)(S - 3) = {}", Box::new(|x| (2.0 * x.powi(2) - x + 5.0) * (x - 3.0))), // Simplified formula: N = (2S² - S + 5)(S - 3)
        (
            "(S^3 + 2S^2 - S)÷2 + 7 = {}",
            Box::new(|x| (x.powi(3) + 2.0 * x.powi(2) - x) / 2.0 + 7.0), // Simplified formula: N = (S³ + 2S² - S) / 2 + 7
        ),
        (
            "(S^2 - 4S + 4) + (3S^3 - S) = {}",
            Box::new(|x| x.powi(2) - 4.0 * x + 4.0 + (3.0 * x.powi(3) - x)), // Simplified formula: N = (S² - 4S + 4) + (3S³ - S)
        ),
        (
            "(S^4 - S^2) - 2(S^3 + S) + 6 = {}",
            Box::new(|x| x.powi(4) - x.powi(2) - 2.0 * (x.powi(3) + 2.0 * x) + 6.0), // Simplified formula: N = (S⁴ - S²) - 2(S³ + 2S) + 6
        ),
        (
            "2(S^3 - 2S^2 + 5) + 3S - 7 = {}",
            Box::new(|x| 2.0 * (x.powi(3) - 2.0 * x.powi(2) + 5.0) + 3.0 * x - 7.0), // Simplified formula: N = 2(S³ - 2S² + 5) + 3S - 7
        ),
        (
            "(S^3 - 4S^2 + 2)×4 - (S^2 - 1) = {}",
            Box::new(|x| (x.powi(3) - 4.0 * x.powi(2) + 2.0) * 4.0 - (x.powi(2) - 1.0)), // Simplified formula: N = 4(S³ - 4S² + 2) - (S² - 1)
        ),
        (
            "(3S^4 - 2S^2)÷4 + 2S - 5 = {}",
            Box::new(|x| (3.0 * x.powi(4) - 2.0 * x.powi(2)) / 4.0 + 2.0 * x - 5.0), // Simplified formula: N = (3S⁴ - 2S²) / 4 + 2S - 5
        ),
        (
            "(2S^2 - S + 3)(S + 2) + 7 = {}",
            Box::new(|x| (2.0 * x.powi(2) - x + 3.0) * (x + 2.0) + 7.0), // Simplified formula: N = (2S² - S + 3)(S + 2) + 7
        ),
        ("5S^3 - (S^2 + 2S) + 4 = {}", Box::new(|x| 5.0 * x.powi(3) - (x.powi(2) + 2.0 * x) + 4.0)), // Simplified formula: N = 5S³ - (S² + 2S) + 4
        (
            "(S^4 + 3S^2 - 2)÷2 + 5S = {}",
            Box::new(|x| (x.powi(4) + 3.0 * x.powi(2) - 2.0) / 2.0 + 5.0 * x), // Simplified formula: N = (S⁴ + 3S² - 2) / 2 + 5S
        ),
        (
            "8(S^2 - S) - (2S^3 + 6) = {}",
            Box::new(|x| 8.0 * (x.powi(2) - x) - (2.0 * x.powi(3) + 6.0)), // Simplified formula: N = 8(S² - S) - (2S³ + 6)
        ),
        (
            "(3S^3 + 2S^2)(S - 1) - 11 = {}",
            Box::new(|x| (3.0 * x.powi(3) + 2.0 * x.powi(2)) * (x - 1.0) - 11.0), // Simplified formula: N = (3S³ + 2S²)(S - 1) - 11
        ),
        (
            "(2S^2 - 3S + 1)÷(S + 3) + 6 = {}",
            Box::new(|x| (2.0 * x.powi(2) - 3.0 * x + 1.0) / (x + 3.0) + 6.0), // Simplified formula: N = (2S² - 3S + 1) / (S + 3) + 6
        ),
        (
            "(S^4 - 3S^2) + (S - 4)^2 = {}",
            Box::new(|x: f64| x.powi(4) - 3.0 * x.powi(2) + (x - 4.0).powi(2)), // Simplified formula: N = (S⁴ - 3S²) + (S - 4)²
        ),
        (
            "(S² - 3)×4 + (S³÷2 - 7) = {}",
            Box::new(|x| (x.powi(2) - 3.0) * 4.0 + (x.powi(3) / 2.0 - 7.0)), // Simplified formula: N = 4(S² - 3) + (S³ / 2) - 7
        ),
        (
            "(2S³ + 5)×3 - (S²÷4 + 8) = {}",
            Box::new(|x| (2.0 * x.powi(3) + 5.0) * 3.0 - (x.powi(2) / 4.0 + 8.0)), // Simplified formula: N = 3(2S³ + 5) - (S² / 4 + 8)
        ),
        (
            "(5S² + 1)×6 - (S³÷3 + 9) = {}",
            Box::new(|x| (5.0 * x.powi(2) + 1.0) * 6.0 - (x.powi(3) / 3.0 + 9.0)), // Simplified formula: N = 6(5S² + 1) - (S³ / 3 + 9)
        ),
        (
            "(3S² + 2S)×2 - (S⁴÷6 + 10) = {}",
            Box::new(|x| (3.0 * x.powi(2) + 2.0 * x) * 2.0 - (x.powi(4) / 6.0 + 10.0)), // Simplified formula: N = 2(3S² + 2S) - (S⁴ / 6 + 10)
        ),
        (
            "(S³ - 2S²)×6 + (3S÷2 - 9) = {}",
            Box::new(|x| (x.powi(3) - 2.0 * x.powi(2)) * 6.0 + ((3.0 * x) / 2.0 - 9.0)), // Simplified formula: N = 6(S³ - 2S²) + (3S / 2 - 9)
        ),
        (
            "3(S² - 4) + 2S - (S³÷5) = {}",
            Box::new(|x| 3.0 * (x.powi(2) - 4.0) + 2.0 * x - x.powi(3) / 5.0), // Simplified formula: N = 3(S² - 4) + 2S - S³ / 5
        ),
        (" (S² + 2S)(S - 1) + 6 = {}", Box::new(|x| (x.powi(2) + 2.0 * x) * (x - 1.0) + 6.0)), // Simplified formula: N = (S² + 2S)(S - 1) + 6
        (
            "7S³ - 2(S² - 5S) + (S÷2) = {}",
            Box::new(|x| 7.0 * x.powi(3) - 2.0 * (x.powi(2) - 5.0 * x) + x / 2.0), // Simplified formula: N = 7S³ - 2(S² - 5S) + S/2
        ),
        // Basic linear
        ("2S + 5 = {}", Box::new(|x| 2.0 * x + 5.0)), // Simplified formula: N = 2S + 5
        ("3S - 7 = {}", Box::new(|x| 3.0 * x - 7.0)), // Simplified formula: N = 3S - 7
        // Quadratic
        ("S² + 3S - 2 = {}", Box::new(|x| x.powi(2) + 3.0 * x - 2.0)), // Simplified formula: N = S² + 3S - 2
        ("2S² - 5S + 1 = {}", Box::new(|x| 2.0 * x.powi(2) - 5.0 * x + 1.0)), // Simplified formula: N = 2S² - 5S + 1
        // Cubic
        ("S³ + 2S - 1 = {}", Box::new(|x| x.powi(3) + 2.0 * x - 1.0)), // Simplified formula: N = S³ + 2S - 1
        ("2S³ - S² + 5 = {}", Box::new(|x| 2.0 * x.powi(3) - x.powi(2) + 5.0)), // Simplified formula: N = 2S³ - S² + 5
        // Mixed operations
        ("S(S + 1) = {}", Box::new(|x| x * (x + 1.0))), // Simplified formula: N = S(S + 1)
        ("(S + 2)(S - 3) = {}", Box::new(|x| (x + 2.0) * (x - 3.0))), // Simplified formula: N = (S + 2)(S - 3)
        ("2(S + 3) = {}", Box::new(|x| 2.0 * (x + 3.0))), // Simplified formula: N = 2(S + 3)
        ("3(S² - 2S) = {}", Box::new(|x| 3.0 * (x.powi(2) - 2.0 * x))), // Simplified formula: N = 3(S² - 2S)
        // Simple fractions
        ("S/2 + 3 = {}", Box::new(|x| x / 2.0 + 3.0)), // Simplified formula: N = S/2 + 3
        ("(S + 1)/3 = {}", Box::new(|x| (x + 1.0) / 3.0)) // Simplified formula: N = (S + 1) / 3
    ];
    
    // Randomly select and display one hint
    let mut rng = rand::rng();
    let (hint, expr) = expressions.choose(&mut rng).unwrap();
    println!("{}: {} = {:.2}", "Hard Hint".purple(), hint, expr(secret_number));
}

/// Displays hint based on player's choice
/// Parameters:
///   op: &str - player's hint selection ("1", "2", or other)
///   secret_number: f64 - number to generate hints for
pub fn choose_hint(op: &str, secret_number: f64) {
    match op.trim() {
        "1" => {
            println!("{}", "Easy hint selected!".blue());
            easy_hint_chooser(secret_number);
        },
        "2" => {
            println!("{}", "Hard hint selected! Calculator recommended.".purple());
            hard_hint_chooser(secret_number);
        },
        _ => println!("{}", "No hints - good luck!".yellow()),
    }
}

/// Manages the core guessing loop
/// Parameters:
///   op: &str - hint choice
///   secret: f64 - target number
///   attempts: i32 - current attempt count
/// Returns:
///   Tuple of (success: bool, total_attempts: i32)
pub fn game_loop(secret: f64, mut attempts: i32) -> (bool, i32) {
    loop {
        attempts += 1;
        println!("\nAttempt #{}", attempts);
        
        // Get and validate player's guess
        print!("Enter your guess (1-100): ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        
        let guess: f64 = match guess.trim().parse() {
            Ok(num) if (1.0..=100.0).contains(&num) => num,
            _ => {
                println!("{}", "Invalid input. Please enter 1-100".red());
                continue;
            }
        };
        
        // Compare guess to secret number
        match guess.partial_cmp(&secret).unwrap() {
            Ordering::Less => {
                println!("{}", "Too small!".red());
                return (false, attempts);
            },
            Ordering::Greater => {
                println!("{}", "Too big!".red());
                return (false, attempts);
            },
            Ordering::Equal => {
                println!("{}", "Correct! You guessed it!".green().bold());
                return (true, attempts);
            }
        }
    }
}
