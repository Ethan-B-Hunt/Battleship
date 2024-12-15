use std::io;
use reqwest;
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    runtime.block_on(async {
        loop {
            println!("Welcome to Hangman!");
            println!("Choose a game mode:");
            println!("1. Random word");
            println!("2. Word of a specific length");
            println!("3. Quit");

            let mut game_mode = String::new();
            io::stdin().read_line(&mut game_mode).expect("Failed to read line");
            let game_mode: i32 = game_mode.trim().parse().expect("Invalid input");

            match game_mode {
                1 => {
                    let url = "https://random-word-api.herokuapp.com/word";
                    let response = reqwest::get(url).await.unwrap();
                    let word = response.text().await.unwrap().trim().to_string();
                    play_game(word).await;
                }
                2 => {
                    println!("Enter the length of the word:");
                    let mut length_str = String::new();
                    io::stdin().read_line(&mut length_str).expect("Failed to read line");
                    let length: i32 = length_str.trim().parse().expect("Invalid input");
                    let url = format!("https://random-word-api.herokuapp.com/word?length={}", length);
                    let response = reqwest::get(&url).await;
                    match response {
                        Ok(response) => {
                            let word = response.text().await.unwrap().trim().to_string();
                            play_game(word).await;
                        }
                        Err(_) => {
                            println!("Sorry, no words of that length are available. Please try again.");
                        }
                    }
                }
                3 => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid game mode. Please choose 1, 2, or 3.");
                }
            }
        }
    });
}

async fn play_game(word: String) {
    println!("Choose the number of lives:");
    let mut lives = String::new();
    io::stdin().read_line(&mut lives).expect("Failed to read line");
    let mut lives: i32 = lives.trim().parse().expect("Invalid input");

    if lives <= 0 {
        println!("Invalid number of lives. Exiting...");
        return;
    }

    let mut guessed_word: Vec<char> = vec!['_'; word.len()];
    let mut letter_bank: Vec<char> = Vec::new();

    println!("Game started!");
    println!("Word: {}", guessed_word.iter().collect::<String>());

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("**Hangman Game**");
        println!("------------------");
        println!("**Lives Remaining:** {}", lives);
        println!("**Word:** {}", guessed_word.iter().collect::<String>());
        println!("**Letter Bank:** {}", letter_bank.iter().collect::<String>());
        println!("------------------");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match guess.trim().chars().next() {
            Some(c) if c.is_alphabetic() => c.to_lowercase().next().unwrap(),
            _ => {
                println!("Please enter a single alphabetic character.");
                std::thread::sleep(std::time::Duration::from_millis(1000));
                continue;
            }
        };

        if letter_bank.contains(&guess) {
            println!("You already guessed the letter '{}'!", guess);
            std::thread::sleep(std::time::Duration::from_millis(1000));
            continue;
        }

        letter_bank.push(guess);

        let mut correct_guess = false;

        for (i, c) in word.chars().enumerate() {
            if c == guess {
                guessed_word[i] = c;
                correct_guess = true;
            }
        }

        if !correct_guess {
            lives -= 1;
            println!("Incorrect guess! {}", guess);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        if lives == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!("**Game Over!**");
            println!("The word was {}.", word);
            println!("Your Letter Bank: {}", letter_bank.iter().collect::<String>());
            break;
        }

        if !guessed_word.contains(&'_') {
            print!("\x1B[2J\x1B[1;1H");
            println!("**Congratulations!**");
            println!("You guessed the word {}!", word);
            println!("Your Letter Bank: {}", letter_bank.iter().collect::<String>());
            break;
        }
    }
}
