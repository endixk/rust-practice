use image::{self, RgbImage};

pub struct Puzzle {
}

fn get_size(img: &RgbImage) -> (u32, u32) {
    let (width, height) = img.dimensions();
    (width, height)
}

fn convert_row(img: &RgbImage, row: u32) -> Vec<u32> {
    let (width, _) = get_size(img);
    let mut result = Vec::new();
    for col in 0..width {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        result.push(color);
    }
    result
}

fn convert_col(img: &RgbImage, col: u32) -> Vec<u32> {
    let (_, height) = get_size(img);
    let mut result = Vec::new();
    for row in 0..height {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        result.push(color);
    }
    result
}

fn count_color_chunks(vec: &Vec<u32>, color: u32) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut last = 0;
    for (i, c) in vec.iter().enumerate() {
        if *c != last {
            if last == color {
                result.push((start, i as u32));
            }
            start = i as u32;
            last = *c;
        }
    }
    result
}

fn parse_digits(mat: &Vec<Vec<bool>>) -> Vec<u8> {
    unimplemented!();
}

fn build_matrix(img: &RgbImage, ws: u32, we: u32, hs: u32, he: u32, color: u32) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for row in hs..he {
        let mut row_vec = Vec::new();
        for col in ws..we {
            let pixel = img.get_pixel(col, row);
            let pixel_color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
            row_vec.push(pixel_color == color);
        }
        result.push(row_vec);
    }
    result
}

pub fn decode(path: &str) -> Puzzle {
    println!("Loading image: {}", path);
    let img = image::open(path).unwrap();
    let img = img.to_rgb8();

    let (width, height) = get_size(&img);
    println!("Image size: {}x{}", width, height);

    let chunks = count_color_chunks(&convert_col(&img, 25), 0xeaedf7);
    println!("Puzzle size: {}x{}", chunks.len(), chunks.len());

    println!("Decoding columns...");
    for chunk in chunks {
        let _mat = build_matrix(&img, 545, 745, chunk.0, chunk.1, 0xeaedf7);
    }

    Puzzle {}
}