use crate::train::digit_pix_generator::dissect;
use crate::train::ocr_classifier::classify;
use image::{self, RgbImage};

pub struct Puzzle {
    size: usize,
    row: Vec<Vec<u8>>,
    col: Vec<Vec<u8>>,
}

pub fn convert_row(img: &RgbImage, row: u32) -> Vec<u32> {
    let (width, _) = img.dimensions();
    let mut ret = Vec::new();
    for col in 0..width {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        ret.push(color);
    }
    ret
}

pub fn convert_col(img: &RgbImage, col: u32) -> Vec<u32> {
    let (_, height) = img.dimensions();
    let mut ret = Vec::new();
    for row in 0..height {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        ret.push(color);
    }
    ret
}

pub fn count_color_chunks(vec: &Vec<u32>, color: u32) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();
    let mut start = 0;
    let mut last = 0;
    for (i, c) in vec.iter().enumerate() {
        if *c != last {
            if last == color {
                ret.push((start, i as u32));
            }
            start = i as u32;
            last = *c;
        }
    }
    ret
}

fn _pixel_chunks(row: &Vec<bool>) -> u32 {
    let mut ret = 0;
    let mut prev = true;
    for pixel in row {
        if prev && !(*pixel) {
            ret += 1;
        }
        prev = *pixel;
    }
    ret
}

fn parse_column_digits(mat: Vec<Vec<bool>>, tolerance: i32) -> Vec<u8> {
    let mut ret = Vec::new();
    let digits = dissect(mat, true);

    let mut i = 0;
    while i < digits.len() {
        if i+1 < digits.len() &&
            (digits[i].max_r as i32 - digits[i+1].max_r as i32).abs() +
            (digits[i].min_r as i32 - digits[i+1].min_r as i32).abs() <= tolerance {
                if digits[i].max_c < digits[i+1].max_c {
                    ret.push(classify(&digits[i].mat) * 10 + classify(&digits[i+1].mat));
                } else {
                    ret.push(classify(&digits[i+1].mat) * 10 + classify(&digits[i].mat));
                }
                i += 1;
            }
        else {
            ret.push(classify(&digits[i].mat));
        }
        i += 1;
    }
    ret
}

fn parse_row_digits(mat: Vec<Vec<bool>>, tolerance: i32) -> Vec<u8> {
    let mut ret = Vec::new();
    let digits = dissect(mat, false);

    let mut i = 0;
    while i < digits.len() {
        if i+1 < digits.len() && digits[i+1].min_c as i32 - digits[i].max_c as i32 <= tolerance {
            ret.push(classify(&digits[i].mat) * 10 + classify(&digits[i+1].mat));
            i += 1;
        } else {
            ret.push(classify(&digits[i].mat));
        }
        i += 1;
    }
    ret
}

pub fn build_matrix(img: &RgbImage, ws: u32, we: u32, hs: u32, he: u32, color: u32) -> Vec<Vec<bool>> {
    let mut ret = Vec::new();
    for row in hs..he {
        let mut row_vec = Vec::new();
        for col in ws..we {
            let pixel = img.get_pixel(col, row);
            let pixel_color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
            row_vec.push(pixel_color == color);
        }
        ret.push(row_vec);
    }
    ret
}

pub fn get_image(path: &str) -> RgbImage {
    let img = image::open(path).unwrap();
    img.to_rgb8()
}

pub fn decode(path: &str) -> Puzzle {
    println!("Loading image: {}", path);
    let img = get_image(path);

    let (width, height) = img.dimensions();
    println!("Image size: {}x{}", width, height);

    let chunks = count_color_chunks(&convert_row(&img, 545), 0xeaedf7);
    let psize = chunks.len();
    println!("Puzzle size: {}x{}", psize, psize);

    let mut col = Vec::new();
    println!("Decoding columns...");
    for chunk in chunks {
        let mat = build_matrix(&img, chunk.0, chunk.1, 545, 745, 0xeaedf7);
        let digits = parse_column_digits(mat, 10);
        println!("{:?}", digits);
        col.push(digits);
    }

    let chunks = count_color_chunks(&convert_col(&img, 25), 0xeaedf7);
    assert_eq!(chunks.len(), psize);

    let mut row = Vec::new();
    println!("Decoding rows...");
    for chunk in chunks {
        let mat = build_matrix(&img, 25, 190, chunk.0, chunk.1, 0xeaedf7);
        let digits = parse_row_digits(mat, match psize {
            10 => 12,
            15 => 9,
            20 => 6,
            _ => panic!("Unknown puzzle size: {}", psize)
        });
        println!("{:?}", digits);
        row.push(digits);
    }

    Puzzle {size: psize, row, col}
}