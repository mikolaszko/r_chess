use std::marker::Copy;

use crate::color::Color;
use crate::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub pieces: [Option<(Piece, Color)>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board { pieces: [None; 64] }
    }
    //TODO do pretty stuff with the Piece notation, not the coords and even better
    //pretty print
    pub fn pp(&self) {}
    fn from_string() {}
}
