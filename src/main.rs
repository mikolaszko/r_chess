mod board;
mod fen;
mod file;
mod piece;
use std::{collections::HashMap, fs::File, path::Path};

use board::*;
use fen::*;
use piece::*;

fn main() {
    //replace with command line arg in the future;
    let file_string = Path::new("savefiles/init.fen").to_str().unwrap();
    let fen = BoardBuilder {
        file_path: String::from(file_string),
    };

    fen.process();
}
