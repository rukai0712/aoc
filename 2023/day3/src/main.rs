use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct GamePart1 {
    width: usize,
    sum: u32,
    pre: Option<Vec<bool>>,
    cur: Option<Vec<bool>>,
    next: Option<Vec<bool>>,
    cur_line: Option<String>,
    next_line: Option<String>,
}

impl GamePart1 {
    fn new(width: usize) -> Self {
        Self {
            width,
            sum: 0,
            pre: None,
            cur: None,
            next: None,
            cur_line: None,
            next_line: None,
        }
    }

    fn feed(&mut self, input: String) {
        self.pre = self.cur.take();
        self.cur = self.next.take();
        self.cur_line = self.next_line.take();
        // calculate valid place in line
        let mut next = vec![false; self.width];
        let line = input.as_bytes();
        for i in 0..line.len() {
            if line[i] == b'.' || line[i].is_ascii_whitespace() || line[i].is_ascii_digit() {
                continue;
            }
            next[i] = true;
            if i > 0 {
                next[i - 1] = true;
            }
            if i + 1 < self.width {
                next[i + 1] = true
            }
        }
        self.next = Some(next);
        self.next_line = Some(input);

        self.add_numbers_in_cur();
    }

    fn end(mut self) -> u32 {
        self.pre = self.cur.take();
        self.cur = self.next.take();
        self.cur_line = self.next_line.take();
        self.add_numbers_in_cur();
        self.sum
    }

    fn add_numbers_in_cur(&mut self) {
        if let Some(cur_line) = self.cur_line.as_ref() {
            let num_re: Regex = Regex::new(r"[0-9]+").unwrap();
            let mut valid = vec![false; self.width];
            for i in 0..self.width {
                if self.pre.as_ref().map_or(false, |v| v[i])
                    || self.cur.as_ref().map_or(false, |v| v[i])
                    || self.next.as_ref().map_or(false, |v| v[i])
                {
                    valid[i] = true
                }
            }
            self.sum = num_re
                .captures_iter(cur_line)
                .fold(self.sum, |mut acc, caps| {
                    let m = caps.get(0).unwrap();
                    let num: u32 = m.as_str().parse().unwrap();
                    for idx in m.start()..m.end() {
                        if valid[idx] {
                            acc += num;
                            break;
                        }
                    }
                    acc
                });
        }
    }
}

struct GamePart2 {
    width: usize,
    sum: u32,
    pre_num: Option<Vec<u32>>,
    cur_num: Option<Vec<u32>>,
    next_num: Option<Vec<u32>>,
    pre: Option<Vec<Option<usize>>>,  // index in pre_num
    cur: Option<Vec<Option<usize>>>,  // index in cur_num
    next: Option<Vec<Option<usize>>>, // index in next_num
    cur_line: Option<String>,
    next_line: Option<String>,
}

impl GamePart2 {
    fn new(width: usize) -> Self {
        Self {
            width,
            sum: 0,
            pre_num: None,
            cur_num: None,
            next_num: None,
            pre: None,
            cur: None,
            next: None,
            cur_line: None,
            next_line: None,
        }
    }

    fn feed(&mut self, input: String) {
        self.pre_num = self.cur_num.take();
        self.cur_num = self.next_num.take();
        self.pre = self.cur.take();
        self.cur = self.next.take();
        self.cur_line = self.next_line.take();

        let num_re: Regex = Regex::new(r"[0-9]+").unwrap();
        let mut next_num: Vec<u32> = vec![];
        let mut next: Vec<Option<usize>> = vec![None; self.width];
        for caps in num_re.captures_iter(&input) {
            let m = caps.get(0).unwrap();
            let num: u32 = m.as_str().parse().unwrap();
            for i in m.start()..m.end() {
                next[i].replace(next_num.len());
            }
            next_num.push(num);
        }
        self.next_num = Some(next_num);
        self.next = Some(next);
        self.next_line = Some(input);

        self.add_gear_in_cur();
    }

    fn end(mut self) -> u32 {
        self.pre_num = self.cur_num.take();
        self.cur_num = self.next_num.take();
        self.pre = self.cur.take();
        self.cur = self.next.take();
        self.cur_line = self.next_line.take();
        self.add_gear_in_cur();
        self.sum
    }

    fn add_gear_in_cur(&mut self) {
        if let Some(line) = self.cur_line.as_ref().map(|l| l.as_bytes()) {
            for i in 0..line.len() {
                if line[i] == b'*' {
                    let range = 0.max(i - 1)..self.width.min(i + 2);
                    let mut nums: Vec<u32> = vec![];
                    if let Some(cur) = self.pre.as_ref() {
                        let mut p = None;
                        for idx in range.clone() {
                            if let Some(c) = cur[idx] {
                                if p.is_none() || p.unwrap() != c {
                                    nums.push(self.pre_num.as_ref().unwrap()[c]);
                                    p.replace(c);
                                }
                            }
                        }
                    }
                    if let Some(cur) = self.cur.as_ref() {
                        let mut p = None;
                        for idx in range.clone() {
                            if let Some(c) = cur[idx] {
                                if p.is_none() || p.unwrap() != c {
                                    nums.push(self.cur_num.as_ref().unwrap()[c]);
                                    p.replace(c);
                                }
                            }
                        }
                    }
                    if let Some(cur) = self.next.as_ref() {
                        let mut p = None;
                        for idx in range.clone() {
                            if let Some(c) = cur[idx] {
                                if p.is_none() || p.unwrap() != c {
                                    nums.push(self.next_num.as_ref().unwrap()[c]);
                                    p.replace(c);
                                }
                            }
                        }
                    }
                    if nums.len() == 2 {
                        self.sum += nums[0] * nums[1];
                    }
                }
            }
        }
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open expected file");
    let mut reader = BufReader::new(f);
    let mut line = Some(String::new());

    let width = reader
        .read_line(line.as_mut().unwrap())
        .expect("Invalid input");
    let mut part1 = GamePart1::new(width);
    let mut part2 = GamePart2::new(width);
    let mut size = width;
    while size > 0 {
        part1.feed(line.clone().take().unwrap());
        part2.feed(line.clone().take().unwrap());
        line = Some(String::new());
        size = reader
            .read_line(line.as_mut().unwrap())
            .expect("Failed to read line");
    }

    println!("{}", part1.end());
    println!("{}", part2.end());
}
