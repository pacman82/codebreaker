use crate::{
    code::{all_possible_codes, Code, NUMBER_OF_PEGS_IN_CODE},
    hint::Hint,
    peg::NUM_DIFFERENT_PEGS,
};

/// A solver for codebreaker
pub struct Solver {
    possible_codes: Vec<Code>,
}

impl Solver {
    pub fn new() -> Self {
        let mut possible_codes =
            Vec::with_capacity((NUM_DIFFERENT_PEGS as usize).pow(NUMBER_OF_PEGS_IN_CODE as u32));
        possible_codes.extend(all_possible_codes());
        Solver { possible_codes }
    }

    pub fn guess(&mut self) -> Code {
        // If we know the answer we "guess" it
        if self.possible_codes.len() == 1 {
            return self.possible_codes[0];
        }
        // Minimize guaranteed remaining possibliities
        let (guess, _max_remaining) = all_possible_codes()
            .map(|candidate_guess| {
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
        self.possible_codes.retain(|&code| {
            let canidate_hint = Hint::new(guess, code);
            hint == canidate_hint
        });
    }

    /// Minimum number of possiblities a guess would eliminate
    fn min_possibilties_eliminated(&self, candidate_guess: Code) -> u32 {
        self.possible_codes
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
        self.possible_codes
            .iter()
            .filter(|&&code| hint != Hint::new(guess, code))
            .count() as u32
    }
}
