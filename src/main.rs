use colored::*;
//use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::fs;

fn main() {
    const WORD_LENGTH: usize = 5;

    println!("Hello, world!");
    let words_raw: String = fs::read_to_string("src/words.txt").expect("Unable to read file");
    let words: Vec<&str> = words_raw.split('\n').collect();
    println!("{} words found in list", words.len());

    //Copy list for guesses
    let mut char_population = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    for n in 0..char_population.len() {
        for c in 'a'..='z' {
            char_population[n].insert(c, 0);
        }
        for word in &words {
            *char_population[n]
                .get_mut(&word.chars().nth(n).unwrap())
                .unwrap() += 1;
        }
    }

    for n in 0..char_population.len() {
        for c in 'a'..='z' {
            print!("{}:{}  ", c, char_population[n].get(&c).unwrap());
        }
        println!()
    }

    let mut guesses_taken = Vec::new();

    let mut filtered_words = words.clone();
    //Sort filtered words by best guesses
    let word_rating = |word: &str| {
        let mut rating = 1;

        let mut unique_chars = 0;
        for i in 0..WORD_LENGTH {
            let mut unique = true;
            for j in 0..WORD_LENGTH {
                if i != j && word.chars().nth(i).unwrap() == word.chars().nth(j).unwrap() {
                    unique = false;
                }
            }
            if unique {
                unique_chars += 1;
            }
            rating += char_population[i]
                .get(&word.chars().nth(i).unwrap())
                .unwrap();
        }

        rating *= unique_chars;

        rating
    };

    filtered_words.reverse();
    filtered_words.sort_by_key(|a| word_rating(a));

    println!();
    for secret_word in &words {
        print!("\r{}", &secret_word);

        let mut guess_word_pool = filtered_words.clone();

        //let secret_word = <&str>::clone(words.choose(&mut rand::thread_rng()).unwrap());
        //println!("Picking secret word: {}", secret_word);

        //Guessing
        let mut guesses = 0;
        loop {
            let guess_word = guess_word_pool.pop().unwrap();
            guesses += 1;

            //TODO filter out words where yellow letter is in same position
            for n in 0..WORD_LENGTH {
                if secret_word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap() {
                    //print!("{}", guess_word.chars().nth(n).unwrap().to_string().green());
                    //Filter out words that don't have matching characters in position n
                    guess_word_pool.retain(|word| {
                        word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap()
                    });
                } else if secret_word.contains(guess_word.chars().nth(n).unwrap()) {
                    //print!(
                    //    "{}",
                    //    guess_word.chars().nth(n).unwrap().to_string().yellow()
                    //);
                    //Filter out words that don't have matching characters in position n
                    guess_word_pool
                        .retain(|word| word.contains(guess_word.chars().nth(n).unwrap()));
                    //Filter out words that have same letter in same position                guess_word_pool.retain(|word| {
                    guess_word_pool.retain(|word| {
                        word.chars().nth(n).unwrap() != guess_word.chars().nth(n).unwrap()
                    });
                } else {
                    //print!("{}", guess_word.chars().nth(n).unwrap());
                    //Filter out words with letters not in guessed word
                    guess_word_pool
                        .retain(|word| !word.contains(guess_word.chars().nth(n).unwrap()));
                }
            }
            //println!();
            if secret_word == &guess_word {
                guesses_taken.push(guesses);
                break;
            } else {
                //Remove bad guess from list
                guess_word_pool.retain(|word| word != &guess_word);
            }
        }
    }
    print!("\r     \r");

    let mut sum = 0;
    for guess_count in guesses_taken.iter() {
        sum += guess_count;
    }
    println!(
        "Average number of guesses to solve: {}",
        (sum as f64) / (guesses_taken.len() as f64)
    );
}
