mod board;
mod piece;
use std::collections::HashMap;

use board::*;
use piece::*;

fn main() {
    let mut init: HashMap<Square, Option<Piece>> = HashMap::new();
    // notation
    let board = Board { board_state: init };
    board.pp();
}
