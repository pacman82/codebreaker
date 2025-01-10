use std::io;

use anyhow::Error;
use code::Code;
use rand::random;

mod code;
mod peg;

fn main() -> Result<(), Error> {
    let code: Code = random();
    println!("Generated code: {code}");

    println!("Please enter a code, or 'q' to quit: ");
    let Some(guess) = ask_for_guess()? else {
        println!("Quit. Have a great day!");
        return Ok(());
    };

    println!("You guessed: {guess}");

    if guess == code {
        println!("Correct");
    } else {
        println!("Does not match")
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