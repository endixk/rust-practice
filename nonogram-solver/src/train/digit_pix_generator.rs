use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::Write;
use crate::solve::image_decoder::*;

fn flood_fill_tb(chunk: &Vec<Vec<bool>>, arr: &mut Vec<Vec<u8>>) -> u8 {
    let mut queue = Vec::new();
    let mut visited = vec![vec![false; chunk[0].len()]; chunk.len()];
    let mut color = 0;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    for r in 0..chunk.len() {
        for c in 0..chunk[r].len() {
            if visited[r][c] || chunk[r][c] {
                continue;
            }
            color += 1;
            queue.push((r, c));
            while let Some((r, c)) = queue.pop() {
                if visited[r][c] || chunk[r][c] {
                    continue;
                }
                visited[r][c] = true;
                arr[r][c] = color;
                for i in 0..4 {
                    let nr = r as i32 + dy[i];
                    let nc = c as i32 + dx[i];
                    if nr < 0 || nr >= chunk.len() as i32 || nc < 0 || nc >= chunk[r].len() as i32 {
                        continue;
                    }
                    queue.push((nr as usize, nc as usize));
                }
            }
        }
    }

    color
}
fn flood_fill_lr(chunk: &Vec<Vec<bool>>, arr: &mut Vec<Vec<u8>>) -> u8 {
    let mut queue = Vec::new();
    let mut visited = vec![vec![false; chunk[0].len()]; chunk.len()];
    let mut color = 0;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    for c in 0..chunk[0].len() {
        for r in 0..chunk.len() {
            if visited[r][c] || chunk[r][c] {
                continue;
            }
            color += 1;
            queue.push((r, c));
            while let Some((r, c)) = queue.pop() {
                if visited[r][c] || chunk[r][c] {
                    continue;
                }
                visited[r][c] = true;
                arr[r][c] = color;
                for i in 0..4 {
                    let nr = r as i32 + dy[i];
                    let nc = c as i32 + dx[i];
                    if nr < 0 || nr >= chunk.len() as i32 || nc < 0 || nc >= chunk[r].len() as i32 {
                        continue;
                    }
                    queue.push((nr as usize, nc as usize));
                }
            }
        }
    }

    color
}

fn bound(arr: &Vec<Vec<u8>>, color: u8) -> (usize, usize, usize, usize) {
    let mut min_r = arr.len();
    let mut max_r = 0;
    let mut min_c = arr[0].len();
    let mut max_c = 0;
    for r in 0..arr.len() {
        for c in 0..arr[r].len() {
            if arr[r][c] == color {
                min_r = min_r.min(r);
                max_r = max_r.max(r);
                min_c = min_c.min(c);
                max_c = max_c.max(c);
            }
        }
    }

    (min_r, max_r, min_c, max_c)
}
fn crop(arr: &Vec<Vec<u8>>, color: u8) -> Vec<Vec<bool>> {
    let (min_r, max_r, min_c, max_c) = bound(arr, color);
    let mut ret = vec![vec![false; max_c - min_c + 1]; max_r - min_r + 1];
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            ret[r - min_r][c - min_c] = arr[r][c] == color;
        }
    }

    ret
}
fn resize(mat: Vec<Vec<bool>>, sw: usize, sh: usize) -> Vec<Vec<bool>> {
    let mut ret = vec![vec![false; sw]; sh];
    for r in 0..sh {
        for c in 0..sw {
            ret[r][c] = mat[r * mat.len() / sh][c * mat[0].len() / sw];
        }
    }
    ret
}

fn _visualize_matrix(mat: &Vec<Vec<bool>>) {
    for r in 0..mat.len() {
        for c in 0..mat[r].len() {
            if mat[r][c] {
                print!("⬛️");
            } else {
                print!("⬜️");
            }
        }
        println!();
    }
}

pub struct Digit {
    pub mat: Vec<Vec<bool>>,
    pub min_r: usize,
    pub max_r: usize,
    pub min_c: usize,
    pub max_c: usize,
}

pub fn dissect(chunk: Vec<Vec<bool>>, col: bool) -> Vec<Digit> {
    let mut arr = vec![vec![0 as u8; chunk[0].len()]; chunk.len()];
    let cnt = match col {
        true => flood_fill_tb(&chunk, &mut arr),
        false => flood_fill_lr(&chunk, &mut arr)
    };

    let mut ret = Vec::new();
    for color in 1..=cnt {
        let (min_r, max_r, min_c, max_c) = bound(&arr, color);
        ret.push(Digit {
            mat: resize(crop(&arr, color), 50, 50),
            min_r, max_r, min_c, max_c
        });
    }

    ret
}

fn record(chunk: Vec<Vec<bool>>, header: String, col: bool) {
    for (color, digit) in dissect(chunk, col).iter().enumerate() {
        // write matrix to file
        let mut file = File::create(format!("lib/ocr/raw/pix_{}_{}.txt", header, color)).unwrap();
        for r in 0..digit.mat.len() {
            for c in 0..digit.mat[r].len() {
                if digit.mat[r][c] {
                    file.write_all(b"1").unwrap();
                } else {
                    file.write_all(b"0").unwrap();
                }
            }
            file.write_all(b"\n").unwrap();
        }
    }
}

fn hash_path(path: &str) -> String {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn generate(path: &str) {
    println!("Loading image: {}", path);
    let header = hash_path(path);
    let img = get_image(path);

    let (width, height) = img.dimensions();
    println!("Image size: {}x{}", width, height);

    let chunks = count_color_chunks(&convert_row(&img, 545), 0xeaedf7);
    println!("Puzzle size: {}x{}", chunks.len(), chunks.len());

    println!("Dissecting columns...");
    for chunk in chunks {
        let mat = build_matrix(&img, chunk.0, chunk.1, 545, 745, 0xeaedf7);
        record(mat, format!("{}_col{}", header, chunk.0), true);
    }

    println!("Dissecting rows...");
    let chunks = count_color_chunks(&convert_col(&img, 25), 0xeaedf7);
    for chunk in chunks {
        let mat = build_matrix(&img, 25, 190, chunk.0, chunk.1, 0xeaedf7);
        record(mat, format!("{}_row{}", header, chunk.0), false);
    }

    println!("Done!\n");
}