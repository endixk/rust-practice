use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

lazy_static! {
    static ref SCORES: Vec<Vec<Vec<f32>>> = {
        let mut scores = Vec::new();
        for i in 0..10 {
            let mut file = File::open(format!("lib/ocr/train/{}.tsv", i)).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let mut mat = Vec::new();
            for line in content.split('\n') {
                let mut row = Vec::new();
                for score in line.split('\t') {
                    if score.len() > 0 {
                        row.push(score.parse::<f32>().unwrap());
                    }
                }
                if row.len() > 0 {
                    mat.push(row);
                }
            }
            scores.push(mat);
        }

        scores
    };
}

pub fn classify(mat: &Vec<Vec<bool>>) -> u8 {
    let mut best_score = 0.0;
    let mut best_digit = 0;
    for digit in 0..10 {
        let mut score = 0.0;
        for r in 0..mat.len() {
            for c in 0..mat[r].len() {
                if mat[r][c] {
                    score += SCORES[digit][r][c];
                } else {
                    score += 1.0 - SCORES[digit][r][c];
                }
            }
        }
        if score > best_score {
            best_score = score;
            best_digit = digit;
        }
    }
    best_digit as u8
}