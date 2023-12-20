use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Prop {
    X(usize),
    M(usize),
    A(usize),
    S(usize),
}

#[derive(Debug, Clone)]
enum Condition {
    Less(Prop),
    Large(Prop),
}

impl Condition {
    fn check(&self, prop: &Prop) -> Option<bool> {
        match self {
            Self::Less(cond) => match (cond, prop) {
                (Prop::X(threshold), Prop::X(v)) => Some(*v < *threshold),
                (Prop::M(threshold), Prop::M(v)) => Some(*v < *threshold),
                (Prop::A(threshold), Prop::A(v)) => Some(*v < *threshold),
                (Prop::S(threshold), Prop::S(v)) => Some(*v < *threshold),
                _ => None,
            },
            Self::Large(cond) => match (cond, prop) {
                (Prop::X(threshold), Prop::X(v)) => Some(*v > *threshold),
                (Prop::M(threshold), Prop::M(v)) => Some(*v > *threshold),
                (Prop::A(threshold), Prop::A(v)) => Some(*v > *threshold),
                (Prop::S(threshold), Prop::S(v)) => Some(*v > *threshold),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Behavior {
    Accept,
    Reject,
    Jump(usize),
}

#[derive(Debug, Clone)]
struct Part {
    props: Vec<Prop>,
    score: usize,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

struct Workflow {
    id: usize,
    name: String,
    conditions: Vec<Condition>,
    behaviors: Vec<Behavior>,
}

struct MachineBuilder {
    workflows: HashMap<String, Workflow>,
    names: HashMap<String, usize>,
}

struct Machine {
    workflows: Vec<Workflow>,
}

impl MachineBuilder {
    fn new() -> Self {
        let mut names = HashMap::new();
        names.insert("in".to_string(), 0);
        Self {
            workflows: HashMap::new(),
            names,
        }
    }

    fn add_new_workflow(&mut self, line: &str) {
        let mut f = line.trim_end().split(&['{', '}']);
        let name = f.next().unwrap().to_string();
        if !self.names.contains_key(&name) {
            self.names.insert(name.clone(), self.names.len());
        }
        let id = self.names.get(&name).unwrap().clone();
        let mut f = f.next().unwrap().split(',');
        let mut conditions = Vec::new();
        let mut behaviors = Vec::new();
        while let Some(c_str) = f.next() {
            if c_str.contains(&['>', '<']) {
                let mut c_f = c_str.split(&['>', '<', ':']);
                let prop_name = c_f.next().unwrap();
                let threshold: usize = c_f.next().unwrap().parse().unwrap();
                let prop = match prop_name {
                    "x" => Prop::X(threshold),
                    "m" => Prop::M(threshold),
                    "a" => Prop::A(threshold),
                    "s" => Prop::S(threshold),
                    _ => unreachable!(),
                };
                let condition = if c_str.contains('>') {
                    Condition::Large(prop)
                } else {
                    assert!(c_str.contains('<'));
                    Condition::Less(prop)
                };
                conditions.push(condition);
                let behavior_name = c_f.next().unwrap();
                let behavior = match behavior_name {
                    "A" => Behavior::Accept,
                    "R" => Behavior::Reject,
                    _ => {
                        if !self.names.contains_key(behavior_name) {
                            self.names
                                .insert(behavior_name.to_string(), self.names.len());
                        }
                        Behavior::Jump(self.names.get(behavior_name).unwrap().clone())
                    }
                };
                behaviors.push(behavior);
            } else {
                let behavior_name = c_str;
                let behavior = match behavior_name {
                    "A" => Behavior::Accept,
                    "R" => Behavior::Reject,
                    _ => {
                        if !self.names.contains_key(behavior_name) {
                            self.names
                                .insert(behavior_name.to_string(), self.names.len());
                        }
                        Behavior::Jump(self.names.get(behavior_name).unwrap().clone())
                    }
                };
                behaviors.push(behavior);
                break;
            }
        }
        self.workflows.insert(
            name.clone(),
            Workflow {
                id,
                name,
                conditions,
                behaviors,
            },
        );
    }

    fn build(self) -> Machine {
        let mut workflows = self
            .workflows
            .into_iter()
            .map(|(_, w)| w)
            .collect::<Vec<Workflow>>();
        workflows.sort_by(|a, b| a.id.cmp(&b.id));
        Machine { workflows }
    }
}

impl Part {
    fn new(line: &str) -> Self {
        let line = &line.trim().trim_end_matches('}')[1..];
        let mut f = line.split(',');
        let mut props = Vec::new();
        let mut score = 0;
        while let Some(p_str) = f.next() {
            let mut p_f = p_str.split('=');
            let p_name = p_f.next().unwrap();
            let p_v: usize = p_f.next().unwrap().parse().unwrap();
            score += p_v;
            let prop = match p_name {
                "x" => Prop::X(p_v),
                "m" => Prop::M(p_v),
                "a" => Prop::A(p_v),
                "s" => Prop::S(p_v),
                _ => unreachable!(),
            };
            props.push(prop);
        }
        Self { props, score }
    }
}

impl Workflow {
    fn handle_part(&self, part: &Part) -> Behavior {
        for (condition, behavior) in self.conditions.iter().zip(self.behaviors.iter()) {
            if let Some(m) = part
                .props
                .iter()
                .filter_map(|prop| condition.check(prop))
                .next()
            {
                if m {
                    return behavior.clone();
                }
            }
        }
        self.behaviors.last().unwrap().clone()
    }

    fn handle_range(&self, part_range: PartRange) -> Vec<(PartRange, Behavior)> {
        let mut sub_parts = Vec::new();
        let mut remain = Some(part_range);
        for (condition, &behavior) in self.conditions.iter().zip(self.behaviors.iter()) {
            if let Some(mut part_range) = remain.take() {
                match condition {
                    Condition::Less(Prop::X(threshold)) => {
                        let threshold = *threshold;
                        if part_range.x.0 >= threshold {
                            remain.replace(part_range);
                        } else {
                            if part_range.x.1 <= threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.x.1 = threshold;
                                part_range.x.0 = threshold;
                                if new_range.x.0 < new_range.x.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.x.0 < part_range.x.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Less(Prop::M(threshold)) => {
                        let threshold = *threshold;
                        if part_range.m.0 >= threshold {
                            remain.replace(part_range);
                        } else {
                            if part_range.m.1 <= threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.m.1 = threshold;
                                part_range.m.0 = threshold;
                                if new_range.m.0 < new_range.m.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.m.0 < part_range.m.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Less(Prop::A(threshold)) => {
                        let threshold = *threshold;
                        if part_range.a.0 >= threshold {
                            remain.replace(part_range);
                        } else {
                            if part_range.a.1 <= threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.a.1 = threshold;
                                part_range.a.0 = threshold;
                                if new_range.a.0 < new_range.a.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.a.0 < part_range.a.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Less(Prop::S(threshold)) => {
                        let threshold = *threshold;
                        if part_range.s.0 >= threshold {
                            remain.replace(part_range);
                        } else {
                            if part_range.s.1 <= threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.s.1 = threshold;
                                part_range.s.0 = threshold;
                                if new_range.s.0 < new_range.s.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.s.0 < part_range.s.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Large(Prop::X(threshold)) => {
                        let threshold = *threshold;
                        if part_range.x.1 <= threshold + 1 {
                            remain.replace(part_range);
                        } else {
                            if part_range.x.0 > threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.x.0 = threshold + 1;
                                part_range.x.1 = threshold + 1;
                                if new_range.x.0 < new_range.x.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.x.0 < part_range.x.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Large(Prop::M(threshold)) => {
                        let threshold = *threshold;
                        if part_range.m.1 <= threshold + 1 {
                            remain.replace(part_range);
                        } else {
                            if part_range.m.0 > threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.m.0 = threshold + 1;
                                part_range.m.1 = threshold + 1;
                                if new_range.m.0 < new_range.m.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.m.0 < part_range.m.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Large(Prop::A(threshold)) => {
                        let threshold = *threshold;
                        if part_range.a.1 <= threshold + 1 {
                            remain.replace(part_range);
                        } else {
                            if part_range.a.0 > threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.a.0 = threshold + 1;
                                part_range.a.1 = threshold + 1;
                                if new_range.a.0 < new_range.a.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.a.0 < part_range.a.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                    Condition::Large(Prop::S(threshold)) => {
                        let threshold = *threshold;
                        if part_range.s.1 <= threshold + 1 {
                            remain.replace(part_range);
                        } else {
                            if part_range.s.0 > threshold {
                                sub_parts.push((part_range, behavior));
                            } else {
                                let mut new_range = part_range.clone();
                                new_range.s.0 = threshold + 1;
                                part_range.s.1 = threshold + 1;
                                if new_range.s.0 < new_range.s.1 {
                                    sub_parts.push((new_range, behavior));
                                }
                                if part_range.s.0 < part_range.s.1 {
                                    remain.replace(part_range);
                                }
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
        if let Some(part_range) = remain.take() {
            sub_parts.push((part_range, self.behaviors.last().unwrap().clone()));
        }
        sub_parts
    }
}

impl Machine {
    fn handle_part(&self, part: &Part) -> bool {
        let mut w_idx = Some(0);
        while w_idx.is_some() {
            let w = &self.workflows[w_idx.unwrap()];
            match w.handle_part(part) {
                Behavior::Accept => {
                    return true;
                }
                Behavior::Reject => {
                    return false;
                }
                Behavior::Jump(idx) => {
                    w_idx.replace(idx);
                }
            }
        }
        false
    }

    fn handle_range(&self, part_range: PartRange) -> Vec<PartRange> {
        let mut bfs = VecDeque::new();
        bfs.push_back((part_range, 0));
        let mut accept_ranges = Vec::new();
        while let Some((range, w_idx)) = bfs.pop_front() {
            let workflow = &self.workflows[w_idx];
            for (sub_range, behavior) in workflow.handle_range(range).into_iter() {
                match behavior {
                    Behavior::Accept => {
                        accept_ranges.push(sub_range);
                    },
                    Behavior::Reject => {}
                    Behavior::Jump(idx) => {
                        bfs.push_back((sub_range, idx));
                    }
                }
            }
        }
        accept_ranges
    }
}

impl PartRange {

    fn count(&self) -> usize {
        (self.x.1-self.x.0)  * (self.m.1-self.m.0) * (self.a.1-self.a.0) * (self.s.1-self.s.0) 
    }
    
}

fn main() {
    let f = File::open("./input").expect("Failed to load");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut builder = MachineBuilder::new();
    while let Ok(_) = reader.read_line(&mut line) {
        let l = line.trim();
        if l.len() == 0 {
            line.clear();
            break;
        }
        builder.add_new_workflow(&l);
        line.clear();
    }
    let machine = builder.build();

    let mut parts = Vec::new();
    while let Ok(_) = reader.read_line(&mut line) {
        let l = line.trim();
        if l.len() == 0 {
            break;
        }
        let part = Part::new(&l);
        parts.push(part);
        line.clear();
    }
    let mut part1 = 0;
    for part in parts.iter() {
        if machine.handle_part(part) {
            part1 += part.score;
        }
    }
    println!("Part1 {}", part1);

    let part_range = PartRange {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };
    let accept_ranges = machine.handle_range(part_range);
    let part2 = accept_ranges.iter().fold(0, |crr, r| crr + r.count());
    println!("Part2 {}", part2);
}
