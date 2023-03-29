use crate::solve::image_decoder::Puzzle;

struct PuzzleGrid {
    puzzle: Puzzle,
    grid: Vec<Vec<i8>>, // 0: unknown, 1: black, -1: white
    rounds: u32,
}

fn combine(a: Vec<i8>, b: Vec<i8>) -> Vec<i8> {
    let mut ret = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        if *x == -2 { // unassigned
            ret.push(*y);
        } else if *x == *y {
            ret.push(*x);
        } else {
            ret.push(0);
        }
    }
    ret
}
fn possible_locations(sizes: &Vec<u8>, status: Vec<i8>) -> Result<Vec<Vec<usize>>, String> {
    let mut ret = Vec::new();



    match ret.len() {
        0 => Err("No possible locations".to_string()),
        _ => Ok(ret),
    }
}

impl PuzzleGrid {
    fn new(puzzle: Puzzle) -> Self {
        let mut grid = Vec::new();
        for _ in 0..puzzle.size {
            let mut row = Vec::new();
            for _ in 0..puzzle.size {
                row.push(0);
            }
            grid.push(row);
        }
        Self {
            puzzle,
            grid,
            rounds: 0,
        }
    }

    fn row_solved(&self, r: usize) -> bool {
        self.grid[r].iter().filter(|&&x| x != 0).count() == self.puzzle.size
    }
    fn col_solved(&self, c: usize) -> bool {
        self.grid.iter().map(|row| row[c]).filter(|&x| x != 0).count() == self.puzzle.size
    }
    fn solved(&self) -> bool {
        for r in 0..self.puzzle.size {
            if !self.row_solved(r) {
                return false;
            }
        }
        for c in 0..self.puzzle.size {
            if !self.col_solved(c) {
                return false;
            }
        }
        true
    }

    fn _visualize(&self) {
        for row in &self.grid {
            for cell in row {
                if *cell == 0 {
                    print!("🔲");
                } else if *cell == 1 {
                    print!("🔳");
                } else {
                    print!("🚫");
                }
            }
            println!();
        }
    }

    fn solve(&mut self, loc: usize, is_row: bool) {
        let sizes = match is_row {
            true => &self.puzzle.row[loc],
            false => &self.puzzle.col[loc],
        };
        let status = match is_row {
            true => self.grid[loc].clone(),
            false => self.grid.iter().map(|row| row[loc]).collect(),
        };

        // permute over possible block arrangements
        let consensus = vec![-2; self.puzzle.size];
        for lvec in possible_locations(sizes, status).unwrap() {

        }
    }
}

pub fn solve(puzzle: Puzzle, verbosity: u8) {

}