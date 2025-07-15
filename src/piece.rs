use std::marker::Copy;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    King,
    Queen,
    //knight
    Knight,
    Bishop,
    Rook,
    Pawn,
}

impl Piece {
    fn move_piece() {}
}
