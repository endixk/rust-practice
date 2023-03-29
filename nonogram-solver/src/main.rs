#[allow(dead_code)]

mod train;
mod solve;

use std::fs;
use std::env::args;
use solve::*;

fn _generate() {
    let paths = fs::read_dir("lib/capture").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".png") || path.ends_with(".PNG") {
            train::digit_pix_generator::generate(path);
        }
    }
}

fn _train() {
    train::ocr_trainer::train();
}

fn main() {
    // _generate();
    // _train();
    let path = args().nth(1).unwrap();
    let verbosity = args().nth(2).unwrap().parse::<u8>().unwrap();

    let puzzle = image_decoder::decode(&path, verbosity);
    puzzle_solver::solve(puzzle, verbosity);
}
