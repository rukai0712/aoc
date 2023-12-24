use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
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
    connections: HashMap<usize, HashMap<usize, usize>>,
    rev_connections: HashMap<usize, HashMap<usize, usize>>,
    start_id: usize,
    end_id: usize,
}

struct Map {
    ids: HashMap<(usize, usize), usize>,
    grids: Vec<Vec<Option<Grid>>>,
    size: (usize, usize),
}

struct MST {
    distances: HashMap<usize, usize>,
    path_ids: HashSet<usize>,
    end_id: usize,
    parent_map: HashMap<usize, usize>,
    children_map: HashMap<usize, HashSet<usize>>,
}

impl Map {
    fn from(input: &str, with_slope: bool) -> Self {
        let mut ids = HashMap::new();
        let mut grids = Vec::new();
        let mut size = (0, 0);
        for (line, row) in input.trim().split_whitespace().into_iter().zip(0_usize..) {
            let mut line_grids = Vec::new();
            for (c, col) in line.chars().zip(0_usize..) {
                let grid = if with_slope {
                    match c {
                        '.' => Some(Grid::Full),
                        '^' => Some(Grid::Up),
                        '>' => Some(Grid::Right),
                        'v' => Some(Grid::Down),
                        '<' => Some(Grid::Left),
                        '#' => None,
                        _ => unreachable!(),
                    }
                } else {
                    match c {
                        '.' | '^' | '>' | 'v' | '<' => Some(Grid::Full),
                        '#' => None,
                        _ => unreachable!(),
                    }
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
        let mut connections: Vec<HashMap<usize, usize>> = Vec::new();
        let mut rev_connections: Vec<HashMap<usize, usize>> = Vec::new();
        for _ in 0..self.ids.len() {
            connections.push(HashMap::with_capacity(4));
            rev_connections.push(HashMap::with_capacity(4));
        }
        for (&(row, col), &id) in self.ids.iter() {
            let grid = self.grids[row][col].unwrap();
            if row > 0 && (grid == Grid::Full || grid == Grid::Up) {
                let nb = (row - 1, col);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id, 1);
                    rev_connections[nb_id].insert(id, 1);
                }
            }
            if col > 0 && (grid == Grid::Full || grid == Grid::Left) {
                let nb = (row, col - 1);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id, 1);
                    rev_connections[nb_id].insert(id, 1);
                }
            }
            if row + 1 < self.size.0 && (grid == Grid::Full || grid == Grid::Down) {
                let nb = (row + 1, col);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id, 1);
                    rev_connections[nb_id].insert(id, 1);
                }
            }
            if col + 1 < self.size.1 && (grid == Grid::Full || grid == Grid::Right) {
                let nb = (row, col + 1);
                if self.grids[nb.0][nb.1].is_some() {
                    let nb_id = self.ids.get(&nb).unwrap().clone();
                    connections[id].insert(nb_id, 1);
                    rev_connections[nb_id].insert(id, 1);
                }
            }
        }
        let start_id = 0;
        let end_id = connections.len() - 1;

        connections.last_mut().unwrap().clear(); // end not allowed as node
        rev_connections.first_mut().unwrap().clear(); // start not allowed as node
        for i in 0..connections.len() {
            connections[i].remove(&start_id);
            rev_connections[i].remove(&end_id);
        }

        let mut remove_nodes = true;
        while remove_nodes {
            remove_nodes = false;
            for i in 1..connections.len()-1 {
                if connections[i].len() == 1 && rev_connections[i].len() == 1 {
                    let (&next, &next_len) = connections[i].iter().next().unwrap();
                    let (&pre, &pre_len) = rev_connections[i].iter().next().unwrap();
                    assert!(connections[pre].remove(&i).is_some());
                    assert!(rev_connections[next].remove(&i).is_some());
                    connections[pre].insert(next, next_len + pre_len);
                    rev_connections[next].insert(pre, next_len + pre_len);
                    connections[i].clear();
                    rev_connections[i].clear();
                    remove_nodes = true;
                }
            }
        }

        let mut graph_connections = HashMap::new();
        let mut graph_rev_connections = HashMap::new();
        for i in 0..connections.len() {
            if connections[i].len() > 0 {
                graph_connections.insert(i, connections[i].clone());
            }
            if rev_connections[i].len() > 0 {
                graph_rev_connections.insert(i, rev_connections[i].clone());
            }
        }

        Graph {
            connections: graph_connections,
            rev_connections: graph_rev_connections,
            start_id: 0,
            end_id: connections.len()-1,
        }
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
            if !self.connections.contains_key(&id) || *distances.get(&id).unwrap() < distance {
                continue;
            }
            for (&next_id, &steps) in self.connections[&id].iter() {
                let next_distance: usize = distance + steps;
                if !distances.contains_key(&next_id)
                    || *distances.get(&next_id).unwrap() > next_distance
                {
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

        MST::new(
            distances,
            parent_map,
            children_map,
            self.end_id,
        )
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
            if !graph.rev_connections.contains_key(&id) {
                continue;
            }
            let sub_ids = mst.get_sub_ids(&id);
            let mut max_parent = mst.parent_map[&id];
            let mut max_distance = distance;
            for (&new_parent, &steps) in graph.rev_connections[&id].iter() {
                if sub_ids.contains(&new_parent) || !mst.distances.contains_key(&new_parent) {
                    continue;
                };
                let new_distance = mst.distances[&new_parent] + steps;
                if new_distance > max_distance {
                    max_parent = new_parent;
                    max_distance = new_distance;
                }
            }
            if max_distance > distance {
                changed = true;
                mst.change_parent(&id, &max_parent);
            }
        }
        if !changed {
            println!("{}", mst.distances[&graph.end_id]);
            for &(id, distance) in distances.iter() {
                if distance == 0 {
                    break;
                }
                if mst.path_ids.contains(&id) || !graph.rev_connections.contains_key(&id) {
                    continue;
                }
                if let Some((path_id, delta)) = mst.get_to_path(&id, &graph) {
                    let sub_ids = mst.get_sub_ids(&path_id);
                    let mut max_parent = mst.parent_map[&id];
                    let mut max_distance = distance - delta;
                    for (&new_parent, &steps) in graph.rev_connections[&id].iter() {
                        if sub_ids.contains(&new_parent) || !mst.distances.contains_key(&new_parent) {
                            continue;
                        };
                        let new_distance = mst.distances[&new_parent] + steps + delta;
                        if new_distance > max_distance {
                            max_parent = new_parent;
                            max_distance = new_distance;
                        }
                    }
                    if max_distance > distance - delta {
                        changed = true;
                        mst.change_parent(&id, &max_parent);
                    }
                }
            }
        }
    }
    mst.distances[&graph.end_id]
}

