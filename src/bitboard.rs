use std::ops::BitXor;

pub struct Bitboard(u64);

impl BitXor for Bitboard {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
