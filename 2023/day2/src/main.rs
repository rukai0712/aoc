use std::{fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn play_game_part1(remain: &str) -> bool {
    let rgb_re: Regex = Regex::new(r"(?<count>[0-9]+)\s+(?<color>red|green|blue)").unwrap();
    rgb_re.captures_iter(remain).fold(true,|fit, caps: regex::Captures<'_>| {
        if !fit {
            return fit;
        }
        let count: u32 = caps.name("count").unwrap().as_str().parse().unwrap();
        let color = caps.name("color").unwrap().as_str();
        match color {
            "red" => count <= 12,
            "green" => count <= 13,
            "blue" => count <= 14,
            _ => false
        }
    })
}

fn play_game_part2(remain: &str) -> u32 {
    let rgb_re: Regex = Regex::new(r"(?<count>[0-9]+)\s+(?<color>red|green|blue)").unwrap();
    let min: [Option<u32>;3] = rgb_re.captures_iter(remain).fold([None, None, None], |mut cur, caps| {
        let count: u32 = caps.name("count").unwrap().as_str().parse().unwrap();
        let color = caps.name("color").unwrap().as_str();
        match color {
            "red" => {
                if cur[0].is_none() || cur[0].unwrap() < count {
                    cur[0].replace(count);
                }
            },
            "green" => {
                if cur[1].is_none() || cur[1].unwrap() < count {
                    cur[1].replace(count);
                }
            },
            "blue" => {
                if cur[2].is_none() || cur[2].unwrap() < count {
                    cur[2].replace(count);
                }
            },
            _ => {}
        };
        cur
    });
    min[0].unwrap_or(1) * min[1].unwrap_or(1) * min[2].unwrap_or(1)
}


fn main() {
    let f = File::open("./input").expect("Failed to open input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    let game_re: Regex = Regex::new(r"^Game\s+([0-9]+)").unwrap();

    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        // line.split_at(mid)

        if let Some(caps) = game_re.captures(&line) {
            let game_idx: u32 = caps[1].parse().unwrap();
            let remain = line.get(caps[0].len()..).unwrap();
            if play_game_part1(remain) {
                sum_part1 += game_idx;
            }
            sum_part2 += play_game_part2(remain);
        } else {
            println!("Not match: {}", &line);
        }

        line.clear();
    }

    println!("part 1 {}", sum_part1);
    println!("part 2 {}", sum_part2);
}
