use crate::file::*;
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

        for c in pieces.chars() {
            match c {
                '/' => {
                    curr_file = curr_file.right();
                }
                _ => {}
            }
        }
        println!("{:?}", curr_file);
    }
}
