use std::{
    collections::{VecDeque, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    grids: Vec<Vec<bool>>,
    start: (usize, usize),
    size: (usize, usize),
}

#[derive(Debug, Default)]
struct MapBuilder {
    grids: Vec<Vec<bool>>,
    start: Option<(usize, usize)>,
    size: (usize, usize),
}

impl MapBuilder {
    fn add_line(&mut self, line: &str) {
        let line = line.trim().as_bytes();
        if self.size.1 == 0 {
            self.size.1 = line.len();
        } else {
            assert!(self.size.1 == line.len());
        }
        let mut row = Vec::with_capacity(line.len());
        for &c in line {
            let g = match c {
                b'.' => true,
                b'#' => false,
                b'S' => {
                    assert!(self.start.replace((self.size.0, row.len())).is_none());
                    true
                }
                _ => unreachable!(),
            };
            row.push(g);
        }
        self.grids.push(row);
        self.size.0 += 1;
    }

    fn build(self) -> Map {
        Map {
            grids: self.grids,
            start: self.start.unwrap(),
            size: self.size,
        }
    }
}

fn bfs(map: &Map, steps: usize) -> usize {
    let mut points = HashSet::new();
    points.insert(map.start.clone());
    for _ in 0..steps {
        let mut next_points = HashSet::new();
        for point in points.iter() {
            next_points.extend(map.get_neighbours(point));
        }
        points = next_points;
    }
    points.len()
}

impl Map {
    fn get_neighbours(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        if point.0 > 0 {
            let next = (point.0 - 1, point.1);
            if self.grids[next.0][next.1] {
                neighbours.push(next);
            }
        }
        if point.0 + 1 < self.size.0 {
            let next = (point.0 + 1, point.1);
            if self.grids[next.0][next.1] {
                neighbours.push(next);
            }
        }
        if point.1 > 0 {
            let next = (point.0, point.1 - 1);
            if self.grids[next.0][next.1] {
                neighbours.push(next);
            }
        }
        if point.1 + 1 < self.size.1 {
            let next = (point.0, point.1 + 1);
            if self.grids[next.0][next.1] {
                neighbours.push(next);
            }
        }
        neighbours
    }
}

fn main() {
    let f = File::open("./input").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut builder = MapBuilder::default();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        builder.add_line(&line);
        line.clear();
    }
    let map = builder.build();

    let part1 = bfs(&map, 64);
    println!("Part1 {}", part1);
}
