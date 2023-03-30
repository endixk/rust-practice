#[allow(dead_code)]

mod train;
mod solve;

use std::fs;
use clap::Parser;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image of the puzzle
    #[arg(short, long)]
    input: String,

    /// Report puzzle statistics
    #[arg(short, long)]
    report: Option<String>,

    /// Show result as numbers instead of colors
    #[arg(short, long)]
    sfw: bool,

    /// Program verbosity
    #[arg(short, long, default_value_t = 0)]
    verbose: u8,
}


fn main() {
    // _generate();
    // _train();
    let args = Args::parse();
    let puzzle = image_decoder::decode(&args.input, args.verbose);
    puzzle_solver::solve(puzzle, args.report, args.sfw, args.verbose);
}
