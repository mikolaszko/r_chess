mod board;
mod color;
mod fen;
mod file;
mod piece;
mod rank;
mod square;

use fen::*;

fn main() {
    //replace with command line arg in the future;
    //let file_string = Path::new("savefiles/init.fen").to_str().unwrap();
    let fen = BoardBuilder::new();
    fen.process();
}
