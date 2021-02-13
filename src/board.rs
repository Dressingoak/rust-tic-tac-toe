use crate::mark::Mark;
use crate::outcome::Outcome;
use crate::outcome::RelativeOutcome;

use std::collections::HashSet;
use strum::IntoEnumIterator;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Board {
    tiles: [Option<Mark>; 9],
    outcome: Option<Outcome>,
    turn: Mark,
}

impl Board {

    pub fn empty(start: Mark) -> Board {
        Board {
            tiles: [None; 9],
            outcome: None,
            turn: start,
        }
    }

    pub fn count(&self) -> usize {
        self.tiles.iter().filter_map(|&x| x).count()
    }

    pub fn get_outcome(&self) -> &Option<Outcome> {
        &self.outcome
    }

    pub fn get_turn(&self) -> &Mark {
        &self.turn
    }

    fn get_index(i: usize, j: usize) -> usize {
        i * 3 + j
    }

    // fn turn(&self) -> Mark {
    //     match self.count() % 2 {
    //         0 => Mark::Cross,
    //         1 => Mark::Nought,
    //         _ => unreachable!()
    //     }
    // }

    pub fn set(&mut self, pos: (usize, usize), mark: Mark) {
        self[pos] = Some(mark);

        for s in Mark::iter() {
            if (0..3).map(|i| (0..3).all(|j| self[(i, j)] == Some(s))).any(|p| p)
            || (0..3).map(|j| (0..3).all(|i| self[(i, j)] == Some(s))).any(|p| p)
            || (0..3).all(|i| self[(i, i)] == Some(s))
            || (0..3).all(|i| self[(i, 2-i)] == Some(s))
            {
                self.outcome = Some(Outcome::Victory(s));
                break;
            }
        }
        if self.count() == 9 {
            self.outcome = Some(Outcome::Draw)
        }
        self.turn = match mark {
            Mark::Cross => Mark::Nought,
            Mark::Nought => Mark::Cross,
        }
        
    }

    pub fn get_options(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Some(_) => None,
                None => Some((i/3, i%3))
            })
    }

    fn transpose(&mut self) {
        for i in 0..3 {
            for j in i+1..3 {
                self.tiles.swap(Board::get_index(i, j), Board::get_index(j, i));
            }
        }   
    }

    fn reverse_columns(&mut self) {
        for i in 0..3 {
            let mut j = 0;
            let mut k = 2;
            while j < k {
                self.tiles.swap(Board::get_index(j, i), Board::get_index(k, i));
                j += 1;
                k -= 1;
            }
        }
    }

    // fn rotate(&mut self) {
    //     self.transpose();
    //     self.reverse_columns();
    // }

    fn get_permutations(&self) -> impl Iterator<Item=Board> + '_ {
        (0..8)
            .scan(self.clone(), |state, i| {
                match i%2 {
                    0 => state.transpose(),
                    1 => state.reverse_columns(),
                    _ => unreachable!(),
                };
                Some(*state)
            })
    }

    pub fn options_without_symmetries(&self) -> impl Iterator<Item=((usize, usize), Board)> + '_ {
        let mark = self.turn;
        let options = self.get_options();
        options
            .map(move |pos| {
                let mut b = self.clone();
                b.set(pos, mark);
                (pos, b)
            })
            .scan(HashSet::<Board>::new(), |state, (pos, board)| {
                let perm: HashSet<Board> = board.get_permutations().collect();
                if perm.is_disjoint(&state) {
                    state.insert(board);
                    Some(Some((pos, board)))
                } else {
                    Some(None)
                }
            })
            .filter_map(|x| x)
    }

    fn deduce_strategies(&self) -> impl Iterator<Item=((usize, usize), RelativeOutcome)> + '_ {
        let mark = self.turn;
        self.options_without_symmetries()
            .map(move |(pos, b): ((usize, usize), Board)| {
                match b.outcome {
                    Some(Outcome::Victory(m)) => {
                        match m == mark {
                            true => (pos, RelativeOutcome::Win),
                            false => (pos, RelativeOutcome::Loss),
                        }
                    }
                    Some(Outcome::Draw) => (pos, RelativeOutcome::Draw),
                    None => {
                        let it = b.deduce_strategies();
                        let best_outcome = match b.turn == mark {
                            true => it.map(|(_, oc)| oc).max(),
                            false => it.map(|(_, oc)| oc.flip()).min(),
                        }.unwrap();
                        (pos, best_outcome)
                    }
                }
            })
    }

    pub fn deduce_optimal_strategies(&self) -> (RelativeOutcome, Vec::<(usize, usize)>) {
        let mut cro = RelativeOutcome::Loss;
        let mut cs = HashSet::<(usize, usize)>::new();
        for (pos, ro) in self.deduce_strategies() {
            if ro > cro {
                cs.clear();
                cs.insert(pos);
                cro = ro;
            } else if ro == cro {
                cs.insert(pos);
            }
        }
        let options: Vec<(usize, usize)> = cs.iter()
            .flat_map(|&pos| {
                let mut l = self.clone();
                l.set(pos, self.turn);
                let permutations: HashSet<Board> = l.get_permutations().collect();
                self.get_options()
                    .filter_map(move |pos| {
                        let mut r = self.clone();
                        r.set(pos, self.turn);
                        match permutations.contains(&r) {
                            true => Some(pos),
                            false => None
                        }
                    })
            })
            .collect();
        
        (cro, options)
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "+-+-+-+");
        for i in 0..3 {
            for j in 0..3 {
                match &self[(i, j)] {
                    Some(v) => write!(f, "|{}", v),
                    None => write!(f, "| "),
                };
            }
            writeln!(f, "|");
            if i < 3 {
                writeln!(f, "+-+-+-+");
            } else {
                write!(f, "+-+-+-+");
            }
        }
        write!(f, "")
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Option<Mark>;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.tiles[pos.0 * 3 + pos.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[pos.0 * 3 + pos.1]
    }
}
