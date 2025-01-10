use anyhow::Error;
use code::Code;
use rand::random;

mod code;
mod peg;

fn main() -> Result<(), Error> {
    let code: Code = random();
    println!("Generated code: {code}");

    println!("Please enter a code, or 'q' to quit: ");
    let mut input = String::new();
    let guess: Code = loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "q" {
            println!("Quit. Have a great day!");
            return Ok(());
        }
        match input.parse() {
            Ok(guess) => break guess,
            Err(e) => println!("Invalid input: {e}\nPlease try again: ")
        }
    };

    println!("You guessed: {guess}");

    if guess == code {
        println!("Correct");
    } else {
        println!("Does not match")
    }
    Ok(())
}
