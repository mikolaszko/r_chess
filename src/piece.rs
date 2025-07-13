use std::marker::Copy;

#[derive(Debug, Copy, Clone)]
pub enum PieceName {
    King,
    Queen,
    //knight
    Night,
    Bishop,
    Rook,
    Pawn,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub name: PieceName,
}

impl Piece {
    fn move_piece() {}
}
