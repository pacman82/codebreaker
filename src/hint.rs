use std::cmp::min;

use crate::{
    code::{Code, NUMBER_OF_PEGS_IN_CODE},
    peg::{Peg, POSSIBLE_COLORS},
};

/// Hints are generated in response to guesses, to give the codebreaker a glue how many pegs fit
/// both, in color and position, and how many pegs only match in color.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hint {
    /// Number of pegs in the guess, which both match color and position of the code.
    pub correct: u8,
    /// Number of pegs in the guess, which only match the color, but not the position. Each peg is
    /// only counted once. E.g. if a guess is red-red-red-blue and the code is red-yellow-green-red
    /// the number of displaced pegs is one.
    pub displaced: u8,
}

impl Hint {
    pub fn new(guess: Code, code: Code) -> Self {
        let (correct, displaced) =
            POSSIBLE_COLORS
                .iter()
                .fold((0, 0), |(total_correct, total_displaced), &color| {
                    let (color_correct, color_displaced) = color_hint(color, guess, code);
                    (
                        total_correct + color_correct,
                        total_displaced + color_displaced,
                    )
                });

        Hint { correct, displaced }
    }

    pub fn is_solution(&self) -> bool {
        self.correct == NUMBER_OF_PEGS_IN_CODE as u8
    }
}

/// Returns for a single color how many pegs are `(correct, displaced)`
fn color_hint(color: Peg, guess: Code, code: Code) -> (u8, u8) {
    let mut color_guess = 0;
    let mut color_code = 0;
    let mut correct = 0;
    for (&guess, code) in guess.0.iter().zip(code.0) {
        if guess == color {
            color_guess += 1;
        }
        if code == color {
            color_code += 1;
        }
        if guess == color && code == color {
            correct += 1;
        }
    }
    let displaced = min(color_code, color_guess) - correct;
    (correct, displaced)
}

#[cfg(test)]
mod tests {
    use super::Hint;


    #[test]
    fn hint() {
        let hint = Hint::new("6335".parse().unwrap(), "3311".parse().unwrap());
        assert_eq!(hint, Hint { correct: 1, displaced: 1})
    }
}