use std::io;

use anyhow::Error;
use code::Code;
use hint::Hint;
use rand::random;

mod code;
mod peg;
mod hint;

fn main() -> Result<(), Error> {
    let mut num_guess = 0;
    let code: Code = random();

    println!("Hello, this is a game, there you guess a code and I give hints after each guess. A \
        code has 4 digits, between 1 and 6.");

    loop {
        println!("({num_guess}) Please enter a code, or 'q' to quit: ");
        let Some(guess) = ask_for_guess()? else {
            println!("Quit. Code was {code}. Have a great day!");
            return Ok(());
        };
        let hint = Hint::new(guess, code);
        num_guess += 1;
        if guess == code {
            println!("Congratulations! You cracked the code in {num_guess} guesses.");
            break;
        }
        println!("Guess: {} correct, {} displaced\n", hint.correct, hint.displaced);
    }
    Ok(())
}

fn ask_for_guess() -> io::Result<Option<Code>> {
    let mut input_raw = String::new();
    let code = loop {
        io::stdin().read_line(&mut input_raw)?;
        let input = input_raw.trim();
        if input == "q" {
            return Ok(None);
        }
        match input.parse() {
            Ok(guess) => break guess,
            Err(e) => println!("Invalid input: {e}\nPlease try again: ")
        }
        input_raw.clear();
    };
    Ok(Some(code))
}