use std::{fs::File, io::{BufReader, BufRead}, char, str::FromStr};

struct Hand {
    value_part1: u32,
    value_part2: u32,
    bid: u32,
}

impl Hand {

    fn new(cards: &str, bid: u32) -> Self {
        let mut value_part1: u32 = Hand::get_rank1(cards);
        let mut value_part2: u32 = Hand::get_rank2(cards);
        let cards_bytes = cards.as_bytes();
        for i in 0..cards_bytes.len() {
            value_part1 *= 16;
            value_part2 *= 16;
            value_part1 += Hand::card_to_value1(cards_bytes[i]);
            value_part2 += Hand::card_to_value2(cards_bytes[i]);
        }
        Self {
            value_part1,
            value_part2,
            bid
        }
    }

    fn get_rank1(cards: &str) -> u32 {
        let mut repeats: [u8;14] = [0;14];
        let cards_bytes = cards.as_bytes();
        for i in 0..cards_bytes.len() {
            let idx = Hand::card_to_value1(cards_bytes[i]) as u32;
            repeats[idx as usize] += 1;
        }
        let mut pattern: [u8; 5] = [0; 5];
        for repeat in repeats[1..].into_iter() {
            if *repeat == 0 {
                continue;
            }
            pattern[*repeat as usize -1] += 1;
        }

        match pattern {
            [0, 0, 0, 0, 1] => 7,
            [1, 0, 0, 1, 0] => 6,
            [0, 1, 1, 0, 0] => 5,
            [2, 0, 1, 0, 0] => 4,
            [1, 2, 0, 0, 0] => 3,
            [3, 1, 0, 0, 0] => 2,
            [5, 0, 0, 0, 0] => 1,
            _ => 0
        }
    }

    fn get_rank2(cards: &str) -> u32 {
        let mut repeats: [u8;14] = [0;14];
        let cards_bytes = cards.as_bytes();
        for i in 0..cards_bytes.len() {
            let idx = Hand::card_to_value2(cards_bytes[i]) as u32;
            repeats[idx as usize] += 1;
        }

        let j_count = repeats[0];
        let mut max_count = 0;
        let mut max_idx = 1;
        for i in 1..14 {
            if repeats[i] > max_count {
                max_count = repeats[i];
                max_idx = i;
            }
        }
        repeats[max_idx] += j_count;

        let mut pattern: [u8; 5] = [0; 5];
        for repeat in repeats[1..].into_iter() {
            if *repeat == 0 {
                continue;
            }
            pattern[*repeat as usize -1] += 1;
        }
        match pattern {
            [0, 0, 0, 0, 1] => 7,
            [1, 0, 0, 1, 0] => 6,
            [0, 1, 1, 0, 0] => 5,
            [2, 0, 1, 0, 0] => 4,
            [1, 2, 0, 0, 0] => 3,
            [3, 1, 0, 0, 0] => 2,
            [5, 0, 0, 0, 0] => 1,
            _ => 0
        }
    }

    fn card_to_value1(card: u8) -> u32 {
        match card {
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            b'9' => 8,
            b'T' => 9,
            b'J' => 10,
            b'Q' => 11,
            b'K' => 12,
            b'A' => 13,
            _ => 0
        }
    }

    fn card_to_value2(card: u8) -> u32 {
        match card {
            b'J' => 0,
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            b'9' => 8,
            b'T' => 9,
            b'Q' => 10,
            b'K' => 11,
            b'A' => 12,
            _ => 0
        }
    }
}


fn main() {
    let f = File::open("input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut hands = Vec::<Hand>::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        let mut iter = line.split_whitespace();
        let cards = iter.next().expect("Failed to read card hand");
        let bid: u32 = iter.next().expect("failed to read bid").parse().expect("invalid bid");
        hands.push(Hand::new(cards, bid));
        line.clear();
    }

    hands.sort_by_key(|h| h.value_part1);
    let mut part1: u32 = 0;
    for i in 0..hands.len() {
        part1 += (i as u32 + 1) * hands[i].bid;
    }
    println!("Part1 {}", part1);
    hands.sort_by_key(|h| h.value_part2);
    let mut part2: u32 = 0;
    for i in 0..hands.len() {
        part2 += (i as u32 + 1) * hands[i].bid;
    }
    println!("Part2 {}", part2);
}
