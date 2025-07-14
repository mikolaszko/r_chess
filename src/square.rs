use crate::{file::File, rank::Rank};

pub struct Square(u8);

impl Square {
    pub fn new(i: u8) -> Square {
        Square(i & 63)
    }
    // ie.
    // 56 + 7 == 63
    // 00111000 ^ 0000111 =
    pub fn make_square(rank: Rank, file: File) -> Square {
        Square((rank.to_index() as u8) << 3 ^ (file.to_index() as u8))
    }

    pub fn to_index(&self) -> u8 {
        self.0 as u8
    }
}
