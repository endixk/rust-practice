#[allow(dead_code)]

mod train;
mod solve;

use std::fs;

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
    solve::image_decoder::decode("lib/capture/IMG_2682.PNG");
}
