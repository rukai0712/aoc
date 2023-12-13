use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeGrid {
    // S
    Start,
    // .
    Ground,
    // |
    NorthSouth,
    // -
    EastWest,
    // L
    NorthEast,
    // J
    NorthWest,
    // 7
    SouthWest,
    // F
    SouthEast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsePipeGridError;

impl Display for ParsePipeGridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse PipeGrid.")
    }
}

impl Error for ParsePipeGridError {}

impl TryFrom<char> for PipeGrid {
    type Error = ParsePipeGridError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            _ => Err(ParsePipeGridError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct YX(usize, usize);

struct Game {
    lines: Vec<Vec<PipeGrid>>,
    start: Option<YX>,
    size: Option<YX>,
    profile: Option<Vec<Vec<PipeGrid>>>,
}

impl Game {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
            start: None,
            size: None,
            profile: None,
        }
    }

    fn insert_new_line(&mut self, line: &str) {
        let grids: Vec<PipeGrid> = line
            .trim()
            .chars()
            .map(|c| c.try_into().expect("Failed to parse grid"))
            .collect();
        for x in 0..grids.len() {
            if grids[x] == PipeGrid::Start {
                let y = self.lines.len();
                if self.start.replace(YX(y, x)).is_some() {
                    panic!("multiple start grid founds");
                }
            }
        }
        let mut size = self.size.unwrap_or(YX(0, grids.len()));
        if size.1 != grids.len() {
            panic!("Invalid grids length")
        }
        self.lines.push(grids);
        size.0 += 1;
        self.size.replace(size);
    }

    fn play_part1(&mut self) -> usize {
        if self.start.is_none() {
            panic!()
        }
        let start = self.start.unwrap();
        let mut max_distance = 0;
        if let Some(north) = self.to_north(start) {
            if let Some(lp) = self.try_find_loop(start, north) {
                if max_distance < lp.len() / 2 {
                    max_distance = lp.len() / 2;
                    self.draw_profile(&lp);
                }
            }
        }
        if let Some(south) = self.to_south(start) {
            if let Some(lp) = self.try_find_loop(start, south) {
                if max_distance < lp.len() / 2 {
                    max_distance = lp.len() / 2;
                    self.draw_profile(&lp);
                }
            }
        }
        if let Some(west) = self.to_west(start) {
            if let Some(lp) = self.try_find_loop(start, west) {
                if max_distance < lp.len() / 2 {
                    max_distance = lp.len() / 2;
                    self.draw_profile(&lp);
                }
            }
        }
        if let Some(east) = self.to_east(start) {
            if let Some(lp) = self.try_find_loop(start, east) {
                if max_distance < lp.len() / 2 {
                    max_distance = lp.len() / 2;
                    self.draw_profile(&lp);
                }
            }
        }
        max_distance
    }

    fn draw_profile(&mut self, lp: &Vec<YX>) {
        let mut profile = Vec::with_capacity(self.size.unwrap().0);
        for _ in 0..self.size.unwrap().0 {
            let mut line = Vec::with_capacity(self.size.unwrap().1);
            for _ in 0..self.size.unwrap().1 {
                line.push(PipeGrid::Ground);
            }
            profile.push(line);
        }

        for grid in lp.iter() {
            profile[grid.0][grid.1] = self.lines[grid.0][grid.1].clone();
        }
        let start = self.start.unwrap();
        let mut linked = [false, false, false, false];  // north, south, west, east
        if let Some(neb) = self.to_north(start) {
            match profile[neb.0][neb.1] {
                PipeGrid::SouthEast | PipeGrid::NorthSouth | PipeGrid::SouthWest => {
                    linked[0] = true;
                },
                _ => {}
            }
        }
        if let Some(neb) = self.to_south(start) {
            match profile[neb.0][neb.1] {
                PipeGrid::NorthEast | PipeGrid::NorthSouth | PipeGrid::NorthWest => {
                    linked[1] = true;
                },
                _ => {}
            }
        }
        if let Some(neb) = self.to_west(start) {
            match profile[neb.0][neb.1] {
                PipeGrid::SouthEast | PipeGrid::NorthEast | PipeGrid::EastWest => {
                    linked[2] = true;
                },
                _ => {}
            }
        }
        if let Some(neb) = self.to_east(start) {
            match profile[neb.0][neb.1] {
                PipeGrid::SouthWest | PipeGrid::NorthWest | PipeGrid::EastWest => {
                    linked[3] = true;
                },
                _ => {}
            }
        }
        profile[start.0][start.1] = match linked {
            [true, true, false, false] => PipeGrid::NorthSouth,
            [true, false, true, false] => PipeGrid::NorthWest,
            [true, false, false, true] => PipeGrid::NorthEast,
            [false, true, true, false] => PipeGrid::SouthWest,
            [false, true, false, true] => PipeGrid::SouthEast,
            [false, false, true, true] => PipeGrid::EastWest,
            _ => panic!("Invalid start grid.")
        };
        self.profile.replace(profile);
    }

