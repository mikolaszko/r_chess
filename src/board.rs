// HashMap is probably less optimal but we can do a Vec3D later
use std::{collections::HashMap, slice::Iter};

use crate::piece::Piece;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl CLetter {
    pub fn iterator() -> Iter<'static, CLetter> {
        static CLETTERS: [CLetter; 8] = [
            CLetter::A,
            CLetter::B,
            CLetter::C,
            CLetter::D,
            CLetter::E,
            CLetter::F,
            CLetter::G,
            CLetter::H,
        ];

        CLETTERS.iter()
    }
}

#[derive(Eq, Hash, Debug, PartialOrd, Ord, PartialEq)]
pub struct Square {
    pub number: u8,
    pub letter: &'static CLetter,
}

impl Square {
    fn to_string(&self) -> String {
        format!("{:?}{:?}", self.letter, self.number)
    }
}

pub struct Board {
    pub board_state: HashMap<Square, Option<Piece>>,
}

impl Board {
    //TODO do pretty stuff with the Piece notation, not the coords and even better
    //pretty print
    pub fn pp(&self) {
        let mut sorted: Vec<_> = self.board_state.iter().collect();
        sorted.sort_by_key(|a| a.0);
        for (square, piece) in sorted.iter() {
            match piece {
                Some(piece) => println!("{:?} {:?}", square.to_string(), piece.name),
                _ => {}
            }
        }
    }
}
