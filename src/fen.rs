use crate::board::*;
use crate::piece::*;
use std::{collections::HashMap, fs};
pub struct Fen {
    pub file_path: String,
}

impl Fen {
    pub fn process(&self) {
        let board_state: HashMap<Square, Option<Piece>> = HashMap::new();
        let fields_as_str = fs::read_to_string(&self.file_path).unwrap();
        let fields: Vec<&str> = fields_as_str.split(|c| c == '/' || c == ' ').collect();
        println!("{:?}", fields);

        if let Some(x) = fields.iter().next() {
            match x.to_owned() {
                "rnbqkbnr" => {
                    println!("White Initial state")
                }
                _ => {}
            }
        }
    }
}