    fn try_find_loop(&self, start: YX, cur: YX) -> Option<Vec<YX>> {
        let mut lp = vec![start];
        let mut cur = cur;
        let mut pre = start;
        while let Some(next) = self.get_next_grid(pre, cur) {
            lp.push(cur);
            pre = cur;
            cur = next;
            if cur == start {
                break;
            }
        }
        if cur == start {
            Some(lp)
        } else {
            None
        }
    }

    fn play_part2(&self) -> usize {
        let mut count = 0;
        for line in self.profile.as_ref().unwrap().iter() {
            let mut in_loop = false;
            let mut pre = PipeGrid::Ground;
            for grid in line.iter() {
                match grid {
                    PipeGrid::EastWest => continue,
                    PipeGrid::NorthSouth | PipeGrid::NorthEast | PipeGrid::SouthEast => {
                        in_loop = !in_loop;
                    }
                    PipeGrid::NorthWest => {
                        if pre == PipeGrid::NorthEast {
                            in_loop = !in_loop;
                        }
                    }
                    PipeGrid::SouthWest => {
                        if pre == PipeGrid::SouthEast {
                            in_loop = !in_loop;
                        }
                    }
                    _ => {
                        if in_loop {
                            count += 1;
                        }
                    }
                }
                pre = *grid;
            }
        }
        count
    }

    fn get_next_grid(&self, pre: YX, cur: YX) -> Option<YX> {
        let cur_grid = self.lines.get(cur.0)?.get(cur.1)?;
        match cur_grid {
            PipeGrid::Start | PipeGrid::Ground => None,
            PipeGrid::NorthSouth => {
                let north = self.to_north(cur)?;
                let south = self.to_south(cur)?;
                if pre == north {
                    Some(south)
                } else if pre == south {
                    Some(north)
                } else {
                    None
                }
            }
            PipeGrid::EastWest => {
                let east = self.to_east(cur)?;
                let west = self.to_west(cur)?;
                if pre == east {
                    Some(west)
                } else if pre == west {
                    Some(east)
                } else {
                    None
                }
            }
            PipeGrid::NorthEast => {
                let north = self.to_north(cur)?;
                let east = self.to_east(cur)?;
                if pre == north {
                    Some(east)
                } else if pre == east {
                    Some(north)
                } else {
                    None
                }
            }
            PipeGrid::NorthWest => {
                let north = self.to_north(cur)?;
                let west = self.to_west(cur)?;
                if pre == north {
                    Some(west)
                } else if pre == west {
                    Some(north)
                } else {
                    None
                }
            }
            PipeGrid::SouthEast => {
                let south = self.to_south(cur)?;
                let east = self.to_east(cur)?;
                if pre == south {
                    Some(east)
                } else if pre == east {
                    Some(south)
                } else {
                    None
                }
            }
            PipeGrid::SouthWest => {
                let south = self.to_south(cur)?;
                let west = self.to_west(cur)?;
                if pre == south {
                    Some(west)
                } else if pre == west {
                    Some(south)
                } else {
                    None
                }
            }
        }
    }

    fn to_north(&self, from: YX) -> Option<YX> {
        if from.0 > 0 {
            Some(YX(from.0 - 1, from.1))
        } else {
            None
        }
    }

    fn to_south(&self, from: YX) -> Option<YX> {
        if from.0 + 1 < self.size?.0 {
            Some(YX(from.0 + 1, from.1))
        } else {
            None
        }
    }

    fn to_west(&self, from: YX) -> Option<YX> {
        if from.1 > 0 {
            Some(YX(from.0, from.1 - 1))
        } else {
            None
        }
    }

    fn to_east(&self, from: YX) -> Option<YX> {
        if from.1 + 1 < self.size?.1 {
            Some(YX(from.0, from.1 + 1))
        } else {
            None
        }
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to read the input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut game = Game::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        game.insert_new_line(&line);
        line.clear();
    }
    let part1 = game.play_part1();
    println!("Part1 {}", part1);
    let part2 = game.play_part2();
    println!("Part2 {}", part2);
}
