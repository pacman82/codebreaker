use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};

use crate::{
    code::{all_possible_codes, Code, NUMBER_OF_PEGS_IN_CODE},
    hint::Hint,
    peg::NUM_DIFFERENT_PEGS,
};

/// A solver for codebreaker
pub struct Solver {
    /// Codes which have not been guessed so far yet.
    unguessed_codes: Vec<Code>,
    /// Solutions which are still possible.
    possible_solutions: Vec<Code>,
}

impl Solver {
    pub fn new() -> Self {
        let mut possible_solutions =
            Vec::with_capacity((NUM_DIFFERENT_PEGS as usize).pow(NUMBER_OF_PEGS_IN_CODE as u32));
        possible_solutions.extend(all_possible_codes());
        Solver { 
            unguessed_codes: possible_solutions.clone(),
            possible_solutions
        }
    }

    pub fn guess(&mut self) -> Code {
        // If we know the answer we "guess" it
        if self.possible_solutions.len() == 1 {
            return self.possible_solutions[0];
        }
        // Minimize guaranteed remaining possibliities
        let (guess, _max_remaining) = self.unguessed_codes.par_iter()
            .map(|&candidate_guess| {
                (
                    candidate_guess,
                    self.min_possibilties_eliminated(candidate_guess),
                )
            })
            .max_by_key(|(_guess, min_possibilities_eliminated)| *min_possibilities_eliminated)
            .expect("All hints must be valid");
        guess
    }

    pub fn update(&mut self, guess: Code, hint: Hint) {
        self.possible_solutions.retain(|&code| {
            let canidate_hint = Hint::new(guess, code);
            hint == canidate_hint
        });
        self.unguessed_codes.retain(|&code| code != guess);
    }

    /// Minimum number of possiblities a guess would eliminate
    fn min_possibilties_eliminated(&self, candidate_guess: Code) -> u32 {
        self.possible_solutions
            .iter()
            .map(|&possible_solution| {
                let hint = Hint::new(candidate_guess, possible_solution);
                self.num_eliminated_possiblities(hint, candidate_guess)
            })
            .min()
            .expect("All hints must be valid")
    }

    /// How many possible codes would be remaining by a guess with a certain hint.
    fn num_eliminated_possiblities(&self, hint: Hint, guess: Code) -> u32 {
        self.possible_solutions
            .iter()
            .filter(|&&code| hint != Hint::new(guess, code))
            .count() as u32
    }
}
