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
    grids: Vec<Vec<HashMap<Direction, [Option<u32>; 3]>>>,
    size: (usize, usize),
}

impl<'a> DistanceMap<'a> {
    fn new(map: &'a Map) -> Self {
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
        }
    }

    fn goto(&mut self, row: usize, col: usize, pre: &GridStatue) -> Option<GridStatue> {
        let distance = pre.distance + self.distances[row][col];
        let (direction, steps) = if row == pre.row {
            if col < pre.col {
                assert!(col + 1 == pre.col);
                match pre.direction {
                    Direction::Left => Some((Direction::Left, pre.steps + 1)),
                    Direction::Right => None,
                    _ => Some((Direction::Left, 1)),
                }
            } else {
                assert!(pre.col + 1 == col);
                match pre.direction {
                    Direction::Right => Some((Direction::Right, pre.steps + 1)),
                    Direction::Left => None,
                    _ => Some((Direction::Right, 1)),
                }
            }
        } else if col == pre.col {
            if row < pre.row {
                assert!(row + 1 == pre.row);
                match pre.direction {
                    Direction::Top => Some((Direction::Top, pre.steps + 1)),
                    Direction::Bottom => None,
                    _ => Some((Direction::Top, 1)),
                }
            } else {
                assert!(pre.row + 1 == row);
                match pre.direction {
                    Direction::Bottom => Some((Direction::Bottom, pre.steps + 1)),
                    Direction::Top => None,
                    _ => Some((Direction::Bottom, 1)),
                }
            }
        } else {
            unreachable!()
        }?;
        if steps > 3 {
            return None;
        }
        let grid_status = &mut self.grids[row][col];
        if !grid_status.contains_key(&direction) {
            grid_status.insert(direction, [None; 3]);
        }
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
        grid_status.get(&direction)?[0..steps]
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

    fn calculate_distance_to_end(&self) -> u32 {
        let mut distance_map = DistanceMap::new(&self.map);
        let mut bfs: VecDeque<GridStatue> = VecDeque::new();
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Top,
            steps: 1,
            distance: 0,
        };
        bfs.push_back(distance_map.goto(0, 1, &init_status).unwrap());
        let init_status = GridStatue {
            row: 0,
            col: 0,
            direction: Direction::Left,
            steps: 1,
            distance: 0,
        };
        bfs.push_back(distance_map.goto(1, 0, &init_status).unwrap());

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
}

fn main() {
    let mut f = File::open("./input").expect("Failed to open input file");
    let mut text = String::new();
    let _ = f.read_to_string(&mut text).expect("Failed to read input");
    let game = Game::new(&text);
    let distance = game.calculate_distance_to_end();
    println!("Part1: {}", distance);
}
