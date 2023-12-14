use core::num;
use std::{
    fs::File,
    io::{BufRead, BufReader}, iter::Map, ops::Deref,
};

#[derive(Debug, Clone, Copy, Hash)]
enum SprintStatus {
    Normal,
    Broken,
}

struct Record {
    sprints: Vec<Option<SprintStatus>>,
    nums: Vec<u8>,
}

impl Record {
    fn build(input: &str) -> Self {
        let pieces: Vec<&str> = input.trim().split_whitespace().collect();
        let sprints: Vec<Option<SprintStatus>> = pieces[0]
            .chars()
            .map(|c| match c {
                '.' => Some(SprintStatus::Normal),
                '#' => Some(SprintStatus::Broken),
                '?' => None,
                _ => panic!("Invalid Character"),
            })
            .collect();
        let nums = pieces[1]
            .split(',')
            .map(|n| n.parse().expect("Invalid Number"))
            .collect();
        Self { sprints, nums }
    }
}


impl Record {
    
    fn possibility(&self) -> usize {
        let mut pre_0: Vec<usize> = [0].repeat(self.sprints.len());
        let mut pre_1: Vec<usize> = [0].repeat(self.sprints.len());
        for i in 0..self.sprints.len() {
            match self.sprints[i] {
             Some(SprintStatus::Normal) | None => {
                pre_0[i] = 1;
             },
             _ => {
                break
             } 
            }
        }
        for num_idx in 0..self.nums.len() {
            // let num = (*num) as usize;
            let mut linked = 0;
            while linked < self.nums[num_idx] {
                linked += 1;
                let mut cur_0: Vec<usize> = [0].repeat(self.sprints.len());
                let mut cur_1: Vec<usize> = [0].repeat(self.sprints.len());
                match self.sprints[0] {
                    Some(SprintStatus::Normal) => {
                        cur_0[0] = 0;
                        cur_1[0] = 0;
                    },
                    Some(SprintStatus::Broken) | None => {
                        if linked == 1 {
                            if num_idx == 0 {
                                cur_1[0] = 1;
                            } else {
                                cur_1[0] = 0;
                            }
                        } else {
                            cur_1[0] = 0;
                        }
                        cur_0[0] = 0;
                    },
                }

                for i in 1..self.sprints.len() {
                    match self.sprints[i] {
                        Some(SprintStatus::Normal) => {
                            cur_0[i] = cur_0[i-1] + cur_1[i-1];
                            cur_1[i] = 0;
                        },
                        Some(SprintStatus::Broken) => {
                            if linked == 1 {
                                cur_1[i] = pre_0[i-1];
                            } else {
                                cur_1[i] = pre_1[i-1];
                            }
                            cur_0[i] = 0;
                        },
                        None => {
                            cur_0[i] = cur_0[i-1] + cur_1[i-1];
                            if linked == 1 {
                                cur_1[i] = pre_0[i-1];
                            } else {
                                cur_1[i] = pre_1[i-1];
                            }
                        }
                    }
                }
                pre_0 = cur_0;
                pre_1 = cur_1;
            }
        }

        match self.sprints.last().unwrap() {
            Some(SprintStatus::Normal) => {
                *pre_0.last().unwrap()
            },
            Some(SprintStatus::Broken) => {
                *pre_1.last().unwrap()
            },
            None => {
                *pre_0.last().unwrap() + *pre_1.last().unwrap()
            }
        }
    }

    fn unfold(&self) -> Self {
        let mut sprints = self.sprints.clone();
        let nums = self.nums.clone().repeat(5);
        for _ in 0..4 {
            sprints.push(None);
            sprints.extend(self.sprints.iter());
        }
        Self { sprints, nums }
    }

}

fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut records = Vec::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        let record = Record::build(&line);
        records.push(record);
        line.clear();
    }

    let mut part1 = 0;
    for record in records.iter() {
        part1 += record.possibility();
    }

    println!("Part1 {}", part1);

    let records: Vec<Record> = records.iter().map(|r| r.unfold()).collect();
    let mut part2 = 0;
    for record in records.iter() {
        part2 += record.possibility();
    }

    println!("Part2 {}", part2);

}
