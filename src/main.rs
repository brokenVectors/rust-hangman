use std::io;
use rand::Rng;

fn main() {
    let WORD_LIST: [&str; 7] = ["house", "hornets", "people", "troglodyte", "personalities", "celebrity", "caffeine"];
    let wordIndex = rand::thread_rng().gen_range(0..WORD_LIST.len());
    let SECRET_WORD: &str = WORD_LIST[wordIndex];
    let mut guesses = 6;

    println!("Let's play some Hangman!");

    let mut total_clue = String::new();
    for _ in 0..SECRET_WORD.len() {
        total_clue.push_str("_");
    }

    loop {

        if guesses == 0 {
            println!("Game over! You ran out of guesses.");
            break;
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
            }

            Err(e) => {
                println!("{:?}", e);
            }
        }

        // user's guess is input, obviously!
        let user_guess = input;
        if user_guess.len() == 1 {
            guesses -= 1;

            // display all places where user's proposed letter is
            for (i, c) in SECRET_WORD.chars().enumerate() {
                let c_string = c.to_string();
                if c_string == user_guess {
                    total_clue.replace_range(i..i+1, &user_guess);
                }
            }

            // display current game state
            println!("{} {} guesses left!", total_clue, guesses);
            
        }
        else {

            // if user guesses correctly, tell them they did
            if SECRET_WORD.eq(&user_guess) {
                println!("Nice! That's the word. You win!");
                break;
            }
            else {
                // if they didn't, tell them they didn't
                guesses -= 1;
                println!("Whoops! That's not the word. {} guesses left!", guesses);
            }
        }
        
    }
}