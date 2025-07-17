use core::fmt;
use std::ops::BitXor;

pub struct Bitboard(u64);

impl BitXor for Bitboard {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
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
