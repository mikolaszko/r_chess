use std::fs::File;
use std::path::Path;

mod bitboard;
mod board;
mod color;
mod fen;
mod file;
mod moves;
mod piece;
mod rank;
mod square;

use fen::*;
use moves::knight;

use crate::moves::knight::write_knight_moves;

fn main() {
    //replace with command line arg in the future;
    //let file_string = Path::new("savefiles/init.fen").to_str().unwrap();
    let fen = BoardBuilder::new();
    fen.process();

    knight::gen_knight_moves();
    let magic_path = Path::new("./outdir/").join("magic_gen.rs");
    let mut f = File::create(&magic_path).unwrap();
    write_knight_moves(&mut f);
}
