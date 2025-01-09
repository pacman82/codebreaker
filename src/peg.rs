use std::fmt::{self, Display, Formatter};

use rand::{distributions::Standard, prelude::Distribution, Rng};

const NUM_DIFFERENT_PEGS: u8 = 6;

/// A peg represents one of the places in the code which has to be guessed. Usuall represented as
/// colors or numbers. The default version of codebreaker uses 6 different kind of pegs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peg(u8);

impl Peg {
    /// Construct a peg from a number between 0 and 5.
    pub fn new(n: u8) -> Self {
        assert!(n < NUM_DIFFERENT_PEGS);
        Peg(n)
    }
}

impl Display for Peg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Distribution<Peg> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Peg {
        // Avoid bias, by ensuring all numbers are equally likely.
        let byte = loop {
            let candidate: u8 = rng.gen();
            if candidate <= u8::MAX - ((u8::MAX - NUM_DIFFERENT_PEGS + 1) % NUM_DIFFERENT_PEGS) {
                break candidate;
            }
        };
        Peg(byte % NUM_DIFFERENT_PEGS)
    }
}