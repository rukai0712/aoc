use std::{
    collections::{HashMap, HashSet},
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
    start_id: usize,
    end_id: usize,
}

struct Map {
    ids: HashMap<(usize, usize), usize>,
    grids: Vec<Vec<Option<Grid>>>,
    size: (usize, usize),
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
            for i in 0..connections.len() {
                if connections[i].len() == 2 && rev_connections[i] == connections[i] {
                    let mut f =  connections[i].iter();
                    let (&pre, &pre_len) = f.next().unwrap();
                    let (&next, &next_len) =f.next().unwrap();
                    if connections[pre].remove(&i).is_some() {
                        connections[pre].insert(next, next_len + pre_len);
                        assert!(rev_connections[next].remove(&i).is_some());
                        rev_connections[next].insert(pre, next_len + pre_len);
                        connections[i].remove(&next);
                        rev_connections[i].remove(&pre);
                        remove_nodes = true;
                    }
                    if connections[next].remove(&i).is_some() {
                        connections[next].insert(pre, next_len + pre_len);
                        assert!(rev_connections[pre].remove(&i).is_some());
                        rev_connections[pre].insert(next, next_len + pre_len);
                        connections[i].remove(&pre);
                        rev_connections[i].remove(&next);
                        remove_nodes =true
                    }
                }
            }
        }

        let mut graph_connections = HashMap::new();
        for i in 0..connections.len() {
            if connections[i].len() > 0 {
                graph_connections.insert(i, connections[i].clone());
            }
        }

        Graph {
            connections: graph_connections,
            start_id,
            end_id,
        }
    }
}


fn find_longest_path(graph: &Graph) -> usize {
    let mut path_ids = HashSet::new();
    path_ids.insert(graph.start_id);
    dfs(graph, &graph.start_id, &mut path_ids).unwrap()
}

fn dfs(graph: &Graph, id: &usize, path_ids: &mut HashSet<usize>) -> Option<usize> {
    if *id == graph.end_id {
        return Some(0);
    }
    if !graph.connections.contains_key(id) {
        return None;
    }

    let mut max_distance = None;

    for (next_id, &next_steps) in graph.connections[id].iter() {
        if path_ids.contains(next_id) {
            continue;
        }
        path_ids.insert(*next_id);
        if let Some(distance) = dfs(graph, next_id, path_ids) {
            if max_distance.is_none() || max_distance.unwrap() < distance + next_steps {
                max_distance.replace(distance + next_steps);
            }
        }
        path_ids.remove(next_id);
    }
    max_distance
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
