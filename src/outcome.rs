use crate::mark::Mark;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Outcome {
    Victory(Mark),
    Draw,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Outcome::Victory(mark) => write!(f, "{}", mark),
            Outcome::Draw => write!(f, "Draw"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum RelativeOutcome {
    Win,
    Draw,
    Loss,
}

impl RelativeOutcome {

    pub fn flip(&self) -> RelativeOutcome {
        match self {
            RelativeOutcome::Win => RelativeOutcome::Loss,
            RelativeOutcome::Draw => RelativeOutcome::Draw,
            RelativeOutcome::Loss => RelativeOutcome::Win,
        }
    }

}

impl PartialOrd for RelativeOutcome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        Some(match (self, other) {
            (RelativeOutcome::Win, RelativeOutcome::Win) => Ordering::Equal,
            (RelativeOutcome::Win, RelativeOutcome::Draw) => Ordering::Greater,
            (RelativeOutcome::Win, RelativeOutcome::Loss) => Ordering::Greater,

            (RelativeOutcome::Draw, RelativeOutcome::Win) => Ordering::Less,
            (RelativeOutcome::Draw, RelativeOutcome::Draw) => Ordering::Equal,
            (RelativeOutcome::Draw, RelativeOutcome::Loss) => Ordering::Greater,

            (RelativeOutcome::Loss, RelativeOutcome::Win) => Ordering::Less,
            (RelativeOutcome::Loss, RelativeOutcome::Draw) => Ordering::Less,
            (RelativeOutcome::Loss, RelativeOutcome::Loss) => Ordering::Equal,
        })
    }
}

impl Ord for RelativeOutcome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self, other) {
            _ if self < other => Ordering::Less,
            _ if self == other => Ordering::Equal,
            _ if self > other => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for RelativeOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RelativeOutcome::Win => write!(f, "win"),
            RelativeOutcome::Draw => write!(f, "draw"),
            RelativeOutcome::Loss => write!(f, "loss"),
        }
    }
}
