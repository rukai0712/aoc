use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

struct Map {
    grids: Vec<Vec<u32>>,
    size: (usize, usize),
}

impl Map {
    fn new() -> Self {
        Self {
            grids: Vec::new(),
            size: (0, 0),
        }
    }
}

struct GridStatue {
    row: usize,
    col: usize,
    direction: Direction,
    steps: usize,
    distance: u32,
}

struct DistanceMap<'a> {
    distances: &'a Vec<Vec<u32>>,
    grids: Vec<Vec<HashMap<Direction, [Option<u32>; 10]>>>,
    size: (usize, usize),
    min_steps: usize,
    max_steps: usize,
}

impl<'a> DistanceMap<'a> {
    fn new(map: &'a Map, min_steps: usize, max_steps: usize) -> Self {
        let mut grids = Vec::with_capacity(map.size.0);
        for _ in 0..map.size.0 {
            let mut line = Vec::with_capacity(map.size.1);
            for _ in 0..map.size.1 {
                line.push(HashMap::new())
            }
            grids.push(line)
        }
        Self {
            distances: &map.grids,
            grids,
            size: map.size.clone(),
            min_steps,
            max_steps,
        }
    }

    fn goto(&mut self, row: usize, col: usize, pre: &GridStatue) -> Option<GridStatue> {
        let (direction, steps, delta) = if row == pre.row {
            if col < pre.col {
                let delta: u32 = self.distances[row][col..pre.col]
                    .iter()
                    .fold(0, |crr, v| crr + *v);
                match pre.direction {
                    Direction::Left => Some((Direction::Left, pre.steps + pre.col - col, delta)),
                    Direction::Right => None,
                    _ => Some((Direction::Left, pre.col - col, delta)),
                }
            } else {
                let delta = self.distances[row][pre.col + 1..col + 1]
                    .iter()
                    .fold(0, |crr, v| crr + *v);
                match pre.direction {
                    Direction::Right => Some((Direction::Right, pre.steps + col - pre.col, delta)),
                    Direction::Left => None,
                    _ => Some((Direction::Right, col - pre.col, delta)),
                }
            }
        } else if col == pre.col {
            if row < pre.row {
                let delta: u32 = self.distances[row..pre.row]
                    .iter()
                    .fold(0, |crr, l| crr + l[col]);
                match pre.direction {
                    Direction::Top => Some((Direction::Top, pre.steps + pre.row - row, delta)),
                    Direction::Bottom => None,
                    _ => Some((Direction::Top, pre.row - row, delta)),
                }
            } else {
                let delta: u32 = self.distances[pre.row + 1..row + 1]
                    .iter()
                    .fold(0, |crr, l| crr + l[col]);
                match pre.direction {
                    Direction::Bottom => {
                        Some((Direction::Bottom, pre.steps + row - pre.row, delta))
                    }
                    Direction::Top => None,
                    _ => Some((Direction::Bottom, row - pre.row, delta)),
                }
            }
        } else {
            unreachable!()
        }?;
        if steps > self.max_steps || steps < self.min_steps {
            return None;
        }
        let grid_status = &mut self.grids[row][col];
        if !grid_status.contains_key(&direction) {
            grid_status.insert(direction, [None; 10]);
        }
        let distance = pre.distance + delta;
        let distances = grid_status.get_mut(&direction).unwrap();
        if distances[steps - 1].map_or(true, |v| v > distance) {
            distances[steps - 1].replace(distance);
        } else {
            return None;
        }
        Some(GridStatue {
            row,
            col,
            direction,
            steps,
            distance,
        })
    }

    fn get_grid_distance_min(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
        steps: usize,
    ) -> Option<u32> {
        let grid_status = &self.grids[row][col];
        grid_status.get(&direction)?[self.min_steps - 1..steps.min(self.max_steps)]
            .iter()
            .filter_map(|v| *v)
            .min()
    }
}

struct Game {
    map: Map,
}

