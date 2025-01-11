use std::cmp::min;

use rand::seq::SliceRandom as _;
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
    /// Creates a solver which deterministically solves the codebreaker game.
    pub fn new() -> Self {
        let mut possible_solutions =
            Vec::with_capacity((NUM_DIFFERENT_PEGS as usize).pow(NUMBER_OF_PEGS_IN_CODE as u32));
        possible_solutions.extend(all_possible_codes());
        Solver {
            unguessed_codes: possible_solutions.clone(),
            possible_solutions,
        }
    }

    /// Creates a solver which picks randomly one of the best guesses
    pub fn with_sampled_guesses(rng: &mut impl rand::Rng) -> Self {
        let mut solver = Solver::new();
        solver.unguessed_codes.shuffle(rng);
        solver
    }

    pub fn guess(&mut self) -> Code {
        // If we know the answer we "guess" it
        if self.possible_solutions.len() == 1 {
            return self.possible_solutions[0];
        }
        // Maximize guaranteed elimination of possiblities
        let best_candidate = self
            .unguessed_codes
            .par_iter()
            .map(|&code| Candidate::new(code))
            .reduce_with(|a, b| {
                Candidate::better(a, b, |code, lower_bound| {
                    self.min_possibilties_eliminated(code, lower_bound)
                })
            })
            .expect("Solution must be possible");
        best_candidate.code
    }

    pub fn update(&mut self, guess: Code, hint: Hint) {
        self.possible_solutions.retain(|&code| {
            let canidate_hint = Hint::new(guess, code);
            hint == canidate_hint
        });
        self.unguessed_codes.retain(|&code| code != guess);
    }

    /// Minimum number of possiblities a guess would eliminate. If the minimum is larger than the
    /// lower_bound result is exact, otherwise it is lower_bound. Setting a lower bound allows the
    /// function to exit early if the result would be smaller than the a better maximum we already
    /// know about.
    fn min_possibilties_eliminated(&self, candidate_guess: Code, lower_bound: u32) -> u32 {
        let mut current_min = u32::MAX;
        for &possible_solution in &self.possible_solutions {
            let hint = Hint::new(candidate_guess, possible_solution);
            let eliminated = self.num_eliminated_possiblities(hint, candidate_guess, current_min);
            current_min = min(current_min, eliminated);
            if current_min <= lower_bound {
                return lower_bound;
            }
        }
        current_min
    }

    /// How many possible codes would be remaining by a guess with a certain hint. The result will
    /// be exact if it is smaller or equal to upper bound. Otherwise it is larger than upper bound.
    fn num_eliminated_possiblities(&self, hint: Hint, guess: Code, upper_bound: u32) -> u32 {
        self.possible_solutions
            .iter()
            .filter(|&&code| hint != Hint::new(guess, code))
            .take(upper_bound as usize)
            .count() as u32
    }
}

/// Candidate for a guess
struct Candidate {
    /// Code the guess represents
    code: Code,
    /// Laziyly computed number of guaranteed possibilities eliminated by this guess
    eliminated: Option<u32>,
}

impl Candidate {
    fn new(code: Code) -> Self {
        Candidate {
            code,
            eliminated: None,
        }
    }

    /// Out of two candidates, picks the better one, or the first one if they are equal.
    fn better(
        mut a: Candidate,
        mut b: Candidate,
        min_eliminated_with_lower_bound: impl Fn(Code, u32) -> u32,
    ) -> Candidate {
        // Calculate the guaranteed number of eliminated possibilities, if missing, but using an
        // existing number as lower bound, if available.
        match (a.eliminated, b.eliminated) {
            // Both values availabe, nothing to do
            (Some(_), Some(_)) => a.replace_if_better(b),
            (Some(a_eliminated), None) => {
                b.eliminated = Some(min_eliminated_with_lower_bound(b.code, a_eliminated));
                // Be careful, the order matters here. Due to short circuiting, above b might be
                // worse, but seem just as good.
                a.replace_if_better(b)
            }
            (None, Some(b_eliminated)) => {
                a.eliminated = Some(min_eliminated_with_lower_bound(a.code, b_eliminated));
                b.replace_if_better(a)
            }
            (None, None) => {
                a.eliminated = Some(min_eliminated_with_lower_bound(a.code, 0));
                b.eliminated = Some(min_eliminated_with_lower_bound(b.code, a.eliminated.unwrap()));
                a.replace_if_better(b)
            }
        }
    }

    /// Replaces the candidate with another one if the other one is better. If both are equal, self
    /// is returned. Requires eliminated to be computed.
    fn replace_if_better(self, other: Candidate) -> Candidate {
        if other.eliminated.unwrap() > self.eliminated.unwrap() {
            other
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{code::Code, hint::Hint};

    use super::Solver;

    #[test]
    fn solve_5611() {
        let code: Code = "5611".parse().unwrap();

        let mut solver = Solver::new();
        let guesses = play_out(code, &mut solver);

        assert_eq!(&["2211", "4321", "5131", "6111", "5611"], &guesses[..]);
    }

    #[test]
    fn solve_2231() {
        let code: Code = "2231".parse().unwrap();

        let mut solver = Solver::new();
        let guesses = play_out(code, &mut solver);

        assert_eq!(&["2211", "3221", "2231"], &guesses[..]);
    }

    fn play_out(code: Code, solver: &mut Solver) -> Vec<String> {
        let mut guesses = Vec::new();
        let mut count = 0;
        loop {
            assert!(count < 5, "Too many guesses");

            let guess = solver.guess();
            guesses.push(guess.to_string());
            let hint = Hint::new(guess, code);
            solver.update(guess, hint);
            if hint.is_solution() {
                break;
            }
            count += 1;
        }
        guesses
    }
}
