use crate::{
    board::{Board, Color},
    file::*,
    piece::Piece,
    rank::Rank,
    square::Square,
};
use std::fs;

#[derive(Copy, Clone)]
pub struct BoardBuilder {
    pieces: [Option<(Piece, Color)>; 64],
    side_to_move: Color,
    en_passant: Option<File>,
}

// based on FEN format
// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
impl BoardBuilder {
    pub fn new() -> BoardBuilder {
        BoardBuilder {
            pieces: [None; 64],
            side_to_move: Color::White,
            en_passant: None,
        }
    }
    pub fn process(&self) {
        let fields_as_str =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let fields: Vec<&str> = fields_as_str.split(" ").collect();
        let mut board = &mut BoardBuilder::new();

        let pieces = fields[0];
        let color = fields[1];
        let castling = fields[2];
        let ep = fields[3];

        let mut curr_file = File::A;
        let mut curr_rank = Rank::Eigth;

        for c in pieces.chars() {
            match c {
                '/' => {
                    curr_rank = curr_rank.down();
                    curr_file = File::A;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    // math from ascii table like in the case of 8 = 0 + 56 - 48
                    curr_file =
                        File::from_index(curr_file.to_index() + (c as usize) - '0' as usize);
                }
                'r' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index()] =
                        Some((Piece::Rook, Color::Black));
                    curr_file.right();
                }

                _ => {}
            }
        }
    }
}
