use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Index(usize, usize); // column idx, row idx

struct Game {
    galaxies: Vec<Index>,
    // the galaxies count in a column
    cols: Vec<usize>,
    // the galaxies count in a row
    rows: Vec<usize>,
}

impl Game {
    fn new() -> Self {
        Self {
            galaxies: Vec::new(),
            cols: Vec::new(),
            rows: Vec::new(),
        }
    }

    fn read_line(&mut self, line: &str) {
        let line = line.trim().as_bytes();
        if self.cols.len() == 0 {
            self.cols = [0].repeat(line.len());
        } else {
            assert!(line.len() == self.cols.len());
        }
        let r = self.rows.len();
        self.rows.push(0);
        for c in 0..line.len() {
            if line[c] == b'#' {
                self.galaxies.push(Index(r, c));
                self.cols[c] += 1;
                self.rows[r] += 1;
            }
        }
    }

    fn part(&self, expand: usize) -> usize {
        let mut row_row_distance = Vec::with_capacity(self.rows.len());
        let mut row = [0_usize].repeat(self.rows.len());
        if self.rows[0] == 0 {
            row[0] = expand
        } else {
            row[0] = 1
        }
        for i in 1..self.rows.len() {
            row[i] = if self.rows[i] == 0 {
                row[i - 1] + expand
            } else {
                row[i - 1] + 1
            };
        }
        row_row_distance.push(row);
        for i in 1..self.rows.len() {
            let delta = if self.rows[i-1] == 0 { expand } else { 1 };
            let mut row: Vec<usize> = row_row_distance
                .last()
                .unwrap()
                .iter()
                .map(|v| (*v).saturating_sub(delta))
                .collect();
            for j in 0..i {
                row[j] = row_row_distance[j][i];
            }
            row_row_distance.push(row);
        }

        let mut col_col_distance = Vec::with_capacity(self.cols.len());
        let mut col = [0_usize].repeat(self.cols.len());
        if self.cols[0] == 0 {
            col[0] = expand
        } else {
            col[0] = 1
        }
        for i in 1..self.cols.len() {
            col[i] = if self.cols[i] == 0 {
                col[i-1] + expand
            } else {
                col[i-1] + 1
            };
        }
        col_col_distance.push(col); 
        for i in 1..self.cols.len() {
            let delta = if self.cols[i - 1] == 0 { expand } else { 1 };
            let mut col: Vec<usize> = col_col_distance.last().unwrap().iter().map(|v| (*v).saturating_sub(delta)).collect();
            for j in 0..i {
                col[j] = col_col_distance[j][i];
            }
            col_col_distance.push(col);
        }

        let mut sum = 0;

        for g_i in 0..self.galaxies.len() {
            let g1 = self.galaxies.get(g_i).unwrap();
            for g_j in g_i+1..self.galaxies.len() {
                let g2 = self.galaxies.get(g_j).unwrap();
                let distance = row_row_distance[g1.0][g2.0] + col_col_distance[g1.1][g2.1] - 2;
                sum += distance;
            }
        }
        sum
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut game = Game::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        game.read_line(&line);
        line.clear();
    }

    println!("Part1 {}", game.part(2));
    println!("Part1 {}", game.part(1000000));
}
