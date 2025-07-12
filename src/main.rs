use std::slice::Iter;

#[derive(Debug, PartialEq, Eq)]
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

struct Square {
    number: u8,
    letter: &'static CLetter,
}

impl Square {
    fn print(&self) -> String {
        format!("{:?}{:?}", self.letter, self.number)
    }
}

struct Board {
    pieces: Vec<u8>,
    squares: Vec<Square>,
}

impl Board {
    //TODO do pretty stuff with the Piece notation, not the algebraic notation
    //pretty print
    fn pp(&self) {
        for x in &self.squares {
            match x.letter {
                &CLetter::H => println!("{}|", x.print()),
                &CLetter::A => print!("|{}|", x.print()),
                _ => {
                    print!("{}|", x.print())
                }
            }
        }
    }
}

fn main() {
    let mut square_vec: Vec<Square> = vec![];
    for x in 1..9 {
        for letter in CLetter::iterator() {
            square_vec.push(Square {
                number: x,
                letter: letter,
            })
        }
    }
    let board = Board {
        pieces: vec![],
        squares: square_vec,
    };
    board.pp();
}
