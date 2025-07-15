use crate::{board::Board, color::*, file::*, piece::Piece, rank::Rank, square::Square};
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
    pub fn process(&self) -> BoardBuilder {
        let fields_as_str =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let fields: Vec<&str> = fields_as_str.split(" ").collect();
        let board = &mut BoardBuilder::new();

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
                'p' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Pawn, Color::Black));
                    curr_file = curr_file.right();
                }
                'r' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Rook, Color::Black));
                    curr_file = curr_file.right();
                }
                'n' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Knight, Color::Black));
                    curr_file = curr_file.right();
                }
                'b' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Bishop, Color::Black));
                    curr_file = curr_file.right();
                }
                'k' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::King, Color::Black));
                    curr_file = curr_file.right();
                }
                'q' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Queen, Color::Black));
                    curr_file = curr_file.right();
                }
                'P' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Pawn, Color::White));
                    curr_file = curr_file.right();
                }
                'R' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Rook, Color::White));
                    curr_file = curr_file.right();
                }
                'N' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Knight, Color::White));
                    curr_file = curr_file.right();
                }
                'B' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Bishop, Color::White));
                    curr_file = curr_file.right();
                }
                'K' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::King, Color::White));
                    curr_file = curr_file.right();
                }
                'Q' => {
                    board.pieces[Square::make_square(curr_rank, curr_file).to_index() as usize] =
                        Some((Piece::Queen, Color::White));
                    curr_file = curr_file.right();
                }
                _ => {}
            }
        }
        BoardBuilder {
            pieces: board.pieces,
            side_to_move: Color::White,
            en_passant: None,
        }
    }
}

#[cfg(test)]
mod fen {
    use super::*;
    use crate::color::Color::{Black, White};
    use crate::piece::Piece::*;

    #[test]
    fn correct_starting_pos() {
        let starting_position = [
            Some((Rook, White)),
            Some((Knight, White)),
            Some((Bishop, White)),
            Some((Queen, White)),
            Some((King, White)),
            Some((Bishop, White)),
            Some((Knight, White)),
            Some((Rook, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            Some((Pawn, White)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Pawn, Black)),
            Some((Rook, Black)),
            Some((Knight, Black)),
            Some((Bishop, Black)),
            Some((Queen, Black)),
            Some((King, Black)),
            Some((Bishop, Black)),
            Some((Knight, Black)),
            Some((Rook, Black)),
        ];
        //TODO: this should be done just on the type
        let empty_board_builder = BoardBuilder::new();
        let full = empty_board_builder.process();
        assert_eq!(full.pieces, starting_position)
    }
}
