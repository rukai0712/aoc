use std::{
    collections::{HashMap, HashSet},
    fs::File, io::Read,
};

#[derive(Debug, Clone)]
struct Graph {
    ids: HashMap<String, usize>, // plugin names and node id mapping
    nodes: HashMap<usize, HashSet<String>>, // plugin names contains in that node
    connections: HashMap<usize, HashMap<usize, usize>>, // connection between nodes
}

impl Graph {
    fn from(lines: &str) -> Self {
        let mut ids: HashMap<String, usize> = HashMap::new();
        let mut nodes: HashMap<usize, HashSet<String>> = HashMap::new();
        let mut connections: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        for line in lines.split("\n") {
            let mut f = line.split(":");
            let name = f.next().unwrap().trim();
            if !ids.contains_key(name) {
                let id = ids.len();
                ids.insert(name.to_string(), id);
                nodes.insert(id, HashSet::from([name.to_string()]));
                connections.insert(id, HashMap::new());
            }
            let id = *ids.get(name).unwrap();

            for nb_name in f.next().unwrap().trim().split_whitespace() {
                let nb_name = nb_name.trim();
                if !ids.contains_key(nb_name) {
                    let nb_id = ids.len();
                    ids.insert(nb_name.to_string(), nb_id);
                    nodes.insert(nb_id, HashSet::from([nb_name.to_string()]));
                    connections.insert(nb_id, HashMap::new());
                }
                let nb_id = *ids.get(nb_name).unwrap();
                connections.get_mut(&id).unwrap().insert(nb_id, 1);
                connections.get_mut(&nb_id).unwrap().insert(id, 1);
            }
        }
        Self {
            ids,
            nodes,
            connections,
        }
    }
}

impl Graph {
    fn merge_nodes(&mut self, node1: &usize, node2: &usize) {
        let names = self.nodes.remove(node2).unwrap();
        for name in names.iter() {
            self.ids.insert(name.clone(), *node1);
        }
        self.nodes.get_mut(node1).unwrap().extend(names.into_iter());
        let mut node_cons = self.connections.remove(node2).unwrap();
        node_cons.remove(node1);
        self.connections.get_mut(node1).unwrap().remove(node2);
        for (id, &w) in node_cons.iter() {
            assert_eq!(
                w,
                self.connections.get_mut(id).unwrap().remove(node2).unwrap()
            );
            let new_w = self
                .connections
                .get_mut(node1)
                .unwrap()
                .remove(id)
                .unwrap_or(0)
                + w;
            self.connections.get_mut(node1).unwrap().insert(*id, new_w);
            self.connections.get_mut(id).unwrap().insert(*node1, new_w);
        }
    }
}

// use [stoer-wagner Algorithm](https://dl.acm.org/doi/pdf/10.1145/263867.263872)

fn minimun_cut_phase(graph: &Graph, start: &usize) -> Option<(usize, usize, usize)> {
    if graph.nodes.len() < 2 || !graph.nodes.contains_key(start) {
        return None;
    }
    let mut weights: HashMap<usize, usize> = HashMap::with_capacity(graph.nodes.len());

    let mut last = Some(*start);
    let mut pre = None;
    let mut groups = HashSet::new();
    for _ in 1..graph.nodes.len() {
        groups.insert(last.unwrap());
        weights.remove(&last.unwrap());
        for (&nb_id, &w) in graph.connections.get(&last.unwrap()).unwrap().iter() {
            if groups.contains(&nb_id) {
                continue;
            }
            let new_w = weights.remove(&nb_id).unwrap_or(0) + w;
            weights.insert(nb_id, new_w);
        }
        pre.replace(last.take().unwrap());
        let mut max_weight = 0;
        for (&id, &w) in weights.iter() {
            if w > max_weight {
                max_weight = w;
                last.replace(id);
            }
        }
    }
    let pre = pre.unwrap();
    let last = last.unwrap();
    Some((weights.remove(&last).unwrap(), last, pre))
}

fn calculate_part1(mut graph: Graph) -> usize {
    let start = 0;
    while let Some((cut_phase, last, pre)) = minimun_cut_phase(&graph, &start) {
        if cut_phase == 3 {
            let group1 = graph.nodes.get(&last).unwrap().len();
            let group2 = graph.ids.len() - group1;
            return group1 * group2;
        } else {
            graph.merge_nodes(&pre, &last);
        }
    }
    return 0;
}

fn main() {
    let mut f = File::open("./input").expect("Failed to open input file");
    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect("Failed to read input file");
    let graph = Graph::from(&lines);
    let part1 = calculate_part1(graph);
    println!("Part1: {}", part1);
}
