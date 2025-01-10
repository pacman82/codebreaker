use std::io;

use anyhow::Error;
use code::Code;
use hint::Hint;
use rand::random;
use solver::Solver;

mod code;
mod peg;
mod hint;
mod solver;

fn main() -> Result<(), Error> {
    let mut num_guess = 0;
    let mut solver = Solver::new();
    let code: Code = random();

    println!("Hello, this is a game, there you guess a code and I give hints after each guess. A \
        code has 4 digits, between 1 and 6.");

    loop {
        println!("({num_guess}) Please enter a code, 's' to let the machine guess for you, or 'q' to quit: ");
        let guess = match ask_for_input()? {
            Input::Guess(guess) => guess,
            Input::Solve => {
                let guess = solver.guess();
                println!("Guess: {guess}");
                guess
            }
            Input::Quit => {
                println!("Quit. Code was {code}. Have a great day!");
                return Ok(());
            },
        };
        let hint = Hint::new(guess, code);
        solver.update(guess, hint);
        num_guess += 1;
        if guess == code {
            println!("Congratulations! You cracked the code in {num_guess} guesses.");
            break;
        }
        println!("Guess: {} correct, {} displaced\n", hint.correct, hint.displaced);
    }
    Ok(())
}

enum Input{ 
    Guess(Code),
    Solve,
    Quit,
}

fn ask_for_input() -> io::Result<Input> {
    let mut input_raw = String::new();
    let code = loop {
        io::stdin().read_line(&mut input_raw)?;
        let input = input_raw.trim();
        match input {
            "s" => return Ok(Input::Solve),
            "q" => return Ok(Input::Quit),
            _ => (),
        }
        match input.parse() {
            Ok(guess) => break guess,
            Err(e) => println!("Invalid input: {e}\nPlease try again: ")
        }
        input_raw.clear();
    };
    Ok(Input::Guess(code))
}