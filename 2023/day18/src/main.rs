use std::{collections::HashSet, fs::File, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Segment {
    start: (usize, usize),
    direction: Direction,
    len: usize,
}

struct Map {
    segments: Vec<Segment>,
}

impl Map {
    fn new(digs: Vec<(Direction, usize)>) -> Self {
        let mut segments: Vec<((i32, i32), Direction, usize)> = Vec::new();
        let mut minial_row = 0;
        let mut minial_col = 0;
        let mut start = (0, 0);
        let mut cut = false;
        for (direction, len) in digs {
            let (next, next_cut) = match direction {
                Direction::Up => ((start.0 - len as i32, start.1), false),
                Direction::Down => ((start.0 + len as i32, start.1), false),
                Direction::Left => ((start.0, start.1 - len as i32), true),
                Direction::Right => ((start.0, start.1 + len as i32), true),
            };
            if cut && !next_cut {
                let last = segments.last_mut().unwrap();
                last.2 += 1;
                segments.push(match direction {
                    Direction::Up => ((start.0 - 1, start.1), Direction::Up, len - 1),
                    Direction::Down => ((start.0 + 1, start.1), Direction::Down, len - 1),
                    _ => unreachable!(),
                })
            } else {
                segments.push((start, direction, len));
            }
            start = next;
            cut = next_cut;
            if minial_row > start.0 {
                minial_row = start.0;
            }
            if minial_col > start.1 {
                minial_col = start.1;
            }
        }
        assert_eq!(start, (0, 0));
        if cut {
            let first = segments.first_mut().unwrap();
            let extend_last = match first.1 {
                Direction::Up => {
                    *first = ((first.0 .0 - 1, first.0 .1), Direction::Up, first.2 - 1);
                    true
                }
                Direction::Down => {
                    *first = ((first.0 .0 + 1, first.0 .1), Direction::Down, first.2 - 1);
                    true
                }
                _ => false,
            };
            if extend_last {
                let last = segments.last_mut().unwrap();
                last.2 += 1;
            }
        }

        let mut segments: Vec<Segment> = segments
            .into_iter()
            .map(|(start, direction, len)| Segment {
                start: (
                    (start.0 - minial_row) as usize,
                    (start.1 - minial_col) as usize,
                ),
                direction,
                len,
            })
            .collect();

        segments.sort_by(|a: &Segment, b| a.start.1.cmp(&b.start.1));

        Self { segments }
    }

    fn calculate_area(&mut self) -> usize {
        let mut sum = self.segments.iter().fold(0, |cur, seg| cur + seg.len);
        let mut rows: Vec<usize> = self
            .segments
            .iter()
            .filter_map(|s| match s.direction {
                Direction::Up | Direction::Down => None,
                Direction::Left | Direction::Right => Some(s.start.0),
            })
            .collect::<HashSet<usize>>()
            .into_iter()
            .collect();
        rows.sort();

        for (&row_start, &row_end) in rows.iter().zip(rows[1..].iter()) {
            let mut filter = self.segments.iter().filter(|seg| match seg.direction {
                Direction::Up => seg.start.0 >= row_start && seg.start.0 - seg.len <= row_start,
                Direction::Down => seg.start.0 <= row_start + 1 && seg.start.0 + seg.len > row_start,
                Direction::Left => false,
                Direction::Right => false,
            });
            let mut square_area = Vec::new();
            let mut col_start = None;
            while let Some(seg) = filter.next() {
                if let Some(col_start) = col_start.take() {
                    square_area.push((col_start, seg.start.1 - col_start));
                } else {
                    col_start.replace(seg.start.1 + 1);
                }
            }
            assert!(col_start.is_none());

            sum = square_area
                .iter()
                .fold(sum, |cur, (_, len)| cur + len * (row_end - row_start - 1));

            let mut filter = self.segments.iter().filter(|seg| match seg.direction {
                Direction::Up => seg.start.0 >= row_start && seg.start.0 - seg.len < row_start,
                Direction::Down => seg.start.0 <= row_start && seg.start.0 + seg.len > row_start,
                Direction::Left | Direction::Right => seg.start.0 == row_start,
            });
            let mut row_area = Vec::new();
            let mut col_start = None;
            let mut sq_idx = 0;
            while let Some(seg) = filter.next() {
                if sq_idx >= square_area.len() {
                    break;
                }
                if let Some(col_start) = col_start.take() {
                    let len = match seg.direction {
                        Direction::Up | Direction::Down | Direction::Right => {
                            seg.start.1 - col_start
                        }
                        Direction::Left => seg.start.1 + 1 - seg.len - col_start,
                    };
                    if len > 0 {
                        while sq_idx < square_area.len() {
                            let (s, l) = square_area[sq_idx];
                            if s > col_start {
                                assert!(s >= col_start + len);
                                break;
                            } else if s + l > col_start {
                                assert!(s + l >= col_start + len);
                                row_area.push((col_start, len));
                                break;
                            }
                            sq_idx += 1;
                        }
                    }
                }
                col_start.replace(match seg.direction {
                    Direction::Up | Direction::Down | Direction::Left => seg.start.1 + 1,
                    Direction::Right => seg.start.1 + seg.len,
                });
            }
            sum = row_area.iter().fold(sum, |cur, (_, len)| cur + len);
        }

        sum
    }
}

fn main() {
    let mut f = File::open("./input").expect("Failed to open input file.");
    let mut text = String::new();
    let _ = f
        .read_to_string(&mut text)
        .expect("Failed to read input file.");
    let mut digs_part1 = Vec::new();
    let mut digs_part2 = Vec::new();
    for line in text.trim().split("\n") {
        let mut line_reader = line.split_ascii_whitespace();
        let direction = line_reader.next().unwrap();
        let len: usize = line_reader.next().unwrap().parse().unwrap();
        let dig = match direction {
            "U" => (Direction::Up, len),
            "D" => (Direction::Down, len),
            "L" => (Direction::Left, len),
            "R" => (Direction::Right, len),
            _ => unreachable!(),
        };
        digs_part1.push(dig);
        let hex = line_reader.next().unwrap();
        let len = usize::from_str_radix(&hex[2..7], 16).unwrap();
        let dig = match &hex[7..8] {
            "0" => (Direction::Right, len),
            "1" => (Direction::Down, len),
            "2" => (Direction::Left, len),
            "3" => (Direction::Up, len), 
            _ => unreachable!(),
        };
        digs_part2.push(dig);
    }
    let mut map1 = Map::new(digs_part1);
    let part1 = map1.calculate_area();
    println!("Part1 {}", part1);
    let mut map2 = Map::new(digs_part2);
    let part2 = map2.calculate_area();
    println!("Part2 {}", part2);
}
