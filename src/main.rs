use colored::*;
use rand::seq::SliceRandom;

use std::collections::HashMap;
use std::fs;

fn main() {
    const WORD_LENGTH: usize = 5;

    let words_answers_raw: String =
        fs::read_to_string("src/wordle-answers.txt").expect("Unable to read file");
    let words_guesses_raw: String =
        fs::read_to_string("src/wordle-guesses.txt").expect("Unable to read file");

    let answers: Vec<&str> = words_answers_raw.split('\n').collect();
    let mut words: Vec<&str> = words_guesses_raw.split('\n').collect();
    words.extend(&answers);

    //Build up character counts for guess ordering
    let mut char_population = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    let mut char_population_total = [0, 0, 0, 0, 0];
    for n in 0..char_population.len() {
        for c in 'a'..='z' {
            char_population[n].insert(c, 0);
        }
        for word in &words {
            *char_population[n]
                .get_mut(&word.chars().nth(n).unwrap())
                .unwrap() += 1;
            char_population_total[n] += 1;
        }
    }

    //Print out character counts
    //for n in 0..char_population.len() {
    //    for c in 'a'..='z' {
    //        print!("{}:{}  ", c, char_population[n].get(&c).unwrap());
    //    }
    //    println!()
    //}

    //Copy list for guesses
    let mut filtered_words = words.clone();

    //Sort filtered words by best guesses
    let word_rating = |word: &str| {
        let mut rating = 1.0;

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
            rating += *char_population[i]
                .get(&word.chars().nth(i).unwrap())
                .unwrap() as f64
                / char_population_total[i] as f64;
        }

        rating *= unique_chars as f64;

        rating
    };
    filtered_words.reverse();
    filtered_words.sort_by(|a, b| word_rating(a).partial_cmp(&word_rating(b)).unwrap());

    //Pick secret word
    let secret_word = <&str>::clone(answers.choose(&mut rand::thread_rng()).unwrap());
    println!("Picking secret word: {}", secret_word);

    //Guessing
    let mut guess_word_pool = filtered_words.clone();
    loop {
        let guess_word = guess_word_pool.pop().unwrap();

        for n in 0..WORD_LENGTH {
            if secret_word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap() {
                print!("{}", guess_word.chars().nth(n).unwrap().to_string().green());
                //Filter out words that don't have matching characters in position n
                guess_word_pool.retain(|word| {
                    word.chars().nth(n).unwrap() == guess_word.chars().nth(n).unwrap()
                });
            } else if secret_word.contains(guess_word.chars().nth(n).unwrap()) {
                print!(
                    "{}",
                    guess_word.chars().nth(n).unwrap().to_string().yellow()
                );
                //Filter out words that don't have matching characters in position n
                guess_word_pool.retain(|word| word.contains(guess_word.chars().nth(n).unwrap()));
                //Filter out words that have same letter in same position                guess_word_pool.retain(|word| {
                guess_word_pool.retain(|word| {
                    word.chars().nth(n).unwrap() != guess_word.chars().nth(n).unwrap()
                });
            } else {
                print!("{}", guess_word.chars().nth(n).unwrap());
                //Filter out words with letters not in guessed word
                guess_word_pool.retain(|word| !word.contains(guess_word.chars().nth(n).unwrap()));
            }
        }
        println!();
        if secret_word == guess_word {
            break;
        } else {
            //Remove bad guess from list
            guess_word_pool.retain(|word| word != &guess_word);
        }
    }
}
