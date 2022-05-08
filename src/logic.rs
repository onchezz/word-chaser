#[warn(dead_code)]
extern crate rand;
extern crate serde;

extern crate serde_json;

/*used generating random index */
use rand::Rng;

/*used parsing json data */
use serde::{Deserialize, Serialize};

/*imports from rust standard librart to read files */
use std::{fs::File, io::prelude::*};

/*Used to read user input */
use std::io;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub word: String,
    pub meaning: String,
    pub example: String,
}

// enum ProgressState {
//     Finished,
//     Failed,
// }

impl Word {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            meaning: String::new(),
            example: String::new(),
        }
    }

    pub fn get_word_json(data: String) -> Self {
        //saving words to a vector of type ::word which is a struct
        let words: Vec<Word> = serde_json::from_str(data.as_str()).expect("error giving json");

        let random_index = rand::thread_rng().gen_range(0..words.len());
        //returning the generated word
        words[random_index].clone()
    }

    /*generating a string with the random word hidden on the example usange of the word */
    pub fn creat_example_string(word: &mut Word) -> String {
        let word_example = &word.example;
        let mut letters = Letters::get_letters(&mut word.word);
        let unknown_word = Letters::display_progress(&mut letters);

        let generated_word = word_example.replace(&word.word, unknown_word.as_str());

        generated_word
    }
}
#[derive(Debug)]
pub struct Letters {
    pub letter: char,
    pub is_revealed: bool,
}

impl Letters {
    //getting letters from the generated word
    fn get_letters(word: &mut String) -> Vec<Letters> {
        let mut chars: Vec<Letters> = Vec::new();
        //destracturing and looping the word letters and  saving them in a vector of type letters
        for c in word.chars() {
            chars.push(Self {
                letter: c,
                is_revealed: false,
            });
        }
        //returning the generated letters
        chars
    }

    /*this functions takes in a vector of letters and creats a display string of letter depending on if it is revealed or not */
    fn display_progress(letters: &mut Vec<Letters>) -> String {
        let mut display_string = String::from("  ");
        for l in letters {
            display_string.push(' ');
            if l.is_revealed {
                display_string.push(l.letter);
            } else {
                display_string.push('_');
            }
        }
        display_string
    }
}

pub mod functionality {

    use super::*;
    /*getting data from json file and returing strings  */
    pub fn get_file_data() -> String {
        let mut file = File::open("words.json").expect("error opening file");
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();
        data
    }

    /*Reading user input and returns a character  */
    pub fn read_user_input() -> char {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input ");
        let input = user_input.chars().next().expect("!");

        input
    }

    pub fn play() {
        let log_meassage = format!("    Chase  the word  ");

        println!("{}", log_meassage);
        let data = get_file_data();
        let mut word = Word::get_word_json(data);

        let mut letters = Letters::get_letters(&mut word.word);
        let example = Word::creat_example_string(&mut word);
        let log_meassage = format!(
            "Meaning :  \n{}        \nExample:  \n{}",
            word.meaning, example
        );

        println!("{}", log_meassage);
        let mut turns_available = word.word.len();
        let mut tries = 0;
        let mut reveal_count = 0;
        let letter_reveared = format!("number of letters revealed {}", reveal_count);
        let mut input: char;
        loop {
            let log_meassage = format!("Turns remaining  {} ", turns_available);

            println!("{}", log_meassage);

            let display = Letters::display_progress(&mut letters);
            println!("Progress {}", display);
            println!("Enter letter ");
            input = read_user_input();

            let mut atleast_found_a_char: bool = false;
            for l in letters.iter_mut() {
                if l.letter == input {
                    l.is_revealed = true;
                    atleast_found_a_char = true;
                    reveal_count += 1;
                }
            }

            if !atleast_found_a_char {
                if turns_available > 0 {
                    turns_available -= 1;
                    tries += 1;
                }
                if turns_available == 0 {
                    let msg = format!("Sorry you lost {}:  turns  left", turns_available);
                    println!("{}", msg);
                    println!("{}", letter_reveared);

                    break;
                }
            }

            /*checking if all letters of the word are reaveled and returning is true*/
            let is_valid = letters
                .iter()
                .zip(letters.iter())
                .all(|(_, c)| c.is_revealed);

            if is_valid {
                println!("Congrats you won the game ");
                let tries_msg = format!("you failed  {}:times  out of {}", tries, word.word.len());
                println!("{}", tries_msg);

                break;
            }
            if input == '!' {
                println!("{}", letter_reveared);
                println!("exiting Game ");
                break;
            }
            if input == '>' {
                next();

                break;
            }
        }

        println!("The word was {}", word.word);
    }
    fn next() {
        println!("Skiping word  >>>> ");
        play();
    }
}
