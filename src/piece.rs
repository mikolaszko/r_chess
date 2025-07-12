#[derive(Debug)]
pub enum PieceName {
    King,
    Queen,
    //knight
    Night,
    Bishop,
    Rook,
    Pawn,
}

pub struct Piece {
    pub name: PieceName,
}

impl Piece {
    fn move_piece() {}
}
