use crossterm::style::Stylize;
use std::{fs, io};
use std::io::Write;
use rand::Rng;

const MAX_TRIES: i8 = 6;

fn main() {
    let mut rng = rand::thread_rng();

    let wordlist = fs::read("wordlist.txt").expect("\nCannot read the wordlist file!");
    let wordlist = String::from_utf8(wordlist).expect("\nCannot convert wordlist to string!");
    let wordlist: Vec<&str> = wordlist.split("\n").collect();

    // Game Loop
    while true{
        clear();
        println!("{}", "Welcome to termo Rust version!".green());
        println!("{}", "Made by capelosini".blue());

        let word = String::from(wordlist[rng.gen_range(0..wordlist.len())].trim());
        let mut guess_state: Vec<String> = vec!["_".to_string(); word.len()];
        let mut corrects: i16 = 0;
        println!("\n Word: {}", guess_state.join(""));
        
        for i in 0..MAX_TRIES{
            let mut guess = String::new();
            print!("\n Guess {}/{}: ", i+1, MAX_TRIES);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            let guess = guess.trim();
            if guess.len() > word.len(){
                let guess = &guess[0..word.len()];
                corrects=update_state(word.as_str(), guess, &mut guess_state);
            } else{
                corrects=update_state(word.as_str(), guess, &mut guess_state);
            }
            println!(" Word: {}", guess_state.join(""));

            if corrects == word.len() as i16{
                println!("\n{}", "You won!".green());
                println!("\nClick enter to start other match..");
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).expect("Error getting input!");
                break;
            } else if corrects != word.len() as i16 && i == MAX_TRIES-1{
                println!("\n{}", "You lost!".red());
                println!("\nThe word was: {}", word.clone().green());
                println!("\nClick enter to start other match..");
                let mut temp = String::new();
                io::stdin().read_line(&mut temp).expect("Error getting input!");
            }
        }
    }
}

fn update_state(word: &str, guess: &str, guess_state: &mut Vec<String>) -> i16{
    let mut guess_out = String::new();
    let mut corrects: i16=0;
    for (letter_i, guess_letter) in guess.chars().enumerate(){
        let letter = word.chars().nth(letter_i).unwrap();
        if letter == guess_letter{
            guess_state[letter_i]=guess_letter.to_string();
            guess_out=guess_out+format!("{}", guess_letter.green()).as_str();
            corrects+=1;
        } else if word.contains(guess_letter){
            guess_out=guess_out+format!("{}", guess_letter.yellow()).as_str();
        } else{
            guess_out=guess_out+format!("{}", guess_letter.red()).as_str();
        }
    }
    println!("\nGuess: {}", guess_out);
    corrects
}

fn clear(){
    println!("\x1b[2J");
}