impl Game {
    fn new(text: &str) -> Self {
        let mut map = Map::new();
        for line in text.trim().split_ascii_whitespace() {
            map.grids.push(
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect(),
            );
        }
        map.size.0 = map.grids.len();
        map.size.1 = map.grids[0].len();
        Self { map }
    }

    fn part1(&self) -> u32 {
        let mut distance_map = DistanceMap::new(&self.map, 1, 3);
        let mut bfs: VecDeque<GridStatue> = VecDeque::new();
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Top,
            steps: 1,
            distance: 0,
        };
        bfs.push_back(init_status);
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Left,
            steps: 1,
            distance: 0,
        };
        bfs.push_back(init_status);

        while let Some(status) = bfs.pop_front() {
            if let Some(min_distance) = distance_map.get_grid_distance_min(
                status.row,
                status.col,
                status.direction,
                status.steps,
            ) {
                if min_distance < status.distance {
                    continue;
                }
            }
            if status.row > 0 {
                if let Some(new_status) = distance_map.goto(status.row - 1, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.row + 1 < distance_map.size.0 {
                if let Some(new_status) = distance_map.goto(status.row + 1, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col > 0 {
                if let Some(new_status) = distance_map.goto(status.row, status.col - 1, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col + 1 < distance_map.size.1 {
                if let Some(new_status) = distance_map.goto(status.row, status.col + 1, &status) {
                    bfs.push_back(new_status);
                }
            }
        }
        let row = distance_map.size.0 - 1;
        let col = distance_map.size.1 - 1;
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|direction| distance_map.get_grid_distance_min(row, col, *direction, 3))
        .min()
        .unwrap()
    }

    fn part2(&self) -> u32 {
        let mut distance_map = DistanceMap::new(&self.map, 4, 10);
        let mut bfs: VecDeque<GridStatue> = VecDeque::new();
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Top,
            steps: 4,
            distance: 0,
        };
        bfs.push_back(init_status);
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Left,
            steps: 4,
            distance: 0,
        };
        bfs.push_back(init_status);

        while let Some(status) = bfs.pop_front() {
            if let Some(min_distance) = distance_map.get_grid_distance_min(
                status.row,
                status.col,
                status.direction,
                status.steps,
            ) {
                if min_distance < status.distance {
                    continue;
                }
            }

            if status.row > 0 && status.direction == Direction::Top {
                if let Some(new_status) = distance_map.goto(status.row - 1, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.row > 3 && status.direction != Direction::Top {
                if let Some(new_status) = distance_map.goto(status.row - 4, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.row + 1 < distance_map.size.0 && status.direction == Direction::Bottom {
                if let Some(new_status) = distance_map.goto(status.row + 1, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.row + 4 < distance_map.size.0 && status.direction != Direction::Bottom {
                if let Some(new_status) = distance_map.goto(status.row + 4, status.col, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col > 0 && status.direction == Direction::Left {
                if let Some(new_status) = distance_map.goto(status.row, status.col - 1, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col > 3 && status.direction != Direction::Left {
                if let Some(new_status) = distance_map.goto(status.row, status.col - 4, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col + 1 < distance_map.size.1 && status.direction == Direction::Right {
                if let Some(new_status) = distance_map.goto(status.row, status.col + 1, &status) {
                    bfs.push_back(new_status);
                }
            }
            if status.col + 4 < distance_map.size.1 && status.direction != Direction::Right {
                if let Some(new_status) = distance_map.goto(status.row, status.col + 4, &status) {
                    bfs.push_back(new_status);
                }
            }
        }

        let row = distance_map.size.0 - 1;
        let col = distance_map.size.1 - 1;
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|direction| distance_map.get_grid_distance_min(row, col, *direction, 10))
        .min()
        .unwrap()
    }
}

fn main() {
    let mut f = File::open("./input").expect("Failed to open input file");
    let mut text = String::new();
    let _ = f.read_to_string(&mut text).expect("Failed to read input");
    let game = Game::new(&text);
    let part1 = game.part1();
    println!("Part1: {}", part1);
    let part2 = game.part2();
    println!("Part2: {}", part2);
}
