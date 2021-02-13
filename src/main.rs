#[macro_use] extern crate lazy_static;
extern crate regex;

mod mark;
mod outcome;
mod board;
mod player;
mod game;

use game::Game;

fn main() {
    Game::against_the_machine();
}
