use std::fmt::{self, Display, Formatter};

use rand::{distributions::Standard, prelude::Distribution, Rng};
const NUM_DIFFERENT_PEGS: u8 = 6;

/// Array of all different peg variants
pub const POSSIBLE_COLORS: [Peg; NUM_DIFFERENT_PEGS as usize] = possible_pegs();

/// A peg represents one of the places in the code which has to be guessed. Usuall represented as
/// colors or numbers. The default version of codebreaker uses 6 different kind of pegs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peg(u8);

impl Peg {
    /// Construct a peg from a number between 0 and 5.
    pub const fn new(n: u8) -> Self {
        assert!(n < NUM_DIFFERENT_PEGS);
        Peg(n)
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Peg(0)),
            '2' => Some(Peg(1)),
            '3' => Some(Peg(2)),
            '4' => Some(Peg(3)),
            '5' => Some(Peg(4)),
            '6' => Some(Peg(5)),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Peg(0) => '1',
            Peg(1) => '2',
            Peg(2) => '3',
            Peg(3) => '4',
            Peg(4) => '5',
            Peg(5) => '6',
            _ => unreachable!(),
        }
    }
}

impl Display for Peg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
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

const fn possible_pegs() -> [Peg; NUM_DIFFERENT_PEGS as usize] {
    let mut pegs = [Peg(0); NUM_DIFFERENT_PEGS as usize];
    let mut index = 0;
    loop {
        if index == NUM_DIFFERENT_PEGS {
            break;
        }
        pegs[index as usize] = Peg::new(index);
        index += 1;
    }
    pegs
}