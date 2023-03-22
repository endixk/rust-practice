use std::fs::{self, File};
use std::io::{Read, Write};

fn read_file_to_matrix(path: &str) -> Vec<Vec<bool>> {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut lines = content.split('\n');
    let mut arr = Vec::new();
    while let Some(line) = lines.next() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '1');
        }
        if row.len() > 0 {
            arr.push(row);
        }
    }
    arr
}

fn _visualize_matrix(mat: &Vec<Vec<i32>>) {
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

fn train_digit(paths: Vec<String>, sw: usize, sh: usize, output_path: String){
    let mut scores = vec![vec![0; sw]; sh];
    let size = paths.len();
    for path in paths {
        let mat = read_file_to_matrix(&path);
        assert_eq!(mat[0].len(), sw);
        assert_eq!(mat.len(), sh);

        for r in 0..sh {
            for c in 0..sw {
                if mat[r][c] {
                    scores[r][c] += 1;
                }
            }
        }
    }

    let mut file = File::create(output_path).unwrap();
    for r in 0..sh {
        for c in 0..sw {
            let score = format!("{:.5}", scores[r][c] as f32 / size as f32);
            file.write_all(score.as_bytes()).unwrap();
            file.write_all(b"\t").unwrap();
        }
        file.write_all(b"\n").unwrap();
    }
}

pub fn train() {
    for digit in 0..10 {
        let paths = fs::read_dir(format!("lib/ocr/label/{}", digit)).unwrap();
        let mut paths: Vec<String> = paths.map(|p| p.unwrap().path().to_str().unwrap().to_string()).collect();
        train_digit(paths, 50, 50, format!("lib/ocr/train/{}.tsv", digit));
    }
}