use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

struct LinkInput(String, String, String);

impl LinkInput {
    fn new(input: &str) -> Option<Self> {
        let input_reg =
            Regex::new(r"^\s*(?<name>[A-Z]{3})\s*=\s*\((?<L>[A-Z]{3}),\s*(?<R>[A-Z]{3})\)\s*$")
                .expect("Invalid input regex");
        if let Some(caps) = input_reg.captures(input) {
            Some(Self(
                caps.name("name").unwrap().as_str().to_string(),
                caps.name("L").unwrap().as_str().to_string(),
                caps.name("R").unwrap().as_str().to_string(),
            ))
        } else {
            None
        }
    }
}

struct Node {
    no: usize,
    name: String,
    nexts: [usize; 2],
    froms: [Vec<usize>; 2],
}

fn main() {
    let f = File::open("./input").expect("Failed to read input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    // parse instructions
    reader
        .read_line(&mut line)
        .expect("Failed to read instructions");
    let instructions: Vec<usize> = line
        .trim()
        .chars()
        .into_iter()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid instruct characters"),
        })
        .collect();
    line.clear();
    reader.read_line(&mut line).expect("Failed to read line");
    assert!(line.trim().len() == 0);
    line.clear();

    // parse nodes
    let mut links = Vec::<LinkInput>::new();
    let mut nameMap: HashMap<String, usize> = HashMap::new();
    // let endswith
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        if let Some(link) = LinkInput::new(line.trim()) {
            let no = links.len();
            if nameMap.insert(link.0.clone(), no).is_some() {
                panic!("Duplicated node definition.");
            }
            links.push(link);
        }
        line.clear();
    }

    // create nodes with nexts
    let mut nodes = Vec::<Node>::new();
    for link_input in links.iter() {
        let no = *nameMap.get(&link_input.0).unwrap();
        let next_l = *nameMap.get(&link_input.1).unwrap();
        let next_r = *nameMap.get(&link_input.2).unwrap();
        let node = Node {
            no,
            name: link_input.0.clone(),
            nexts: [next_l, next_r],
            froms: [Vec::new(), Vec::new()],
        };
        nodes.push(node);
    }

    // update froms in nodes
    for i in 0..nodes.len() {
        let node = &nodes[i];
        let no = node.no;
        let next_l = node.nexts[0];
        let next_r = node.nexts[1];
        nodes[next_l].froms[0].push(no);
        nodes[next_r].froms[1].push(no);
    }

    let mut part1_steps = 0;
    let mut node = &nodes[*nameMap.get("AAA").unwrap()];
    while node.name != "ZZZ" {
        let idx = part1_steps % instructions.len();
        let instruct = instructions[idx];
        node = &nodes[node.nexts[instruct]];
        part1_steps += 1;
    }
    println!("Part1 Steps {}", part1_steps);
}
