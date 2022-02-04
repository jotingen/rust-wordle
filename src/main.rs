use colored::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::fs;

fn main() {
    const WORD_LENGTH: usize = 5;

    println!("Hello, world!");
    let words_raw: String = fs::read_to_string("src/words.txt").expect("Unable to read file");
    let words: Vec<&str> = words_raw.split('\n').collect();
    println!("{} words found in list", words.len());

    //Copy list for guesses
    let mut filtered_words = words.clone();
    let mut char_population = [HashMap::new(),HashMap::new(), HashMap::new(), HashMap::new(),  HashMap::new() ];
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
            rating += char_population[i].get(&word.chars().nth(i).unwrap()).unwrap();
        }

        rating *= unique_chars;

        rating
    };

    filtered_words.reverse();
    filtered_words.sort_by_key(|a| word_rating(a));

    let secret_word = <&str>::clone(words.choose(&mut rand::thread_rng()).unwrap());
    println!("Picking secret word: {}", secret_word);

    //Guessing
    loop {
        let guess_word = filtered_words.pop().unwrap();

        //TODO filter out words where yellow letter is in same position
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
                //Filter out words that have same letter in same position                filtered_words.retain(|word| {
                filtered_words.retain(|word| {
                    word.chars().nth(n).unwrap() != guess_word.chars().nth(n).unwrap()
                });
            } else {
                print!("{}", guess_word.chars().nth(n).unwrap());
                //Filter out words with letters not in guessed word
                filtered_words.retain(|word| !word.contains(guess_word.chars().nth(n).unwrap()));
            }
        }
        println!();
        if secret_word == guess_word {
            break;
        } else {
            //Remove bad guess from list
            filtered_words.retain(|word| word != &guess_word);
        }
    }
}
