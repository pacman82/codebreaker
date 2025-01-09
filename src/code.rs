use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::peg::Peg;

/// Number of Pegs in code
const NUMBER_OF_PEGS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code(pub [Peg; NUMBER_OF_PEGS]);

impl Distribution<Code> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Code {
        let inner = rng.gen();
        Code(inner)
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.0[0])?;
        for i in 1..NUMBER_OF_PEGS {
            write!(f, ", {}", self.0[i])?;
        }
        write!(f, "]")?;
        Ok(())
    }
}