impl MST {
    fn new(
        distances: HashMap<usize, usize>,
        parent_map: HashMap<usize, usize>,
        children_map: HashMap<usize, HashSet<usize>>,
        end_id: usize,
    ) -> Self {
        let mut instance = Self {
            distances,
            parent_map,
            children_map,
            end_id,
            path_ids: HashSet::new(),
        };
        instance.update_path();
        instance
    }

    fn update_path(&mut self) {
        self.path_ids.clear();
        self.path_ids.insert(self.end_id);
        let mut cur_id = self.end_id;
        while let Some(parent_id) = self.parent_map.get(&cur_id) {
            self.path_ids.insert(*parent_id);
            cur_id = *parent_id;
        }
    }

    fn change_parent(&mut self, node_id: &usize, new_parent: &usize) {
        let pre_parent = self.parent_map[node_id];
        self.children_map
            .get_mut(&pre_parent)
            .unwrap()
            .remove(node_id);
        self.parent_map.insert(*node_id, *new_parent);
        self.children_map
            .get_mut(new_parent)
            .unwrap()
            .insert(*node_id);
        let new_distance = self.distances[new_parent] + 1;
        self.distances.insert(*node_id, new_distance);
        if self.path_ids.contains(node_id) {
            // path changed
            self.update_path();
        }
        self.update_sub_distances(node_id);
    }

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

    fn get_to_path(&self, node_id: &usize, graph: &Graph) -> Option<(usize, usize)> {
        let mut delta = 0;
        let mut cur_id = *node_id;
        while !self.path_ids.contains(&cur_id) {
            let &parent_id = self.parent_map.get(&cur_id).unwrap();
            if !graph.connections.get(&parent_id).is_some_and(|hs| hs.contains_key(&cur_id)) {
                return None;
            }
            delta += graph.connections[&parent_id][&cur_id];
            cur_id = parent_id;
        }
        Some((cur_id, delta))
    }
}

fn main() {
    let mut f = File::open("./input").expect("Failed to open input file.");
    let mut input = String::new();
    let _ = f.read_to_string(&mut input).unwrap();
    let map = Map::from(&input, true);
    let graph = map.build_graph();
    let part1 = find_longest_path(&graph);
    println!("Part1: {}", part1);
    let map = Map::from(&input, false);
    let graph = map.build_graph();
    let part2 = find_longest_path(&graph);
    println!("Part2: {}", part2);
}
