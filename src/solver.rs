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
        self.possible_codes.remove(0)
    }

    pub fn update(&mut self, guess: Code, hint: Hint) {
        self.possible_codes.retain(|&code| {
            let canidate_hint = Hint::new(guess, code);
            hint == canidate_hint
        });
    }
}
