use std::{fs::File, io::{BufReader, BufRead}};


fn calculate_part1(line: &Vec<i64>) -> (i64, i64) {
    let n_zero = line.iter().fold(false, |acc, v| acc || *v != 0);
    if !n_zero {
        return (0, 0);
    }
    let mut new_line = Vec::with_capacity(line.len()-1);
    for i in 0..line.len()-1 {
        new_line.push(line[i+1] - line[i]);
    }
    let (pre, next) = calculate_part1(&new_line);
    return (line.first().unwrap()-pre, next + line.last().unwrap());
}



fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut part1 = 0;
    let mut part2 = 0;
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        let digit: Vec<i64> = line.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let (pre, next) = calculate_part1(&digit);
        part1 += next;
        part2 += pre;
        line.clear();
    }

    println!("Part1 {}", part1);
    println!("Part1 {}", part2);
}
