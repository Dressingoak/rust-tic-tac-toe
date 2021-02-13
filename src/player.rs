
use crate::mark::Mark;
use crate::board::Board;

use regex::Regex;
use rand::seq::SliceRandom;

pub enum PlayerType {
    User,
    Machine,
}

pub struct Player {
    mark: Mark,
    player_type: PlayerType,
}

fn get_line_input() -> String {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}

impl Player {
    pub fn create(mark: Mark, player_type: PlayerType) -> Player {
        Player { mark, player_type }
    }

    pub fn get_mark(&self) -> &Mark {
        &self.mark
    }

    pub fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    pub fn get_action(&self, board: &Board) -> (usize, usize) {
        match self.player_type {
            PlayerType::Machine => self.get_optimal_action(board),
            PlayerType::User => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^\s*([1-3]),*\s*([1-3])\s*$").unwrap();
                }
                println!("Enter position (as \"row, column\"):");
                let input = get_line_input();
                let caps = RE.captures(&input);
                match caps {
                    Some(cap) => {
                        let y = cap.get(1).map(|x| x.as_str().parse::<usize>().unwrap() - 1);
                        let x = cap.get(2).map(|x| x.as_str().parse::<usize>().unwrap() - 1);
                        match (y, x) {
                            (Some(i), Some(j)) if board.get_options().any(|pos| pos == (i, j)) => (i, j),
                            (Some(i), Some(j)) => {
                                println!("Your input \"({}, {})\" is pointing to an existing field, pick another field!", i + 1, j + 1);
                                self.get_action(board)
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        println!("Couldn't infer input from \"{}\", please try again...", input);
                        self.get_action(board)
                    }
                }
            }
        }
    }

    pub fn get_optimal_action(&self, board: &Board) -> (usize, usize) {
        let (outcome, optimal_options) = board.deduce_optimal_strategies();
        // println!("Game results in at worst a {} for {} (options are: {:?})", outcome, self, optimal_options);
        println!("Game results in at worst a {} for {}", outcome, self);
        let choice = optimal_options.choose(&mut rand::thread_rng());
        match choice {
            Some(c) => *c,
            None => unreachable!()
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.player_type {
            PlayerType::User => write!(f, "{} (user)", self.mark),
            PlayerType::Machine => write!(f, "{} (machine)", self.mark),
        }
    }
}
