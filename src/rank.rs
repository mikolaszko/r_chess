#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eigth = 7,
}

const NUM_RANKS: usize = 8;

const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eigth,
];

impl Rank {
    #[inline]
    pub fn from_index(i: usize) -> Rank {
        match i & 7 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eigth,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn down(&self) -> Rank {
        Rank::from_index(self.to_index().wrapping_sub(1))
    }

    #[inline]
    pub fn up(&self) -> Rank {
        Rank::from_index(self.to_index() + 1)
    }

    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}
