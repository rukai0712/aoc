use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_win_count(line: String) -> u32 {
    let mut count: u32 = 0;
    let parts: Vec<&str> = line.trim().split(&[':', '|']).collect();
    let win_nums: Vec<&str> = parts[1].split(char::is_whitespace).collect();
    let mut win_nums: Vec<u32> = win_nums.iter().filter_map(|s| s.parse().ok()).collect();
    let raw_nums: Vec<&str> = parts[2].split(char::is_whitespace).collect();
    let raw_nums: Vec<u32> = raw_nums.iter().filter_map(|s| s.parse().ok()).collect();
    win_nums.sort();
    for num in &raw_nums {
        if win_nums.binary_search(num).is_ok() {
            count += 1;
        }
    }
    count
}

struct Card {
    idx: usize,
    copy: u32,
}

struct Part2 {
    sum: u32,
    candidates: VecDeque<Card>,
}

impl Part2 {
    fn new() -> Self {
        Self {
            sum: 0,
            candidates: VecDeque::new(),
        }
    }
    fn feed_card(&mut self, idx: usize, win: u32) {
        let card = if !self.candidates.front().map_or(false, |v| v.idx == idx) {
            Card { idx, copy: 0 }
        } else {
            self.candidates.pop_front().unwrap()
        };
        let copy = card.copy + 1;
        self.sum += copy;
        // update following candidates
        for i in 0..win as usize {
            if self.candidates.len() < i + 1 {
                self.candidates.push_back(Card {
                    idx: i + card.idx + 1,
                    copy: 0,
                });
            }
            self.candidates[i].copy += copy;
        }
    }

    fn end(self) -> u32 {
        self.sum
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line_buffer = Some(String::new());
    let mut part1_total: u32 = 0;
    let mut idx: usize = 0;
    let mut part2 = Part2::new();
    while let Ok(size) = reader.read_line(line_buffer.as_mut().unwrap()) {
        if size == 0 {
            break;
        }
        let win_count = calculate_win_count(line_buffer.take().unwrap());
        if win_count > 0 {
            part1_total += 2_i32.pow(win_count - 1) as u32;
        }
        part2.feed_card(idx, win_count);
        line_buffer.replace(String::new());
        idx += 1;
    }
    println!("Part1 {}", part1_total);
    println!("Part2 {}", part2.end());
}
