use crate::{file::*, rank::Rank};
use std::fs;

pub struct BoardBuilder {
    pub file_path: String,
}

// based on FEN format
// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
impl BoardBuilder {
    pub fn process(&self) {
        let fields_as_str = fs::read_to_string(&self.file_path).unwrap();
        let fields: Vec<&str> = fields_as_str.split(" ").collect();

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

                _ => {}
            }
        }
    }
}
