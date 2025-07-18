use crate::{bitboard::*, square::ALL_SQUARES};
use std::fs::File;
use std::io::Write;

static mut KNIGHT_MOVES: [Bitboard; 64] = [EMPTY; 64];

pub fn gen_knight_moves() {
    unsafe {
        for src in ALL_SQUARES.iter() {
            KNIGHT_MOVES[src.to_index()] = ALL_SQUARES
                .iter()
                .filter(|dest| {
                    let src_rank = src.get_rank().to_index() as i8;
                    let src_file = src.get_file().to_index() as i8;
                    let dest_rank = dest.get_rank().to_index() as i8;
                    let dest_file = dest.get_file().to_index() as i8;

                    ((src_rank - dest_rank).abs() == 2 && (src_file - dest_file).abs() == 1)
                        || ((src_rank - dest_rank).abs() == 1 && (src_file - dest_file).abs() == 2)
                })
                .fold(EMPTY, |b, s| b | Bitboard::from_square(*s));
        }
    }
}

// Write the KNIGHT_MOVES array to the specified file.
pub fn write_knight_moves(f: &mut File) {
    write!(f, "const KNIGHT_MOVES: [BitBoard; 64] = [\n").unwrap();
    for i in 0..64 {
        unsafe { write!(f, "    BitBoard({}),\n", KNIGHT_MOVES[i].0).unwrap() };
    }
    write!(f, "];\n").unwrap();
}
