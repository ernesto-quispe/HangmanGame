// Hangman Game in Rust
// --------------------

/*
    This program implements a command-line Hangman game.
    The player tries to guess a secret word selected randomly from a list by entering one letter at a time.

    Features:
    - Random word selection from a predefined list
    - Tracks correct and incorrect guesses
    - Displays progress, guessed letters, and hangman drawing
    - Detects win/loss and shows messages
    - Detailed comments for educational purposes
*/

// Bring required modules into scope
use std::io;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Hangman figure representation for each stage (indexed by wrong guesses)
const HANGMAN_STAGES: [&str; 7] = [
    "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
    "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
];

// Number of maximum wrong attempts allowed
const MAX_ATTEMPTS: usize = 6;

fn main() {
    // Predefined word list: can be replaced with a larger or file-based set
    let word_list = vec![
        "rustacean", "hangman", "programming", "computer",
        "science", "algorithm", "variable", "function",
        "debugging", "development"
    ];

    // Selects a random word from the list and converts it to lowercase
    let secret_word = select_word(&word_list);

    // Set to keep track of user guesses
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut wrong_attempts = 0; // Tracks number of incorrect guesses

    println!("Welcome to Hangman in Rust!");
    println!("Try to guess the secret word one letter at a time.");
    println!("You have {} wrong attempts before the hangman is completed.", MAX_ATTEMPTS);

    // Main game loop: runs until win or loss
    loop {
        // Print current hangman stage depending on wrong guesses
        draw_hangman(wrong_attempts);

        // Print the progress (underscores for unguessed, letters for guessed positions)
        let progress = reveal_word(&secret_word, &guessed_letters);
        println!("Word: {}", progress);

        // Show guessed letters so far, sorted alphabetically for readability
        let mut guessed_vec: Vec<_> = guessed_letters.iter().cloned().collect();
        guessed_vec.sort();
        println!("Guessed letters: {:?}", guessed_vec);

        // Check win/loss conditions and end game if needed
        if is_winner(&secret_word, &guessed_letters) {
            println!("Congratulations! You've guessed the word '{}'.", secret_word);
            break;
        }

        if wrong_attempts >= MAX_ATTEMPTS {
            println!("Game Over! The word was '{}'.", secret_word);
            break;
        }

        // Get valid user input
        let guess = ask_for_letter();

        // Ignore already guessed letters
        if guessed_letters.contains(&guess) {
            println!("You have already guessed '{}'. Try another letter.", guess);
            continue;
        }

        // Add guess to set
        guessed_letters.insert(guess);

        // Check if the guess is in the word
        if secret_word.contains(guess) {
            println!("Good guess! '{}' is in the word.", guess);
        } else {
            println!("Sorry, '{}' is not in the word.", guess);
            wrong_attempts += 1;
        }
    }
}

/// Selects a random word from a reference to a vector of words and returns its lowercase string
fn select_word(words: &Vec<&str>) -> String {
    let mut rng = thread_rng();
    words.choose(&mut rng).unwrap().to_lowercase()
}

/// Returns a string representation with guessed letters shown and unguessed as underscores
fn reveal_word(secret_word: &str, guessed_letters: &HashSet<char>) -> String {
    let mut revealed = String::new();
    for c in secret_word.chars() {
        if guessed_letters.contains(&c) {
            revealed.push(c);
        } else {
            revealed.push('_');
        }
        revealed.push(' '); // for easy reading
    }
    revealed.trim_end().to_string()
}

/// Checks if the user guessed all letters correctly
fn is_winner(secret_word: &str, guessed_letters: &HashSet<char>) -> bool {
    secret_word.chars().all(|c| guessed_letters.contains(&c))
}

/// Requests a single letter from the user, validates input, forces lowercase and ensures it's a letter
fn ask_for_letter() -> char {
    loop {
        println!("Please enter a single letter:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_lowercase();
                if trimmed.len() == 1 && trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return trimmed.chars().next().unwrap();
                }
            },
            Err(_) => {},
        }
        println!("Invalid entry. Enter a single alphabetic letter (A-Z).");
    }
}

/// Draws the current Hangman stage based on wrong attempts
fn draw_hangman(wrong_attempts: usize) {
    println!("{}", HANGMAN_STAGES[wrong_attempts]);
}

// ------ End of Hangman Game ------
