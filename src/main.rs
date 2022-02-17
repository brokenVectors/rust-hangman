use std::io;
use rand::Rng;

static word_list: [&str; 7] = ["house", "hornets", "people", "troglodyte", "personalities", "celebrity", "caffeine"];
fn init_clue(word: &String) -> String {
    let mut clue = String::new();
    for _ in 0..word.len() {
        clue.push_str("_");
    }
    return clue;
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn game_over(secret_word: String) {
    println!("Game over! You ran out of guesses. The word was: {}", secret_word);
}

fn display_game_state(clue: &String, guesses: i32, failed_letters: &String) {
    println!("Let's play some Hangman!\n{} {} guesses left!\nFailed letters: {}", clue, guesses, failed_letters);
}

fn win(user_guess: &String) {
    println!("Nice! That's the word. You win! (The word was '{}')", user_guess);
}

fn main() {
    clear_terminal();
    
    let secret_word: String = String::from(word_list[rand::random::<usize>() % word_list.len()]);
    let mut guesses = 6;
    let mut failed_letters = String::new();

    println!("Let's play some Hangman!");

    let mut total_clue = init_clue(&secret_word);

    loop {
        clear_terminal();
        if guesses == 0 {
            game_over(secret_word);
            break;
        }

        let mut user_guess = String::new();
        match io::stdin().read_line(&mut user_guess) {
            Ok(_) => {
                user_guess = user_guess.trim().to_string();
            }

            Err(e) => {
                println!("{:?}", e);
            }
        }

        if user_guess.len() == 1 {
            // display all places where user's proposed letter is
            let mut letter_is_in_word = false;
            let mut word_is_complete = true;

            for (i, c) in secret_word.chars().enumerate() {
                let char_not_found = total_clue.chars().nth(i).unwrap().to_string().eq("_");
                if c.to_string() == user_guess {
                    total_clue.replace_range(i..i+1, &user_guess);
                    letter_is_in_word = true;
                } else if char_not_found {
                    word_is_complete = false;
                }
            }
            if !letter_is_in_word {
                failed_letters.push_str(&format!("{}, ", user_guess));
                guesses -= 1;
            }
            if word_is_complete {
                display_game_state(&total_clue, guesses, &failed_letters);
                win(&secret_word);
                break;
            }

            // display current game state
            display_game_state(&total_clue, guesses, &failed_letters);
        }
        else {
            // if user guesses correctly, tell them they did
            if secret_word.eq(&user_guess) {
                display_game_state(&total_clue, guesses, &failed_letters);
                win(&user_guess);
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