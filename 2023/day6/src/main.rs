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
    let single_peak = (t + 1) % 2 == 0;
    let mut remain = distance.to_string();
    let mut bottom = 0;
    let mut result = 0;
    let mut negative = false;
    while bottom < peak {
        bottom += 1;
        let delta = t - 2*bottom + 1;
        (remain, negative) = sub(&remain, delta);
        if negative {
            result = (peak + 1 - bottom) * 2;
            if single_peak {
                result -= 1;
            }
        }
    }
    result
}

fn sub(lhs: &str, rhs: u32) -> (String, bool) {
 todo!()
}
