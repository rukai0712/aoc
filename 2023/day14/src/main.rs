use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round(usize),
    Cube(usize),
}

struct Game {
    circle: usize,
    cols: Vec<Vec<Rock>>,
    size: (usize, usize),  // number of rows, number of cols
}

impl Game {
    
    fn new() -> Self {
        Self { circle: 0, cols: Vec::new(), size: (0, 0) }
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

    fn calculate_part1(&self) -> usize {
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

    fn calculate_load(&self) -> usize {
        let mut load = 0;
        for col in self.cols.iter() {
            for r in col.iter() {
                match *r {
                    Rock::Round(row_idx) => {
                        load += self.size.0 - row_idx;
                    }
                    Rock::Cube(_) => {}
                }
            }
        }
        load
    }

    fn rotate_90(&self) -> Self {
        let mut rotated = Self {
            circle: self.circle,
            cols: Vec::with_capacity(self.size.0),
            size: (self.size.1, self.size.0),
        };
        for _ in 0..rotated.size.1 {
            rotated.cols.push(Vec::new());
        }
        for i in 0..self.cols.len() {
            let mut new_i: usize = self.size.0;
            for r in self.cols[i].iter() {
                match *r {
                    Rock::Cube(row_idx) => {
                        new_i = self.size.0 - row_idx - 1;
                        rotated.cols[new_i].push(Rock::Cube(i));
                    },
                    Rock::Round(_) => {
                        new_i -= 1;
                        rotated.cols[new_i].push(Rock::Round(i));
                    }
                }
            }
        }
        rotated
    } 

    fn rotate_360(&self) -> Self {
        let mut game = self.rotate_90().rotate_90().rotate_90().rotate_90();
        game.circle += 1;
        game
    }

}


impl Display for Game {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = Vec::new();
        for _ in 0..self.size.0 {
            lines.push(String::new());
        }

        for col in self.cols.iter() {
            let mut i = 0;
            for r in col.iter() {
                match *r {
                    Rock::Round(idx) => {
                        while i < idx {
                            lines[i].push('.');
                            i += 1;
                        }
                        lines[i].push('O');
                        i += 1;
                    },
                    Rock::Cube(idx) => {
                        while i < idx {
                            lines[i].push('.');
                            i += 1;
                        }
                        lines[i].push('#');
                        i += 1;
                    }
                }
            }
            while i < self.size.1 {
                lines[i].push('.');
                i += 1;
            }
        }

        writeln!(f, "{}", lines.join("\n"))
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
    println!("Part1: {}", game.calculate_part1());

    let mut history = HashMap::<usize, Vec<Game>>::new();
    let mut next_game = None;
    let load: usize = game.calculate_load();
    next_game.replace(game.rotate_360());   
    history.insert(load, vec![game]);

    let end_loops = 1000000000;
    while next_game.as_ref().unwrap().circle < end_loops {
        let mut game = next_game.take().unwrap();
        let load = game.calculate_load();
        let mut repeats = 0;
        if history.contains_key(&load) {
            for pre_game in history.get(&load).unwrap().iter() {
                if game.cols == pre_game.cols {
                    repeats = game.circle - pre_game.circle;
                }
            }
        }
        if repeats > 0 {
            game.circle += repeats * ((end_loops - game.circle) / repeats);
            next_game.replace(game);
            break;
        }
        next_game.replace(game.rotate_360());
        if !history.contains_key(&load) {
            history.insert(load, vec![game]);
        } else {
            history.get_mut(&load).unwrap().push(game);
        }        
    }

    let mut game = next_game.take().unwrap();
    while game.circle < end_loops {
        game = game.rotate_360();
    }
    println!("Part2 {}", game.calculate_load());

}
