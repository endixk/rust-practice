use image::{self, RgbImage};

pub struct Puzzle {
}

fn get_size(img: &RgbImage) -> (u32, u32) {
    let (width, height) = img.dimensions();
    (width, height)
}

fn convert_row(img: &RgbImage, row: u32) -> Vec<u32> {
    let (width, _) = get_size(img);
    let mut ret = Vec::new();
    for col in 0..width {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        ret.push(color);
    }
    ret
}

fn convert_col(img: &RgbImage, col: u32) -> Vec<u32> {
    let (_, height) = get_size(img);
    let mut ret = Vec::new();
    for row in 0..height {
        let pixel = img.get_pixel(col, row);
        let color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
        ret.push(color);
    }
    ret
}

fn count_color_chunks(vec: &Vec<u32>, color: u32) -> Vec<(u32, u32)> {
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

fn pixel_chunks(row: &Vec<bool>) -> u32 {
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

fn parse_digit(mat: &Vec<Vec<bool>>) -> u8 {
    let mut row_chunks = Vec::new();
    for row in mat {
        row_chunks.push(pixel_chunks(row));
    }
    row_chunks.dedup();

    let mut col_chunks = Vec::new();
    for col in 0..mat[0].len() {
        let mut col_vec = Vec::new();
        for row in mat {
            col_vec.push(row[col]);
        }
        col_chunks.push(pixel_chunks(&col_vec));
    }
    col_chunks.dedup();

    println!("{:?}\n{:?}\n", row_chunks, col_chunks);

    // classify
    return if row_chunks == vec![1, 2, 1] && col_chunks == vec![0, 1, 0] {
        1
    } else if row_chunks == vec![1, 2, 1] && col_chunks == vec![0, 2, 3, 2, 0] {
        2
    } else if row_chunks == vec![1, 2, 1, 2, 1] && col_chunks == vec![0, 1, 2, 3, 2, 1, 0] {
        3
    } else if row_chunks == vec![1, 2, 1] && col_chunks == vec![0, 1, 2, 1, 0] {
        4
    } else if row_chunks.len() == 7 && col_chunks.len() == 6 {
        5
    } else {
        0
    }
}

fn parse_column_digits(mat: &Vec<Vec<bool>>) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut buf = Vec::new();
    for row in mat {
        if row.contains(&false) {
            buf.push(row.clone());
        } else {
            if buf.len() > 0 {
                ret.push(parse_digit(&buf));
                buf.clear();
            }
        }
    }
    ret
}

fn parse_row_digits(mat: &Vec<Vec<bool>>) -> Vec<u8> {
    unimplemented!();
}

fn build_matrix(img: &RgbImage, ws: u32, we: u32, hs: u32, he: u32, color: u32) -> Vec<Vec<bool>> {
    let mut ret = Vec::new();
    for row in hs..he {
        let mut row_vec = Vec::new();
        for col in ws..we {
            let pixel = img.get_pixel(col, row);
            let pixel_color = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
            row_vec.push(pixel_color != color);
        }
        ret.push(row_vec);
    }
    ret
}

pub fn decode(path: &str) -> Puzzle {
    println!("Loading image: {}", path);
    let img = image::open(path).unwrap();
    let img = img.to_rgb8();

    let (width, height) = get_size(&img);
    println!("Image size: {}x{}", width, height);

    let chunks = count_color_chunks(&convert_row(&img, 545), 0xeaedf7);
    println!("Puzzle size: {}x{}", chunks.len(), chunks.len());
    println!("Decoding columns...");
    for chunk in chunks {
        let mat = build_matrix(&img, chunk.0, chunk.1, 545, 745, 0x271f56);
        let digits = parse_column_digits(&mat);
    }

    let chunks = count_color_chunks(&convert_col(&img, 25), 0xeaedf7);
    println!("Decoding rows...");
    for chunk in chunks {
        let mat = build_matrix(&img, 25, 200, chunk.0, chunk.1, 0x271f56);
        // let digits = parse_row_digits(&mat);
    }

    Puzzle {}
}