use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Test,
}

#[derive(Debug, Default, Clone)]
struct NodeConnection {
    from: Vec<usize>,
    to: Vec<usize>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlipFlop {
    on: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Conjunction {
    on: Vec<bool>,
}

#[derive(Debug, Clone)]
struct Machine {
    ids: HashMap<String, usize>,
    connections: Vec<NodeConnection>,
    nodes: Vec<Node>,
}

struct MachineBuilder {
    nodes: HashMap<String, Node>,
    connections: HashMap<String, NodeConnection>,
    ids: HashMap<String, usize>,
}

impl FlipFlop {
    fn handle_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;
                if self.on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
        }
    }

    fn clear(&mut self) {
        self.on = false;
    }
}

impl Conjunction {
    fn init(&mut self, from_len: usize) {
        assert!(self.on.len() == 0);
        self.on = [false].repeat(from_len);
    }

    fn handle_pulse(&mut self, from_idx: usize, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => {
                self.on[from_idx] = true;
            }
            Pulse::Low => {
                self.on[from_idx] = false;
            }
        }
        if self.on.iter().fold(true, |crr, v| crr && *v) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn clear(&mut self) {
        self.on = [false].repeat(self.on.len());
    }
}

impl MachineBuilder {
    fn new() -> Self {
        let mut ids: HashMap<String, usize> = HashMap::new();
        ids.insert("broadcaster".to_string(), 0);
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            ids,
        }
    }

    fn add_line(&mut self, line: &str) {
        let line = line.trim();
        let mut f = line.split("->");
        let node_name = f.next().unwrap().trim();
        let mut f = f.next().unwrap().split(',');
        let name = if node_name.starts_with("%") {
            let name = &node_name[1..];
            self.nodes
                .insert(name.to_string(), Node::FlipFlop(FlipFlop::default()));
            name
        } else if node_name.starts_with("&") {
            let name = &node_name[1..];
            self.nodes
                .insert(name.to_string(), Node::Conjunction(Conjunction::default()));
            name
        } else {
            assert!(node_name == "broadcaster");
            let name = node_name;
            self.nodes.insert(name.to_string(), Node::Broadcaster);
            name
        };
        if !self.ids.contains_key(name) {
            self.ids.insert(name.to_string(), self.ids.len());
        }
        let node_id = self.ids.get(name).unwrap().clone();
        if !self.connections.contains_key(name) {
            self.connections
                .insert(name.to_string(), NodeConnection::default());
        }
        while let Some(link_to) = f.next() {
            let link_to = link_to.trim();
            if link_to.len() == 0 {
                continue;
            }
            if !self.ids.contains_key(link_to) {
                self.ids.insert(link_to.to_string(), self.ids.len());
            }
            let link_to_id = self.ids.get(link_to).unwrap().clone();
            if !self.connections.contains_key(link_to) {
                self.connections
                    .insert(link_to.to_string(), NodeConnection::default());
            }
            self.connections
                .get_mut(link_to)
                .unwrap()
                .from
                .push(node_id);
            self.connections.get_mut(name).unwrap().to.push(link_to_id);
        }
    }

    fn build(mut self) -> Machine {
        let names: Vec<String> = self
            .ids
            .iter()
            .filter_map(|(name, _)| {
                if !self.nodes.contains_key(name) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();
        for name in names.into_iter() {
            self.nodes.insert(name, Node::Test);
        }
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        for (name, &idx) in self.ids.iter() {
            let node = self.nodes.remove(name).unwrap();
            let connection = self.connections.remove(name).unwrap();
            let node = match node {
                Node::Conjunction(mut state) => {
                    state.init(connection.from.len());
                    Node::Conjunction(state)
                }
                _ => node,
            };
            nodes.push((node, idx));
            connections.push((connection, idx));
        }
        nodes.sort_by(|a, b| a.1.cmp(&b.1));
        connections.sort_by(|a, b| a.1.cmp(&b.1));
        Machine {
            ids: self.ids,
            nodes: nodes.into_iter().map(|(node, _)| node).collect(),
            connections: connections
                .into_iter()
                .map(|(connection, _)| connection)
                .collect()
        }
    }
}

impl Machine {
    fn press(&mut self, start: &str, stop: Option<&str>, stop_status: Option<&Node>) -> (usize, usize, bool) {
        let mut low_count: usize = 1;
        let mut high_count = 0;
        let mut find_stop_state = false;
        let mut bfs = VecDeque::new();
        let start = self.ids.get(start).unwrap().clone();
        let stop = stop.map(|s| self.ids.get(s).unwrap().clone());
        let pulse = match self.nodes.get_mut(start).unwrap() {
            Node::Broadcaster => {
                Some(Pulse::Low)
            },
            Node::FlipFlop(state) => {
                state.handle_pulse(Pulse::Low)
            },
            _ => unreachable!()
        };
        if let Some(pulse) = pulse {
            for &to_id in self.connections.get(start).unwrap().to.iter() {
                bfs.push_back((start, pulse, to_id));
            }
        }
        while let Some((from_id, pulse, to_id)) = bfs.pop_front() {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }
            let from_idx: usize = self
                .connections
                .get(to_id)
                .unwrap()
                .from
                .iter()
                .position(|&v| v == from_id)
                .unwrap();
            let node = self.nodes.get_mut(to_id).unwrap();
            let pulse = match node {
                Node::Broadcaster => Some(pulse),
                Node::Conjunction(state) => state.handle_pulse(from_idx, pulse),
                Node::FlipFlop(state) => state.handle_pulse(pulse),
                Node::Test => None,
            };
            if let Some(stop) = stop {
                if stop == to_id {
                    // cut pulse when reached to stop
                    if *node == *stop_status.unwrap() {
                        find_stop_state = true;
                    }
                    continue;
                }
            }
            if let Some(pulse) = pulse {
                for &next_id in self.connections.get(to_id).unwrap().to.iter() {
                    bfs.push_back((to_id, pulse, next_id))
                }
            }
        }
        (low_count, high_count, find_stop_state)
    }

    fn reset(&mut self) {
        for node in self.nodes.iter_mut() {
            match node {
                Node::Conjunction(state) => state.clear(),
                Node::FlipFlop(state) => state.clear(),
                _ => {}
            }
        }
    }

}

