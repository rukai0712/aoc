use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round(usize),
    Cube(usize),
}

struct Game {
    cols: Vec<Vec<Rock>>,
    size: (usize, usize),  // number of rows, number of cols
}

impl Game {
    
    fn new() -> Self {
        Self { cols: Vec::new(), size: (0, 0) }
    }

    fn add_row(&mut self, line: &str) {
        let line = line.as_bytes();
        if self.cols.len() == 0 {
            for _ in 0..line.len() {
                self.cols.push(Vec::new());
            }
            self.size.1 = line.len();
        }
        let row_idx = self.size.0;
        for i in 0..line.len() {
            match line[i] {
                b'O' => self.cols[i].push(Rock::Round(row_idx)),
                b'#' => self.cols[i].push(Rock::Cube(row_idx)),
                b'.' => {},
                _ => unreachable!(),
            }
        }
        self.size.0 += 1;
    }

    fn calculate_load(&self) -> usize {
        let mut load = 0;
        for col in self.cols.iter() {
            let mut score = self.size.0;
            for r in col.iter() {
                match *r {
                    Rock::Cube(row_idx) => {
                        score = self.size.0 - row_idx - 1;
                    },
                    Rock::Round(_) => {
                        load += score;
                        score -= 1;
                    }
                }
            }
        }
        load
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
        game.add_row(line.trim());
        line.clear();
    }
    let part1 = game.calculate_load();
    println!("Part1: {}", part1);

}
