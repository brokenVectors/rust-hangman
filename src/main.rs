use std::io;
use rand::Rng;

fn init_clue(clue: &mut String, word: &String) {
    for _ in 0..word.len() {
        clue.push_str("_");
    }
}
fn main() {
    let WORD_LIST: [&str; 7] = ["house", "hornets", "people", "troglodyte", "personalities", "celebrity", "caffeine"];
    let wordIndex = rand::thread_rng().gen_range(0..WORD_LIST.len());
    let secret_word: String = String::from(WORD_LIST[wordIndex]);
    let mut guesses = 6;

    println!("Let's play some Hangman!");
    let mut total_clue = String::new();
    init_clue(&mut total_clue, &secret_word);

    loop {

        if guesses == 0 {
            println!("Game over! You ran out of guesses. The word was: {}", secret_word);
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
            

            // display all places where user's proposed letter is
            let mut letter_is_in_word = false;
            for (i, c) in secret_word.chars().enumerate() {
                let c_string = c.to_string();
                if c_string == user_guess {
                    total_clue.replace_range(i..i+1, &user_guess);
                    letter_is_in_word = true;
                }
            }
            if !letter_is_in_word {
                guesses -= 1;
            }

            // display current game state
            println!("{} {} guesses left!", total_clue, guesses);
            
        }
        else {

            // if user guesses correctly, tell them they did
            if secret_word.eq(&user_guess) {
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