// HashMap is probably less optimal but we can do a Vec3D later
use std::{collections::HashMap, slice::Iter};

use std::marker::Copy;

use crate::piece::Piece;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White = 1,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub pieces: [Option<(Piece, Color)>; 64],
}

impl Board {
    fn new() -> Board {
        Board { pieces: [None; 64] }
    }
    //TODO do pretty stuff with the Piece notation, not the coords and even better
    //pretty print
    pub fn pp(&self) {}

    fn from_string() {}
}
