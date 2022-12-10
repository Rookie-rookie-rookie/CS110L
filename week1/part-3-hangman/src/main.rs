// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::collections::HashSet;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_word_so_far(word_so_far :&Vec<char>){
    print!("The word so far is ");
    for c in word_so_far {
        print!("{}",c);
    }
    println!("");
}

fn print_guesses_word(guessed_word :&HashSet<char>){
    print!("You have guessed the following letters: ");
    for c in guessed_word {
        print!("{}",c);
    }
    println!("");
}

fn print_changes_and_enter(changes :u32){
    println!("You have {} guesses left",changes);
    print!("Please guess a letter: ");
}

fn main() {
    println!("Welcome to CS110L Hangman!");
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut word_so_far = Vec::new();
    let mut guessed_word = HashSet::new();
    let mut not_guess_word = HashSet::new();
    let mut changes = NUM_INCORRECT_GUESSES;
    for i in 0..secret_word_chars.len() {
        word_so_far.push('-');
        not_guess_word.insert(secret_word_chars[i]);
    }
    loop{
        print_word_so_far(&word_so_far);
        print_guesses_word(&guessed_word);
        print_changes_and_enter(changes);
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        let guess_char : Vec<char> = guess.chars().collect();
        if not_guess_word.contains(&guess_char[0]){
            not_guess_word.remove(&guess_char[0]);
            guessed_word.insert(guess_char[0]);
            for i in 0..secret_word_chars.len(){
                if secret_word_chars[i] == guess_char[0] {
                    word_so_far[i] = guess_char[0];
                }
            }
        } else {
            changes -= 1;
            println!("Sorry, that letter is not in the word");
        }
        if changes == 0{
            println!("Sorry, you ran out of guesses!");
            break;
        }
        if not_guess_word.is_empty(){
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
    }
}
