use crate::{
    code::{Code, NUMBER_OF_PEGS_IN_CODE},
    hint::Hint,
    peg::{NUM_DIFFERENT_PEGS, POSSIBLE_COLORS},
};

/// A solver for codebreaker
pub struct Solver {
    possible_codes: Vec<Code>,
}

impl Solver {
    /// Create a new solver
    pub fn new() -> Self {
        let mut possible_codes =
            Vec::with_capacity((NUM_DIFFERENT_PEGS as usize).pow(NUMBER_OF_PEGS_IN_CODE as u32));
        for a in POSSIBLE_COLORS {
            for b in POSSIBLE_COLORS {
                for c in POSSIBLE_COLORS {
                    for d in POSSIBLE_COLORS {
                        possible_codes.push(Code([a, b, c, d]));
                    }
                }
            }
        }
        Solver { possible_codes }
    }

    /// Make a guess
    pub fn guess(&mut self) -> Code {
        self.possible_codes.remove(0)
    }

    /// Update the solver with the hint
    pub fn update(&mut self, guess: Code, hint: Hint) {
        self.possible_codes.retain(|&code| {
            let candidate_hint = Hint::new(guess, code);
            hint == candidate_hint
        });
    }
}
