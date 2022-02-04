use colored::*;
use rand::seq::SliceRandom;

use std::fs;

fn main() {
    const WORD_LENGTH: usize = 5;

    println!("Hello, world!");
    let words_raw: String = fs::read_to_string("src/words.txt").expect("Unable to read file");
    let words: Vec<&str> = words_raw.split('\n').collect();
    println!("{} words found in list", words.len());

    //Copy list for guesses
    let mut filtered_words = words.clone();

    let secret_word = <&str>::clone(words.choose(&mut rand::thread_rng()).unwrap());
    println!("Picking secret word: {}", secret_word);

    //Guessing
    loop {
        let guess_word = <&str>::clone(filtered_words
            .choose(&mut rand::thread_rng())
            .unwrap()
            );

        for n in 0..WORD_LENGTH {
            if secret_word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap() {
                print!("{}", guess_word.chars().nth(n).unwrap().to_string().green());
                //Filter out words that don't have matching characters in position n
                filtered_words.retain(|word| {
                    word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap()
                });
            } else if secret_word.contains(guess_word.chars().nth(n).unwrap()) {
                print!(
                    "{}",
                    guess_word.chars().nth(n).unwrap().to_string().yellow()
                );
                //Filter out words that don't have matching characters in position n
                filtered_words.retain(|word| word.contains(guess_word.chars().nth(n).unwrap()));
            } else {
                print!("{}", guess_word.chars().nth(n).unwrap());
            }
        }
        println!();
        if secret_word == guess_word{
            break;
        } else {
            //Remove bad guess from list
                filtered_words.retain(|word| word != &guess_word);
        }
    }
}
