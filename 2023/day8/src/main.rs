use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

struct LinkInput(String, String, String);

impl LinkInput {
    fn new(input: &str) -> Option<Self> {
        let input_reg =
            Regex::new(r"^\s*(?<name>[\w]{3})\s*=\s*\((?<L>[\w]{3}),\s*(?<R>[\w]{3})\)\s*$")
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

// steps take to the nearest Z
#[derive(Debug, Clone, Copy)]
struct Steps2Z {
    z_no: usize,    // node no which ends with z
    inst_no: usize, // instruction no
    len: u64,       // length to z_no node
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
    let mut name_map: HashMap<String, usize> = HashMap::new();
    // let endswith
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        if let Some(link) = LinkInput::new(line.trim()) {
            let no = links.len();
            if name_map.insert(link.0.clone(), no).is_some() {
                panic!("Duplicated node definition.");
            }
            links.push(link);
        }
        line.clear();
    }

    // create nodes with nexts
    let mut nodes = Vec::<Node>::new();
    for link_input in links.iter() {
        let no = *name_map.get(&link_input.0).unwrap();
        let next_l = *name_map.get(&link_input.1).unwrap();
        let next_r = *name_map.get(&link_input.2).unwrap();
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
    let mut node = &nodes[*name_map.get("AAA").unwrap()];
    while node.name != "ZZZ" {
        let idx = part1_steps % instructions.len();
        let instruct = instructions[idx];
        node = &nodes[node.nexts[instruct]];
        part1_steps += 1;
    }
    println!("Part1 Steps {}", part1_steps);

    // node_idx, inst_idx
    let mut steps_table: Vec<Vec<Option<Steps2Z>>> = Vec::with_capacity(nodes.len());
    let mut b_search = VecDeque::<(usize, usize, Steps2Z)>::new();
    let mut starts = Vec::<usize>::new();
    let mut ends = Vec::<usize>::new();
    for node in &nodes {
        let mut ins = Vec::with_capacity(instructions.len());
        for _ in 0..instructions.len() {
            ins.push(None);
        }
        steps_table.push(ins);
        if node.name.ends_with('Z') {
            for inst_no in 0..instructions.len() {
                assert!(node.name.ends_with('Z'));
                b_search.push_back((
                    node.no,
                    inst_no,
                    Steps2Z {
                        z_no: node.no,
                        inst_no,
                        len: 0,
                    },
                ))
            }
            ends.push(node.no);
        }
        if node.name.ends_with('A') {
            starts.push(node.no);
        }
    }

    while let Some((no, inst_no, step)) = b_search.pop_front() {
        let node = &nodes[no];
        let pre_inst_no = (inst_no + instructions.len() - 1) % instructions.len();
        let pre_inst = instructions[pre_inst_no];
        for &pre_no in &node.froms[pre_inst] {
            let pre_step = steps_table[pre_no][pre_inst_no].take();
            if pre_step.is_some() && pre_step.unwrap().len <= step.len + 1 {
                steps_table[pre_no][pre_inst_no].replace(pre_step.unwrap());
                continue;
            }
            let mut pre_step = step.clone();
            pre_step.len += 1;
            b_search.push_back((pre_no, pre_inst_no, pre_step.clone()));
            steps_table[pre_no][pre_inst_no].replace(pre_step);
        }
    }

    // table from z to z loop; (z_no, inst_no)
    let mut zloop_table: HashMap<(usize, usize), u64> =
        HashMap::with_capacity(ends.len() * instructions.len());
    for &e in ends.iter() {
        for inst_no in 0..instructions.len() {
            if let Some(step) = steps_table[e][inst_no].clone() {
                let mut inst_table = Vec::with_capacity(instructions.len());
                for _ in 0..instructions.len() {
                    inst_table.push(HashMap::with_capacity(ends.len()));
                }
                let mut b_search = VecDeque::new();
                inst_table[step.inst_no].insert(step.z_no, step.len);
                b_search.push_back(step);

                while let Some(step) = b_search.pop_front() {
                    if let Some(mut next) = steps_table[step.z_no][step.inst_no].clone() {
                        next.len += step.len;
                        if !inst_table[next.inst_no].contains_key(&next.z_no) {
                            inst_table[next.inst_no].insert(next.z_no, next.len);
                            b_search.push_back(next);
                        }
                    }
                }
                if inst_table[inst_no].contains_key(&e) {
                    zloop_table.insert((e, inst_no), inst_table[inst_no][&e]);
                }
            }
        }
    }

    // table from a to z in inst; a_no, inst_no, z_no
    let mut a2z_table: HashMap<usize, Vec<HashMap<usize, u64>>> =
        HashMap::with_capacity(starts.len());
    for &a in starts.iter() {
        let mut inst_table = Vec::with_capacity(instructions.len());
        for _ in 0..instructions.len() {
            inst_table.push(HashMap::with_capacity(ends.len()));
        }
        let mut b_search = VecDeque::new();
        if let Some(step) = steps_table[a][0].clone() {
            inst_table[step.inst_no].insert(step.z_no, step.len);
            b_search.push_back(step);
        }
        while let Some(step) = b_search.pop_front() {
            if let Some(mut next) = steps_table[step.z_no][step.inst_no].clone() {
                next.len += step.len;
                if !inst_table[next.inst_no].contains_key(&next.z_no) {
                    inst_table[next.inst_no].insert(next.z_no, next.len);
                    b_search.push_back(next);
                }
            }
        }
        a2z_table.insert(a, inst_table);
    }

    let mut part2 = None;
    for inst_no in 0..instructions.len() {
        if let Some(count) = find_answer(
            inst_no,
            instructions.len(),
            Vec::new(),
            &starts,
            &ends,
            &a2z_table,
            &zloop_table,
        ) {
            if part2.is_none() || part2.unwrap() > count {
                part2.replace(count);
            }
        }
    }

    if let Some(part2) = part2 {
        println!("Part2 {}", part2)
    } else {
        println!("Not find");
    }
}

fn find_answer(
    inst_no: usize,
    inst_length: usize,
    start2end: Vec<usize>,
    starts: &Vec<usize>,
    ends: &Vec<usize>,
    a2z_table: &HashMap<usize, Vec<HashMap<usize, u64>>>,
    zloop_table: &HashMap<(usize, usize), u64>,
) -> Option<u64> {
    if start2end.len() == starts.len() {
        println!("========{}/{}========", inst_no, inst_length);
        let mut a2z = Vec::new();
        let mut z2z = Vec::new();
        for i in 0..start2end.len() {
            let a_no = starts[i];
            let e_no = start2end[i];
            let &h = a2z_table.get(&a_no).unwrap()[inst_no].get(&e_no).unwrap();
            let &l = zloop_table.get(&(e_no, inst_no)).unwrap();
            let al = (h - inst_no as u64) / inst_length as u64;
            let zl = l / inst_length as u64;
            println!("{}->{}={} ({}*{}+{}); {}->{}={} ({}*{});", a_no, e_no, h, al, inst_length, inst_no, e_no, e_no, l, zl, inst_length);
            a2z.push(al);
            z2z.push(zl);
        }
        /*
        ========0/283========
        29->285=20093 (71*283+0); 285->285=20093 (71*283);
        225->363=12169 (43*283+0); 363->363=12169 (43*283);
        244->9=13301 (47*283+0); 9->9=13301 (47*283);
        261->417=20659 (73*283+0); 417->417=20659 (73*283);
        306->18=16697 (59*283+0); 18->18=16697 (59*283);
        666->233=17263 (61*283+0); 233->233=17263 (61*283);
        */
        // only calcuate when a2z == z2z
        if a2z != z2z {
            panic!("Not implemented Case");
        }
        // As there's only one candidate, so use fake algrithem to calcaute LCM for z2z
        let lcm = z2z.iter().fold(1, |cur, v| {cur * *v});
        return Some(lcm * (inst_length as u64) + inst_no as u64);
    }

    let i = start2end.len();
    let a_no = starts[i];
    let mut result = None;
    for &z_no in a2z_table.get(&a_no).unwrap().get(inst_no).unwrap().keys() {
        let mut new_s2e = start2end.clone();
        new_s2e.push(z_no);
        if let Some(r) = find_answer(
            inst_no,
            inst_length,
            new_s2e,
            starts,
            ends,
            a2z_table,
            zloop_table,
        ) {
            if result.is_none() || result.unwrap() > r {
                result.replace(r);
            }
        }
    }
    result
}
