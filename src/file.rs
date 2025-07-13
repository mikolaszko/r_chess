#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

const NUM_FILES: usize = 8;

const ALL_FILES: [File; NUM_FILES] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    pub fn from_index(i: usize) -> File {
        match i & 7 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
    pub fn left(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    pub fn right(&self) -> File {
        File::from_index(self.to_index() + 1)
    }

    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}
