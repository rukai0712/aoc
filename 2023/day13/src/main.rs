use std::{fs::File, io::{BufReader, BufRead}};


struct Graph {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Graph {

    fn new() -> Self {
        Self { rows: Vec::new(), cols: Vec::new()}
    }

    fn add_line(&mut self, line: &str) {
        let line = line.as_bytes();
        if self.cols.len() == 0 {
            self.cols = [0].repeat(line.len());
        }
        let mut new_row = 0;
        for i in 0..line.len() {
            let bit = match line[i] {
                b'#' => 1,
                b'.' => 0,
                _ => panic!("Invalid character"),
            };
            new_row = new_row * 2 + bit;
            self.cols[i] = self.cols[i] * 2 + bit;
        }
        self.rows.push(new_row);
    }

    fn find_mirror(&self) -> (Option<usize>, Option<usize>) {
        let mut result = (None, None);
        for mirror in 1..self.rows.len() {
            let compare_len = mirror.min(self.rows.len() - mirror);
            let mut is_mirror = true;
            for i in 0..compare_len {
                if self.rows[mirror - i - 1] != self.rows[mirror + i] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                result.0.replace(mirror);
                break;
            }
        }

        for mirror in 1..self.cols.len() {
            let compare_len = mirror.min(self.cols.len() - mirror);
            let mut is_mirror = true;
            for i in 0..compare_len {
                if self.cols[mirror - i - 1] != self.cols[mirror + i] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                result.1.replace(mirror);
                break;
            }
        }

        
        return result;
    }
    
}


fn main() {
    let f = File::open("./input").expect("Failed to read input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut graphs = Vec::<Graph>::new();
    let mut graph = Graph::new();
    while let Ok(size) = reader.read_line(&mut line) {
        let input = line.trim();
        if size == 0 || input.len() == 0 {
            assert!(graph.rows.len() > 0 && graph.cols.len() > 0);
            graphs.push(graph);
            graph = Graph::new();
            line.clear();
            if size == 0 {
                break;
            }
            continue;
        }
        graph.add_line(input);
        line.clear();
    }
    assert!(graph.rows.len() == 0 && graph.cols.len() == 0);

    let mut part1 = 0;
    for graph in graphs.iter() {
        let (row_idx, col_idx) = graph.find_mirror();
        if let Some(row_idx) = row_idx {
            assert!(col_idx.is_none());
            part1 += row_idx * 100;
        } else if let Some(col_idx) = col_idx {
            part1 += col_idx;
        } else {
            println!("Not find mirror");
        }
    }

    println!("Part1: {}", part1);
}
