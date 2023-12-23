use std::{
    collections::HashSet,
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

    fn get_neighbours_i64(&self, point: &(i64, i64)) -> Vec<(i64, i64)> {
        let mut neighbours = Vec::with_capacity(4);
        let next = (point.0 - 1, point.1);
        if self.get_grids_i64(&next) {
            neighbours.push(next);
        }
        let next = (point.0 + 1, point.1);
        if self.get_grids_i64(&next) {
            neighbours.push(next);
        }
        let next = (point.0, point.1 - 1);
        if self.get_grids_i64(&next) {
            neighbours.push(next);
        }
        let next = (point.0, point.1 + 1);
        if self.get_grids_i64(&next) {
            neighbours.push(next);
        }
        neighbours
    }

    fn get_grids_i64(&self, point: &(i64, i64)) -> bool {
        let mut point = (point.0 % (self.size.0 as i64), point.1 % (self.size.1 as i64));
        if point.0 < 0 {
            point.0 += self.size.0 as i64;
        }
        if point.1 < 0 {
            point.1 += self.size.1 as i64;
        }
        assert!(point.0 >= 0 && point.1 >= 0);
        self.grids[point.0 as usize][point.1 as usize]
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

fn valid_grids(map: &Map) -> HashSet<(usize, usize)> {
    let mut points = HashSet::new();
    let mut valid_points = HashSet::new();
    valid_points.insert(map.start.clone());
    points.insert(map.start.clone());
    while points.len() != 0 {
        let mut next_points = HashSet::new();
        for point in points.iter() {
            for next_point in map.get_neighbours(point) {
                if valid_points.contains(&next_point) {
                    continue;
                }
                valid_points.insert(next_point.clone());
                next_points.insert(next_point);
            }
        }
        points = next_points;
    }
    valid_points
}


fn infinit_bfs(map: &Map, steps: usize) -> usize {
    let mut points = HashSet::new();
    points.insert((map.start.0 as i64, map.start.1 as i64));
    for _ in 0..steps {
        let mut next_points = HashSet::new();
        for point in points.iter() {
            next_points.extend(map.get_neighbours_i64(point));
        }
        points = next_points;
    }
    points.len()
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
    println!("Start {:?}", map.start); // (65, 65)
    println!("{:?}", map.size); // (131, 131)
    let steps = 26501365;
    let s_repeats = steps / map.size.0; // 202300
    let s_remains = steps % map.size.0; // 65
    
    // 34920, 96829, 189644, 313365 ...
    //    61909, 92815, 123721 ...
    //       30906, 30906 ...
    let mut values = Vec::new();    
    for i in 1..4 {
        let s = s_remains + map.size.0 * i;
        let count = infinit_bfs(&map, s);
        println!("{}: {}", i, count);
        values.push(count);
    }
    let delta1 = values[1] - values[0];
    let delta2 = values[2] - values[1];
    let delta_inc = delta2 - delta1;
    let part2 = values[0] + (delta1 + delta1 + delta_inc * (s_repeats - 2)) * (s_repeats - 1) / 2;
    println!("Part2 {}", part2);

}
