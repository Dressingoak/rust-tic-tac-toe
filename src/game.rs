use crate::mark::Mark;
use crate::outcome::Outcome;
use crate::player::PlayerType;
use crate::player::Player;
use crate::board::Board;

pub struct Game {
    board: Board,
    players: (Player, Player),
}

fn get_line_input() -> String {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}

impl Game {

    pub fn create(start: Mark, player1: Player, player2: Player) -> Game {
        Game {
            board: Board::empty(start),
            players: (player1, player2),
        }
    }

    pub fn against_the_machine() {
        use std::str::FromStr;

        println!("Would you like to be X or O?");
        let mark = loop {
            let input = get_line_input();
            match Mark::from_str(&input) {
                Ok(m) => break m,
                Err(v) => println!("{}", v),
            }
        };
        loop {
            let start: Mark = rand::random();
            let player1 = Player::create(mark, PlayerType::User);
            let player2 = Player::create(mark.reverse(), PlayerType::Machine);
            println!("You choose {}. {} starts.", mark, start);
            let mut game = Game::create(start, player1, player2);
            game.play();
            println!("Would you like to play again (y/n)?");
            let replay: bool = loop {
                let input = get_line_input();
                match char::from_str(&input) {
                    Ok('y') => break true,
                    Ok('n') => break false,
                    Ok(c) => println!("Type 'y' or 'n' (you typed '{}')", c),
                    Err(v) => println!("{}", v),
                }
            };
            if !replay {
                break;
            }
        }
    }

    fn get_player_by_mark(&self, mark: &Mark) -> &Player {
        if mark == self.players.0.get_mark() {
            &self.players.0
        } else {
            &self.players.1
        }
    }

    pub fn play(&mut self) -> Outcome {
        loop {
            match self.board.get_outcome() {
                Some(outcome) => {
                    println!("{}", self);
                    break *outcome
                },
                None => {
                    let turn = self.board.get_turn().clone();
                    let player = self.get_player_by_mark(&turn);
                    println!("{}", self);
                    let choice = player.get_action(&self.board);
                    println!("Player {} puts its mark at ({}, {}).", player, choice.0 + 1, choice.1 + 1);
                    self.board.set(choice, turn);
                }
            }
        }
    }

}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Turns: {}, ", self.board.count());
        match self.board.get_outcome() {
            Some(Outcome::Victory(mark)) => writeln!(f, "ended with victory for {}", self.get_player_by_mark(mark)),
            Some(Outcome::Draw) => writeln!(f, "ended in draw"),
            None => writeln!(f, "up next is {}", self.get_player_by_mark(self.board.get_turn())),
        };
        writeln!(f, "{}", self.board)
    }
}
