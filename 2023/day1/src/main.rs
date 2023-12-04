use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn parse_line_part1(line: &str) -> u32 {
    let mut num = 0;
    let line_bytes = line.as_bytes();
    for b in line_bytes {
        if let Some(c) = char::from(*b).to_digit(10) {
            num = c * 10;
            break;
        }
    }
    for b in line_bytes.iter().rev() {
        if let Some(c) = char::from(*b).to_digit(10) {
            num += c;
            break;
        }
    }
    num
}

fn parse_digit_word(s: &str) -> Option<u32> {
    match s {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        "0" => Some(0),
        _ => None,
    }
}

fn parse_line_part2(line: &str) -> u32 {
    let mut num = 0;
    for idx in 0..line.len() {
        if let Some(c) = parse_digit_word(&line[idx..idx+1]) {
            num = c * 10;
            break;
        }
        if idx >= 2 {
            if let Some(c) = parse_digit_word(&line[idx - 2..idx + 1]) {
                num = c * 10;
                break;
            }
        }
        if idx >= 3 {
            if let Some(c) = parse_digit_word(&line[idx - 3..idx + 1]) {
                num = c * 10;
                break;
            }
        }
        if idx >= 4 {
            if let Some(c) = parse_digit_word(&line[idx - 4..idx + 1]) {
                num = c * 10;
                break;
            }
        }
    }

    for idx in (0..line.len()).rev() {
        if let Some(c) = parse_digit_word(&line[idx..idx+1]) {
            num += c;
            break;
        }
        if idx + 2 < line.len() {
            if let Some(c) = parse_digit_word(&line[idx..idx + 3]) {
                num += c;
                break;
            }
        }
        if idx + 3 < line.len() {
            if let Some(c) = parse_digit_word(&line[idx..idx + 4]) {
                num += c;
                break;
            }
        }
        if idx + 4 < line.len() {
            if let Some(c) = parse_digit_word(&line[idx..idx + 5]) {
                num += c;
                break;
            }
        }
    }
    num
}

fn main() {
    let file = File::open("./input").expect("Failed to open input file");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    loop {
        match buf_reader.read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                sum_part1 += parse_line_part1(&line);
                sum_part2 += parse_line_part2(&line);
                line.clear();
            }
            Err(_) => break,
        }
    }
    println!("{}", sum_part1);
    println!("{}", sum_part2);
}

#[cfg(test)]
mod test {
    use crate::{parse_line_part1, parse_line_part2};

    #[test]
    fn test_parse_line_part1() {
        let line = "97ninesevenrhchvppnztvfbfpkzrbcone";
        let sum = parse_line_part1(line);
        println!("{}", sum);
    }

    #[test]
    fn test_parse_line_part2() {
        let line = "ninseven97rhchvppnztvfbfpkzrbcone";
        let sum = parse_line_part2(line);
        println!("{}", sum);
    }
}
