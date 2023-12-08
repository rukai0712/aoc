use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use regex::Regex;

struct Node {
    name: String,
    links: [String; 2],
}

impl Node {
    fn build(input: &str) -> Option<Self> {
        let input_reg = Regex::new(r"^\s*(?<name>[A-Z]{3})\s*=\s*\((?<L>[A-Z]{3}),\s*(?<R>[A-Z]{3})\)\s*$").expect("Invalid input regex");
        if let Some(caps) = input_reg.captures(input) {
            Some(Self { 
                name: caps.name("name").unwrap().as_str().to_string(),
                links: [
                    caps.name("L").unwrap().as_str().to_string(),
                    caps.name("R").unwrap().as_str().to_string(),
                ] })
        } else {
            None
        }
    }
}


fn main() {
    let f = File::open("./input").expect("Failed to read input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    // parse instructions
    reader.read_line(&mut line).expect("Failed to read instructions");
    let instructions: Vec<usize> = line.trim().chars().into_iter().map(|c| {
        match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid instruct characters")
        }
    }).collect();
    line.clear();
    reader.read_line(&mut line).expect("Failed to read line");
    assert!(line.trim().len() == 0);
    line.clear();

    // parse nodes
    let mut nodes = HashMap::<String, Node>::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        if let Some(node) = Node::build(line.trim()) {
            if nodes.insert(node.name.clone(), node).is_some() {
                panic!("Duplicated node definition.");
            }
        }
        line.clear();
    }

    let mut steps = 0;
    let mut node_name = "AAA";
    while node_name != "ZZZ" {
        let idx = steps % instructions.len();
        let instruct = instructions[idx];
        node_name = nodes.get(node_name).unwrap().links[instruct].as_str();
        steps += 1;
    }

    println!("Steps {}", steps);
}
