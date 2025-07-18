use core::fmt;
use std::ops::{BitOr, BitXor, BitXorAssign};

use crate::square::Square;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Bitboard(pub u64);

pub const EMPTY: Bitboard = Bitboard(0);

impl Bitboard {
    pub fn new(i: u64) -> Self {
        Bitboard(i)
    }
    pub fn from_square(sqr: Square) -> Bitboard {
        Bitboard::new(1u64 >> sqr.to_int())
    }

    pub fn to_square(&self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Bitboard) {
        self.0 ^= other.0;
    }
}

impl fmt::Display for Bitboard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if x % 8 == 7 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}
/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for Bitboard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= Bitboard::from_square(result);
            Some(result)
        }
    }
}
