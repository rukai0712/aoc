use std::{fs::File, io::Read, collections::{HashSet, VecDeque}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    // -
    Horizantal,
    // |
    Vertical,
    // /
    Slash,
    // \
    BackSlash,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LightDirection {
    Left,
    Right,
    Top,
    Bottom,
}

struct Game {
    map: Vec<Vec<Option<Mirror>>>,
    size: (usize, usize)
}

impl Game {
    fn new() -> Self {
        Self { map: Vec::new(), size: (0, 0) }
    }

    fn add_input(&mut self, input: &str) {
        let input = input.as_bytes();
        let mut new_l = Vec::new();
        for c in input {
            new_l.push(match *c {
                b'|' => Some(Mirror::Vertical),
                b'-' => Some(Mirror::Horizantal),
                b'/' => Some(Mirror::Slash),
                b'\\' => Some(Mirror::BackSlash),
                b'.' => None,
                _ => unreachable!()
            })
        }
        if self.size.1 == 0 {
            self.size.1 = new_l.len();
        } else {
            assert_eq!(new_l.len(), self.size.1)
        }
        self.map.push(new_l);
        self.size.0 += 1;
    }

    fn part1(&self) -> usize {
        let mut reached: HashSet<(usize, usize)> = HashSet::new();
        let mut reached_status: HashSet<(usize, usize, LightDirection)> = HashSet::new();
        let mut bfs = VecDeque::new();
        bfs.push_back((0_usize, 0_usize, LightDirection::Right));
        while let Some(state) = bfs.pop_front() {
            if reached_status.contains(&state) {
                continue;
            }
            let row = state.0;
            let col = state.1;
            let mirror = &self.map[row][col];
            let direction = state.2;
            reached_status.insert(state);
            reached.insert((row, col));
            let directions = match direction {
                LightDirection::Top => {
                    match *mirror {
                        Some(Mirror::Slash) => vec![LightDirection::Right],
                        Some(Mirror::BackSlash) => vec![LightDirection::Left],
                        Some(Mirror::Horizantal) => vec![LightDirection::Left, LightDirection::Right],
                        _ => vec![LightDirection::Top],
                    }
                },
                LightDirection::Bottom => {
                    match *mirror {
                        Some(Mirror::Slash) => vec![LightDirection::Left],
                        Some(Mirror::BackSlash) => vec![LightDirection::Right],
                        Some(Mirror::Horizantal) => vec![LightDirection::Left, LightDirection::Right],
                        _ => vec![LightDirection::Bottom],
                    }
                },
                LightDirection::Left => {
                    match *mirror {
                        Some(Mirror::Slash) => vec![LightDirection::Bottom],
                        Some(Mirror::BackSlash) => vec![LightDirection::Top],
                        Some(Mirror::Vertical) => vec![LightDirection::Top, LightDirection::Bottom],
                        _ => vec![LightDirection::Left],
                    }
                },
                LightDirection::Right => {
                    match *mirror {
                        Some(Mirror::Slash) => vec![LightDirection::Top],
                        Some(Mirror::BackSlash) => vec![LightDirection::Bottom],
                        Some(Mirror::Vertical) => vec![LightDirection::Top, LightDirection::Bottom],
                        _ => vec![LightDirection::Right],
                    }
                },
            };
            let next_status = directions.iter().filter_map(|d| {
                match d {
                    LightDirection::Top => {
                        if row > 0 {
                            Some((row-1, col, *d))
                        } else {
                            None
                        }
                    },
                    LightDirection::Bottom => {
                        if row + 1 < self.size.0 {
                            Some((row+1, col, *d))
                        } else {
                            None
                        }
                    },
                    LightDirection::Left => {
                        if col > 0 {
                            Some((row, col-1, *d))
                        } else {
                            None
                        }
                    },
                    LightDirection::Right => {
                        if col + 1 < self.size.1 {
                            Some((row, col+1, *d))
                        } else {
                            None
                        }
                    }
                }
            });
            bfs.extend(next_status)
        }

        reached.len()
    }
}

fn main() {
    let mut f = File::open("./input").expect("Faile to open input file.");
    let mut input = String::new();
    let _ = f.read_to_string(&mut input).expect("Failed to read input");
    let mut game = Game::new();
    for line in input.trim().split_whitespace() {
        game.add_input(line);
    }
    println!("Part1: {}", game.part1());

}
