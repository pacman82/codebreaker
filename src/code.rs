use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};
use thiserror::Error;

use crate::peg::{Peg, NUM_DIFFERENT_PEGS};

/// Number of Pegs in code
pub const NUMBER_OF_PEGS_IN_CODE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code(pub [Peg; NUMBER_OF_PEGS_IN_CODE]);

impl Distribution<Code> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Code {
        let inner = rng.gen();
        Code(inner)
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..NUMBER_OF_PEGS_IN_CODE {
            write!(f, "{}", self.0[i])?;
        }
        Ok(())
    }
}

impl FromStr for Code {
    type Err = CodeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pegs = [Peg::new(0); NUMBER_OF_PEGS_IN_CODE];
        let mut it_c = s.chars();
        for peg in &mut pegs {
            let Some(c) = it_c.next() else {
                return Err(CodeParsingError::TooFewPegs);
            };
            *peg = Peg::from_char(c).ok_or(CodeParsingError::InvalidPeg(c))?;
        }
        if it_c.next().is_some() {
            return Err(CodeParsingError::TooManyPegs);
        }
        Ok(Code(pegs))
    }
}

/// A gererator for all possible codes
pub fn all_possible_codes() -> impl Iterator<Item = Code> {
    let num_possible_codes = (NUM_DIFFERENT_PEGS as u32).pow(NUMBER_OF_PEGS_IN_CODE as u32);
    (0..num_possible_codes).map(|n| {
        let mut code = [Peg::new(0); NUMBER_OF_PEGS_IN_CODE];
        let mut n = n;
        for peg in &mut code {
            *peg = Peg::new((n % NUM_DIFFERENT_PEGS as u32) as u8);
            n /= NUM_DIFFERENT_PEGS as u32;
        }
        Code(code)
    })
}

#[derive(Error, Debug)]
pub enum CodeParsingError {
    #[error("Too many pegs in code")]
    TooManyPegs,
    #[error("Too few pegs in code")]
    TooFewPegs,
    #[error("Invalid peg '{0}'")]
    InvalidPeg(char),
}
