use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grid {
    Full,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Graph {
    connections: Vec<HashSet<usize>>,
    rev_connections: Vec<HashSet<usize>>,
}

struct Map {
    ids: HashMap<(usize, usize), usize>,
    grids: Vec<Vec<Option<Grid>>>,
    size: (usize, usize),
}

struct MST {
    distances: HashMap<usize, usize>,
    parent_map: HashMap<usize, usize>,
    children_map: HashMap<usize, HashSet<usize>>,
}

impl Map {
    fn from(input: &str) -> Self {
        let mut ids = HashMap::new();
        let mut grids = Vec::new();
        let mut size = (0, 0);
        for (line, row) in input.trim().split_whitespace().into_iter().zip(0_usize..) {
            let mut line_grids = Vec::new();
            for (c, col) in line.chars().zip(0_usize..) {
                let grid = match c {
                    '.' => Some(Grid::Full),
                    '^' => Some(Grid::Up),
                    '>' => Some(Grid::Right),
                    'v' => Some(Grid::Down),
                    '<' => Some(Grid::Left),
                    '#' => None,
                    _ => unreachable!(),
                };
                if grid.is_some() {
                    ids.insert((row, col), ids.len());
                }
                line_grids.push(grid);
            }
            if size.1 == 0 {
                size.1 = line_grids.len();
            } else {
                assert!(size.1 == line_grids.len());
            }
            grids.push(line_grids);
            size.0 += 1;
        }
        Self { ids, grids, size }
    }

    fn build_graph(&self) -> Graph {
        // build connections
        let mut connections: Vec<HashSet<usize>> = Vec::new();
        let mut rev_connections: Vec<HashSet<usize>> = Vec::new();
        for _ in 0..self.ids.len() {
            connections.push(HashSet::with_capacity(4));
            rev_connections.push(HashSet::with_capacity(4));
        }
        for (&(row, col), &id) in self.ids.iter() {
            let grid = self.grids[row][col].unwrap();
            if row > 0 && (grid == Grid::Full || grid == Grid::Up ){
                let nb = (row - 1, col);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id);
                    rev_connections[nb_id].insert(id);
                }
            }
            if col > 0 && (grid == Grid::Full || grid == Grid::Left) {
                let nb = (row, col - 1);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id);
                    rev_connections[nb_id].insert(id);
                }
            }
            if row + 1 < self.size.0 && (grid == Grid::Full || grid == Grid::Down) {
                let nb = (row + 1, col);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id);
                    rev_connections[nb_id].insert(id);
                }
            }
            if col + 1 < self.size.1 && (grid == Grid::Full || grid == Grid::Right) {
                let nb = (row, col + 1);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id);
                    rev_connections[nb_id].insert(id);
                }
            }
        }
        Graph { connections, rev_connections }
    }
}


impl Graph {

    fn build_mst(&self) -> MST {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut parent_map: HashMap<usize, usize> = HashMap::new();
        distances.insert(0, 0);
        let mut bfs = VecDeque::new();
        bfs.push_back((0_usize, 0_usize));
        while let Some((id, distance)) = bfs.pop_front() {
            if *distances.get(&id).unwrap() < distance {
                continue;
            }
            let next_distance = distance + 1;
            for &next_id in self.connections[id].iter() {
                if !distances.contains_key(&next_id) || *distances.get(&next_id).unwrap() > next_distance {
                    distances.insert(next_id, next_distance);
                    parent_map.insert(next_id, id);
                    bfs.push_back((next_id, next_distance));
                }
            }
        }
        let mut children_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for id in distances.keys() {
            if !children_map.contains_key(id) {
                children_map.insert(*id, HashSet::new());
            }
            if let Some(parent) = parent_map.get(id) {
                if !children_map.contains_key(parent) {
                    children_map.insert(*parent, HashSet::new());
                }
                children_map.get_mut(parent).unwrap().insert(*id);
            }
        }

        MST {
            distances,
            parent_map,
            children_map,
        }
    }
}


fn find_longest_path(graph: &Graph) -> usize {
    let mut mst = graph.build_mst();

    let mut changed = true;
    while changed {
        changed = false;
        let mut distances: Vec<(usize, usize)> = mst.distances.clone().into_iter().collect();
        distances.sort_by(|a, b| b.1.cmp(&a.1));
        for &(id, distance) in distances.iter() {
            if distance == 0 {
                break;
            }
            let sub_ids = mst.get_sub_ids(&id);
            let mut max_parent = mst.parent_map[&id];
            let mut max_distance = distance;
            for &new_parent in graph.rev_connections[id].iter() {
                if sub_ids.contains(&new_parent) || !mst.distances.contains_key(&new_parent) {
                    continue;
                };
                let new_distance = mst.distances[&new_parent] + 1;
                if new_distance > max_distance {
                    max_parent = new_parent;
                    max_distance = new_distance;
                }
            }
            if max_distance > distance {
                changed = true;
                let pre_parent = mst.parent_map[&id];
                mst.children_map.get_mut(&pre_parent).unwrap().remove(&id);
                mst.distances.insert(id, max_distance);
                mst.parent_map.insert(id, max_parent);
                mst.children_map.get_mut(&max_parent).unwrap().insert(id);
                mst.update_sub_distances(&id);
            }
        }
    }
    let last = graph.connections.len()-1;
    mst.distances[&last]
}

impl MST {
    fn update_sub_distances(&mut self, node_id: &usize) {
        let mut bfs = VecDeque::new();
        bfs.push_back(*node_id);
        while let Some(id) = bfs.pop_front() {
            let next_distance = self.distances[&id] + 1;
            for &sub_id in self.children_map.get(&id).unwrap().iter() {
                self.distances.insert(sub_id, next_distance);
                bfs.push_back(sub_id);
            }
        }
    }

    fn get_sub_ids(&self, node_id: &usize) -> HashSet<usize> {
        let mut sub_ids = Vec::new();
        sub_ids.push(*node_id);
        let mut i = 0;
        while i < sub_ids.len() {
            let id = sub_ids[i];
            for &sub_id in self.children_map.get(&id).unwrap().iter() {
                sub_ids.push(sub_id);
            }
            i += 1;
        }
        HashSet::from_iter(sub_ids)
    }
}


fn main() {
    let mut f = File::open("./input").expect("Failed to open input file.");
    let mut input = String::new();
    let _ = f.read_to_string(&mut input).unwrap();
    let map = Map::from(&input);
    let graph = map.build_graph();
    println!("{}", graph.connections.len());
    let part1 = find_longest_path(&graph);
    println!("Part1: {}", part1);
}
