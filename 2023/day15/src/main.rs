use std::{fs::File, io::Read, collections::{LinkedList, HashMap}, default};

struct LensSlot {
    label: String,
    count: usize,
}

impl LensSlot {
    fn new(text: &str) -> Self {
        let mut count = 0;
        let parts: Vec<&str> = text.split(&['=', '-']).collect();
        let label = parts[0].to_string();
        if !text.ends_with('-') {
            count = parts[1].parse().unwrap();
        }
        Self { label, count }
    }

    fn hash(&self) -> usize {
        let mut hash = 0;
        for c in self.label.as_bytes() {
            hash = (hash + (*c) as u32) * 17 % 256
        }
        hash as usize
    }
}

struct Game {
    boxes: [LinkedList<LensSlot>; 256]
}

impl Game {

    fn new() -> Self {
        let boxes = [(); 256].map(|_| LinkedList::new());
        Self { boxes }
    }

    fn slot_ops(&mut self, lens_slot: LensSlot) {
        let b_idx = lens_slot.hash();
        let b = self.boxes.get_mut(b_idx).unwrap();
        let mut s_idx = None;
        for (slot, idx) in b.iter_mut().zip(0..) {
            if slot.label == lens_slot.label {
                s_idx.replace(idx);
                break;
            }
        }
        if let Some(s_idx) = s_idx {
            let mut splited = b.split_off(s_idx);
            let _ = splited.pop_front().unwrap();
            if lens_slot.count > 0 {
                b.push_back(lens_slot);
            }
            b.append(&mut splited);
        } else {
            if lens_slot.count > 0 {
                b.push_back(lens_slot);
            }
        }
    }

    fn sum(&self) -> usize {
        let mut sum = 0; 
        for (b, b_score) in self.boxes.iter().zip(1..) {
            for (s, s_score) in b.iter().zip(1..) {
                sum += b_score * s_score * s.count;
            }
        }
        sum
    }
    
}



fn main() {
    let mut f = File::open("./input").expect("Failed to open input file.");
    let mut text = String::new();

    f.read_to_string(&mut text).expect("Faild to read input");

    let mut part1 = 0;
    let mut game = Game::new();
    for len in text.trim_end().split(',') {
        let mut hash = 0;
        for c in len.as_bytes() {
            hash = (hash + (*c) as u32) * 17 % 256
        }
        part1 += hash;
        let lens_slot = LensSlot::new(len);
        game.slot_ops(lens_slot);
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", game.sum());
}
