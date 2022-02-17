use std::io;
use rand::Rng;

fn init_clue(clue: &mut String, word: &String) {
    for _ in 0..word.len() {
        clue.push_str("_");
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn game_over(secret_word: String) {
    println!("Game over! You ran out of guesses. The word was: {}", secret_word);
}

fn display_game_state(clue: &String, guesses: i32) {
    println!("Let's play some Hangman!\n{} {} guesses left!", clue, guesses);
}

fn win(user_guess: &String) {
    println!("Nice! That's the word. You win! (The word was '{}')", user_guess);
}

fn random_word(word_list: &[&str]) -> String {
    return String::from(word_list[rand::thread_rng().gen_range(0..word_list.len())]);
}

fn main() {
    clear_terminal();
    let word_list: [&str; 7] = ["house", "hornets", "people", "troglodyte", "personalities", "celebrity", "caffeine"];
    let secret_word: String = random_word(&word_list);
    let mut guesses = 6;

    println!("Let's play some Hangman!");

    let mut total_clue = String::new();
    init_clue(&mut total_clue, &secret_word);

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
            for (i, c) in secret_word.chars().enumerate() {
                if c.to_string() == user_guess {
                    total_clue.replace_range(i..i+1, &user_guess);
                    letter_is_in_word = true;
                }
            }
            if !letter_is_in_word {
                guesses -= 1;
            }

            // display current game state
            display_game_state(&total_clue, guesses);
        }
        else {
            // if user guesses correctly, tell them they did
            if secret_word.eq(&user_guess) {
                display_game_state(&total_clue, guesses);
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