impl Machine {
    fn calcuate_repeats(&mut self, start: &str, stop: &str, stop_status: Node) -> Option<(usize, usize)> {
        self.reset();
        let mut snapshots: Vec<(Vec<Node>, usize)> = Vec::new();
        let mut press = 0;
        while press < 100000 {
            let (_, _, find) = self.press(start, Some(stop), Some(&stop_status));
            press += 1;
            if find {
                let snapshot = self.nodes.clone();
                for (pre_snapshot, pre_press) in snapshots.iter() {
                    if *pre_snapshot == snapshot {
                        let pre_press = pre_press.clone();
                        let repeat = press - pre_press;
                        return Some((pre_press, repeat));
                    }
                }
                snapshots.push((snapshot, press));
            }
        }
        None
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut builder = MachineBuilder::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        builder.add_line(&line);
        line.clear();
    }
    let mut machine = builder.build();
    let mut part1_counts = (0, 0);
    for _ in 0..1000 {
        let counts = machine.press("broadcaster", None, None);
        part1_counts.0 += counts.0;
        part1_counts.1 += counts.1;
    }
    println!("Part1 {}", part1_counts.0 * part1_counts.1);

    // Part2, input special algorithm
    let mut state = Conjunction::default();
    state.init(1);
    let repeat1 = machine.calcuate_repeats("nm", "pk", Node::Conjunction(state.clone())).unwrap();
    let repeat2 = machine.calcuate_repeats("sh", "hf", Node::Conjunction(state.clone())).unwrap();
    let repeat3 = machine.calcuate_repeats("ps", "pm", Node::Conjunction(state.clone())).unwrap();
    let repeat4 = machine.calcuate_repeats("fs", "mk", Node::Conjunction(state.clone())).unwrap();
    
    println!("{:?}", repeat1);    // (4021, 4021)
    println!("{:?}", repeat2);    // (4013, 4013)
    println!("{:?}", repeat3);    // (3881, 3881)
    println!("{:?}", repeat4);    // (3889, 3889)

    let part2 = lcm(vec![repeat1.1, repeat2.1, repeat3.1, repeat4.1]);
    println!("Part2 {}", part2);

}


fn lcm(numbers: Vec<usize>) -> usize {
    let mut lcm = 1;
    let mut remains = numbers;

    for i in 2.. {
        if remains.len() == 0 {
            break;
        }
        let mut find = true;
        while find {
            find = false;
            let cur = remains;
            remains = Vec::new();
            for &c in cur.iter() {
                if c % i == 0 {
                    find = true;
                    let c = c / i;
                    if c > 1 {
                        remains.push(c);
                    }
                } else {
                    remains.push(c);
                }
            }
            if find {
                lcm *= i;
            }
        }
    }

    lcm
}
