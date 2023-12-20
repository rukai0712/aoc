use std::{
    collections::HashMap,
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

struct Part {
    props: Vec<Prop>,
    score: usize,
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
}

impl Machine {
    fn handle_part(&self, part: &Part) -> bool {
        let mut w_idx = Some(0);
        while w_idx.is_some() {
            let w = &self.workflows[w_idx.unwrap()];
            match w.handle_part(part) {
                Behavior::Accept => {
                    return true;
                },
                Behavior::Reject => {
                    return false;
                },
                Behavior::Jump(idx) => {
                    w_idx.replace(idx);
                }
            }
        }
        false
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

}
