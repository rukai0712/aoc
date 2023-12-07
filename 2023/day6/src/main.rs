use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("./input").expect("Failed to open input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read time");
    let times: Vec<String> = line
        .split(&char::is_whitespace)
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|v| v.parse::<u32>().ok())
        .map(|v| v.to_string())
        .collect();
    line.clear();
    reader
        .read_line(&mut line)
        .expect("Failed to read distance");
    let distances: Vec<String> = line
        .split(&char::is_whitespace)
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|v| v.parse::<u32>().ok())
        .map(|v| v.to_string())
        .collect();

    let mut part1 = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        part1 *= resolve(time.as_str(), distance.as_str());
    }
    println!("Part1 {}", part1);
    let time = times.join("");
    let distance = distances.join("");
    let part2 = resolve(time.as_str(), distance.as_str());
    println!("Part2 {}", part2);
}

fn resolve(time: &str, distance: &str) -> u32 {
    let t: u32 = time.parse().expect("invalid value");
    let peak = (t + 1) / 2;
    let record : Vec<u8>= distance.chars().map(|c| c.to_digit(10).unwrap() as u8).rev().collect(); // in little-endian
    let mut sum: Vec<u8> = vec![0];
    let mut button = None;
    let mut top = None;
    let mut cur = 0;
    while cur < peak {
        cur += 1;
        let delta = t + 1 - 2*cur;
        sum = add(&sum, delta);
        if button.is_none() && large(&sum, &record) {
            button.replace(cur);
        }
    }

    while cur < t {
        cur += 1;
        let delta = 2*cur - 1 - t;
        sum = saturating_sub(&sum, delta);
        if top.is_none() && !large(&sum, &record) {
            top.replace(cur);
        }
    }
    if let Some(b) = button {
        let t = top.unwrap_or(t);
        t - b
    } else {
        0
    }
}

fn add(lhs: &Vec<u8>, rhs: u32) -> Vec<u8> {
    let mut d = rhs;
    let mut i = 0;
    let mut remain = Vec::new();
    while i < lhs.len() {
        let s = (d % 10) as u8;
        d = d / 10;
        let sum = lhs[i] + s;
        remain.push(sum % 10);
        d += (sum / 10) as u32;
        i += 1;
    }
    while d > 0 {
        remain.push((d % 10) as u8);
        d = d / 10;
    }
    remain
}

fn saturating_sub(lhs: &Vec<u8>, rhs: u32) -> Vec<u8> {
    let mut d = rhs;
    let mut i = 0;
    let mut remain = Vec::new();
    while i < lhs.len() {
        let s = (d % 10) as u8;
        d = d / 10;
        if lhs[i] >= s {
            remain.push(lhs[i] - s);
        } else {
            remain.push(10 + lhs[i] - s);
            d += 1;
        }
        i += 1;
    }
    if d > 0 {
        vec![0]
    } else {
        while remain.len() > 0 && remain.last().is_some_and(|v| *v == 0) {
            remain.pop();
        }
        remain
    }
}

fn large(lhs: &Vec<u8>, rhs: &Vec<u8>) -> bool {
    if rhs.len() > lhs.len() {
        return false;
    } else if rhs.len() < lhs.len() {
        return true;
    }
    let mut i = rhs.len();
    while i > 0 {
        i -= 1;
        if lhs[i] > rhs[i] {
            return true
        } else if lhs[i] < rhs[i] {
            return false
        }
    }
    return false;
}
