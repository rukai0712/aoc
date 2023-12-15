use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Cube,
}

struct Game {
    cols: Vec<Vec<Option<Rock>>>,
}

impl Game {
    
    fn new() -> Self {
        Self { cols: Vec::new() }
    }

    fn add_row(&mut self, line: &str) {
        let line = line.as_bytes();
        if self.cols.len() == 0 {
            for _ in 0..line.len() {
                self.cols.push(Vec::new());
            }
        }
        for i in 0..line.len() {
            self.cols[i].push(match line[i] {
                b'O' => Some(Rock::Round),
                b'#' => Some(Rock::Cube),
                b'.' => None,
                _ => panic!("Invalid character"),
            });
        }
    }

    fn calculate_part1(&self) -> usize {
        let mut load = 0;
        for col in self.cols.iter() {
            let mut score = col.len();
            for i in 0..col.len() {
                match col[i] {
                    Some(Rock::Round) => {
                        load += score;
                        score -= 1;
                    },
                    Some(Rock::Cube) => {
                        score = col.len() - i - 1;
                    },
                    None => {}
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
    let part1 = game.calculate_part1();
    println!("Part1: {}", part1);

}
