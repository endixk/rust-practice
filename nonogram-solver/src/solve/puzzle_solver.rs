use std::io::stdin;
use itertools::Itertools;
use crate::solve::image_decoder::Puzzle;

fn char(n: u8) -> char {
    match n {
        0..=9 => (n as u8 + b'0') as char,
        10..=35 => (n as u8 + b'A' - 10) as char,
        _ => panic!("char: n out of range"),
    }
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

fn valid_loc(sizes: &Vec<u8>, status: &Vec<i8>, locs: &Vec<usize>) -> bool {
    // check that the number of blocks matches the number of locations
    assert_eq!(sizes.len(), locs.len());

    // build a vector
    let mut vec = vec![-1; status.len()];
    for (&size, &loc) in sizes.iter().zip(locs.iter()) {
        if loc > 0 && vec[loc - 1] == 1 {
            // two blocks are adjacent
            return false;
        }
        for i in loc..loc + size as usize {
            if i >= vec.len() {
                // block is out of bounds
                return false;
            }
            if vec[i] == 1 {
                // two blocks overlap
                return false;
            }
            vec[i] = 1;
        }
    }

    // check that the blocks match the status
    for (x, y) in vec.iter().zip(status.iter()) {
        if *y != 0 && *x != *y {
            return false;
        }
    }

    true
}

fn possible_locations(sizes: &Vec<u8>, status: Vec<i8>) -> Result<Vec<Vec<usize>>, String> {
    let mut ret = Vec::new();

    for locs in (0..status.len()).combinations(sizes.len()) {
        if valid_loc(sizes, &status, &locs) {
            ret.push(locs);
        }
    }

    match ret.len() {
        0 => Err("No possible locations".to_string()),
        _ => Ok(ret),
    }
}

struct PuzzleGrid {
    puzzle: Puzzle,
    grid: Vec<Vec<i8>>, // 0: unknown, 1: black, -1: white
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

    // free width = number of available locations - minimum width to place blocks
    fn free_width(&self, loc: usize, is_row: bool) -> u8 {
        let sizes = match is_row {
            true => &self.puzzle.row[loc],
            false => &self.puzzle.col[loc],
        };
        let status = match is_row {
            true => self.grid[loc].clone(),
            false => self.grid.iter().map(|row| row[loc]).collect(),
        };

        status.iter().filter(|&x| *x >= 0).count() as u8 - sizes.iter().sum::<u8>() - sizes.len() as u8 + 1
    }

    // dof = number of possible block arrangements
    fn dof(&self, loc: usize, is_row: bool) -> u32 {
        let sizes = match is_row {
            true => &self.puzzle.row[loc],
            false => &self.puzzle.col[loc],
        };
        let status = match is_row {
            true => self.grid[loc].clone(),
            false => self.grid.iter().map(|row| row[loc]).collect(),
        };

        possible_locations(sizes, status).unwrap().len() as u32
    }

    // number of updates to the grid
    fn info_gain(&self, loc: usize, is_row: bool) -> u8 {
        let sizes = match is_row {
            true => &self.puzzle.row[loc],
            false => &self.puzzle.col[loc],
        };
        let status = match is_row {
            true => self.grid[loc].clone(),
            false => self.grid.iter().map(|row| row[loc]).collect(),
        };

        let mut consensus = vec![-2; self.puzzle.size];
        for lvec in possible_locations(sizes, status.clone()).unwrap() {
            let mut arr = vec![-1; self.puzzle.size];
            for (&size, &loc) in sizes.iter().zip(lvec.iter()) {
                for i in loc..loc + size as usize {
                    arr[i] = 1;
                }
            }
            consensus = combine(consensus, arr);
        }

        status.iter().zip(consensus.iter()).filter(|(&x, &y)| x == 0 && y != 0).count() as u8
    }

    fn visualize(&self) {
        let mut rfmt = Vec::new();
        for row in &self.puzzle.row {
            let mut r = String::new();
            for &digit in row {
                r.push_str(format!("{} ", char(digit)).as_str());
            }
            rfmt.push(r);
        }
        let rmax = rfmt.iter().map(|s| s.len()).max().unwrap();

        let &cmax = &self.puzzle.col.iter().map(|x| x.len()).max().unwrap();
        for i in (0..cmax).rev() {
            print!("{:>width$}", "", width=rmax+1);
            for col in &self.puzzle.col {
                if i < col.len() {
                    print!("{} ", char(col[col.len()-i-1]));
                } else {
                    print!("  ");
                }
            }
            println!();
        }
        for (rstr, row) in rfmt.iter().zip(&self.grid) {
            print!("{:>width$}", rstr, width=rmax);
            for cell in row {
                if *cell == 0 {
                    print!("⬜️");
                } else if *cell == 1 {
                    print!("⬛️");
                } else {
                    print!("🔲");
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
        let mut consensus = vec![-2; self.puzzle.size];
        for lvec in possible_locations(sizes, status).unwrap() {
            let mut arr = vec![-1; self.puzzle.size];
            for (&size, &loc) in sizes.iter().zip(lvec.iter()) {
                for i in loc..loc + size as usize {
                    arr[i] = 1;
                }
            }
            consensus = combine(consensus, arr);
        }

        // update the grid
        match is_row {
            true => {
                for (i, &x) in consensus.iter().enumerate() {
                    if self.grid[loc][i] == 0 {
                        assert_ne!(x, -2);
                        self.grid[loc][i] = x;
                    }
                }
            },
            false => {
                for (i, &x) in consensus.iter().enumerate() {
                    if self.grid[i][loc] == 0 {
                        assert_ne!(x, -2);
                        self.grid[i][loc] = x;
                    }
                }
            },
        }
    }
}

struct GridVector {
    loc: usize,
    is_row: bool,
    cargo: i32,
}

struct SolveResult {
    grid: PuzzleGrid,
    strategy: String,
    rounds: u32,
    guesses: u32,
}
impl SolveResult {
    fn min(self, other: SolveResult) -> Self {
        if self.guesses < other.guesses {
            self
        } else if self.guesses > other.guesses {
            other
        } else {
            if self.rounds < other.rounds {
                self
            } else {
                other
            }
        }
    }
}

fn strategy_simple(puzzle: Puzzle, verbosity: u8) -> SolveResult {
    let mut grid = PuzzleGrid::new(puzzle);
    let strategy = String::from("Simple iteration");
    let mut rounds = 0;
    let mut guesses = 0;

    if verbosity > 3 {
        println!("Strategy: {}", strategy);
        println!("Press enter to start...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }

    while !grid.solved() {
        for r in 0..grid.puzzle.size {
            if !grid.row_solved(r) {
                grid.solve(r, true);
                guesses += 1;
            }
        }
        for c in 0..grid.puzzle.size {
            if !grid.col_solved(c) {
                grid.solve(c, false);
                guesses += 1;
            }
        }
        rounds += 1;
        if verbosity > 3 {
            println!("Round {}", rounds);
            grid.visualize();
            println!("Press enter to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }

    if verbosity > 0 {
        println!("{} strategy: {} rounds, {} guesses", strategy, rounds, guesses);
    }
    SolveResult { grid, strategy, rounds, guesses }
}
fn strategy_outskirt(puzzle: Puzzle, verbosity: u8) -> SolveResult {
    let mut grid = PuzzleGrid::new(puzzle);
    let strategy = String::from("Outskirt first");
    let mut rounds = 0;
    let mut guesses = 0;

    if verbosity > 3 {
        println!("Strategy: {}", strategy);
        println!("Press enter to start...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }

    while !grid.solved() {
        for d in 0..grid.puzzle.size / 2 {
            if !grid.row_solved(d) {
                grid.solve(d, true);
                guesses += 1;
            }
            if !grid.row_solved(grid.puzzle.size - d - 1) {
                grid.solve(grid.puzzle.size - d - 1, true);
                guesses += 1;
            }
            if !grid.col_solved(d) {
                grid.solve(d, false);
                guesses += 1;
            }
            if !grid.col_solved(grid.puzzle.size - d - 1) {
                grid.solve(grid.puzzle.size - d - 1, false);
                guesses += 1;
            }
        }
        if grid.puzzle.size % 2 == 1 {
            if !grid.row_solved(grid.puzzle.size / 2) {
                grid.solve(grid.puzzle.size / 2, true);
                guesses += 1;
            }
            if !grid.col_solved(grid.puzzle.size / 2) {
                grid.solve(grid.puzzle.size / 2, false);
                guesses += 1;
            }
        }

        rounds += 1;
        if verbosity > 3 {
            println!("Round {}", rounds);
            grid.visualize();
            println!("Press enter to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }

    if verbosity > 0 {
        println!("{} strategy: {} rounds, {} guesses", strategy, rounds, guesses);
    }

    SolveResult { grid, strategy, rounds, guesses }
}
fn strategy_free_width(puzzle: Puzzle, verbosity: u8) -> SolveResult {
    let mut grid = PuzzleGrid::new(puzzle);
    let strategy = String::from("Free width");
    let mut rounds = 0;
    let mut guesses = 0;

    if verbosity > 3 {
        println!("Strategy: {}", strategy);
        println!("Press enter to start...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }

    while !grid.solved() {
        let mut gvecs = Vec::new();
        for r in 0..grid.puzzle.size {
            if !grid.row_solved(r) {
                gvecs.push(GridVector { loc: r, is_row: true, cargo: grid.free_width(r, true) as i32 });
            }
        }
        for c in 0..grid.puzzle.size {
            if !grid.col_solved(c) {
                gvecs.push(GridVector { loc: c, is_row: false, cargo: grid.free_width(c, false) as i32 });
            }
        }
        gvecs.sort_by(|a, b| a.cargo.cmp(&b.cargo));

        for gvec in gvecs {
            match gvec.is_row {
                true if grid.row_solved(gvec.loc) => continue,
                false if grid.col_solved(gvec.loc) => continue,
                _ => (),
            }
            grid.solve(gvec.loc, gvec.is_row);
            guesses += 1;
        }

        rounds += 1;
        if verbosity > 3 {
            println!("Round {}", rounds);
            grid.visualize();
            println!("Press enter to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }

    if verbosity > 0 {
        println!("{} strategy: {} rounds, {} guesses", strategy, rounds, guesses);
    }

    SolveResult { grid, strategy, rounds, guesses }
}
fn strategy_dof(puzzle: Puzzle, verbosity: u8) -> SolveResult {
    let mut grid = PuzzleGrid::new(puzzle);
    let strategy = String::from("Degree of freedom");
    let mut rounds = 0;
    let mut guesses = 0;

    if verbosity > 3 {
        println!("Strategy: {}", strategy);
        println!("Press enter to start...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }

    while !grid.solved() {
        let mut gvecs = Vec::new();
        for r in 0..grid.puzzle.size {
            if !grid.row_solved(r) {
                gvecs.push(GridVector { loc: r, is_row: true, cargo: grid.dof(r, true) as i32 });
            }
        }
        for c in 0..grid.puzzle.size {
            if !grid.col_solved(c) {
                gvecs.push(GridVector { loc: c, is_row: false, cargo: grid.dof(c, false) as i32 });
            }
        }
        gvecs.sort_by(|a, b| a.cargo.cmp(&b.cargo));

        for gvec in gvecs {
            match gvec.is_row {
                true if grid.row_solved(gvec.loc) => continue,
                false if grid.col_solved(gvec.loc) => continue,
                _ => (),
            }
            grid.solve(gvec.loc, gvec.is_row);
            guesses += 1;
        }

        rounds += 1;
        if verbosity > 3 {
            println!("Round {}", rounds);
            grid.visualize();
            println!("Press enter to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }

    if verbosity > 0 {
        println!("{} strategy: {} rounds, {} guesses", strategy, rounds, guesses);
    }

    SolveResult { grid, strategy, rounds, guesses }
}
fn strategy_info_gain(puzzle: Puzzle, verbosity: u8) -> SolveResult {
    let mut grid = PuzzleGrid::new(puzzle);
    let strategy = String::from("Information gain");
    let mut rounds = 0;
    let mut guesses = 0;

    if verbosity > 3 {
        println!("Strategy: {}", strategy);
        println!("Press enter to start...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }

    while !grid.solved() {
        let mut gvecs = Vec::new();
        for r in 0..grid.puzzle.size {
            if !grid.row_solved(r) {
                gvecs.push(GridVector { loc: r, is_row: true, cargo: grid.info_gain(r, true) as i32 });
            }
        }
        for c in 0..grid.puzzle.size {
            if !grid.col_solved(c) {
                gvecs.push(GridVector { loc: c, is_row: false, cargo: grid.info_gain(c, false) as i32 });
            }
        }
        gvecs.sort_by(|a, b| b.cargo.cmp(&a.cargo));

        for gvec in gvecs {
            match gvec.is_row {
                true if grid.row_solved(gvec.loc) => continue,
                false if grid.col_solved(gvec.loc) => continue,
                _ => (),
            }
            grid.solve(gvec.loc, gvec.is_row);
            guesses += 1;
        }

        rounds += 1;
        if verbosity > 3 {
            println!("Round {}", rounds);
            grid.visualize();
            println!("Press enter to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }

    if verbosity > 0 {
        println!("{} strategy: {} rounds, {} guesses", strategy, rounds, guesses);
    }

    SolveResult { grid, strategy, rounds, guesses }
}

pub fn solve(puzzle: Puzzle, verbosity: u8) {
    let best = strategy_simple(puzzle.clone(), verbosity);
    let best = best.min(strategy_outskirt(puzzle.clone(), verbosity));
    let best = best.min(strategy_free_width(puzzle.clone(), verbosity));
    let best = best.min(strategy_info_gain(puzzle.clone(), verbosity));
    let best = best.min(strategy_dof(puzzle.clone(), verbosity));
    if verbosity > 0 { println!(); }

    println!("Best strategy: {} [{} rounds, {} guesses]", best.strategy, best.rounds, best.guesses);
    println!("Solution: ");
    best.grid.visualize();
}