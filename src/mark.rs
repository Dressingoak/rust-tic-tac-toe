use strum_macros::EnumIter;

use rand::{ distributions::{Distribution, Standard}, Rng };

#[derive(Hash, Eq, PartialEq, Copy, Clone, EnumIter)]
pub enum Mark {
    Cross,
    Nought,
}

impl Mark {
    pub fn reverse(&self) -> Mark {
        match self {
            Mark::Cross => Mark::Nought,
            Mark::Nought => Mark::Cross,
        }
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mark::Cross => write!(f, "X"),
            Mark::Nought => write!(f, "O"),
        }
    }
}

impl std::str::FromStr for Mark {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let c = s.trim().chars().nth(0);
        match c {
            Some('X') | Some('x') => Ok(Mark::Cross),
            Some('O') | Some('o') => Ok(Mark::Nought),
            _ => Err("Couldn't infer mark (be sure to write either 'X', 'x', 'O' or 'o')"),
        }
    }
}

impl Distribution<Mark> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mark {
        match rng.gen_range(0..=1) { // rand 0.8
            0 => Mark::Cross,
            _ => Mark::Nought,
        }
    }
}
