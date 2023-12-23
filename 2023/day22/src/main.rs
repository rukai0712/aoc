use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet, VecDeque}};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}


#[derive(Debug, Default)]
struct Brick {
    origin: Coord,
    size: Coord,
}


#[derive(Default)]
struct SnapshotBuilder {
    bricks: Vec<Brick>,
    space: Coord,
}


struct Snapshot {
    bricks: Vec<Brick>,
    space: Coord,
}

struct Connetions {
    supports: HashMap<usize, HashSet<usize>>,
    relies: HashMap<usize, HashSet<usize>>,
}


impl SnapshotBuilder {

    fn add_brick(&mut self, line: &str) {
        let mut f = line.trim().split(&[',', '~']);
        let start = Coord {
            x: f.next().unwrap().parse().unwrap(),
            y: f.next().unwrap().parse().unwrap(),
            z: f.next().unwrap().parse().unwrap(),
        };
        let end = Coord {
            x: f.next().unwrap().parse().unwrap(),
            y: f.next().unwrap().parse().unwrap(),
            z: f.next().unwrap().parse().unwrap(),
        };
        if end.x >= self.space.x {
            self.space.x = end.x + 1
        }
        if end.y >= self.space.y {
            self.space.y = end.y + 1
        }
        if end.z >= self.space.z {
            self.space.z = end.z + 1
        }
        let size = Coord {
            x: end.x - start.x + 1,
            y: end.y - start.y + 1,
            z: end.z - start.z + 1,
        };
        self.bricks.push(Brick { origin: start, size });
    } 
    
    fn build(mut self) -> Snapshot {
        self.bricks.sort_by(|a, b| a.origin.z.cmp(&b.origin.z));
        Snapshot {
            bricks: self.bricks,
            space: self.space,
        }
    }
}

impl Snapshot {
    
    fn simulate(&self) -> Connetions {
        let mut supports: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(self.bricks.len());
        let mut relies: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(self.bricks.len());
        for idx in 0..self.bricks.len() {
            supports.insert(idx, HashSet::new());
            relies.insert(idx, HashSet::new());
        }

        let mut xy_plane_height: Vec<Vec<usize>> = Vec::new();
        let mut xy_plane_bricks: Vec<Vec<Option<usize>>> = Vec::new();
        for _ in 0..self.space.x {
            xy_plane_height.push([0].repeat(self.space.y));
            xy_plane_bricks.push([None].repeat(self.space.y));
        }
        for idx in 0..self.bricks.len() {
            let brick = self.bricks.get(idx).unwrap();
            let mut buttom = 0;
            for x in brick.origin.x..(brick.origin.x + brick.size.x) {
                for y in brick.origin.y..(brick.origin.y + brick.size.y) {
                    if xy_plane_height[x][y] > buttom {
                        buttom = xy_plane_height[x][y];
                    }
                }
            }
            let mut s_idxes = HashSet::new();
            for x in brick.origin.x..(brick.origin.x + brick.size.x) {
                for y in brick.origin.y..(brick.origin.y + brick.size.y) {
                    if xy_plane_height[x][y] == buttom {
                        if let Some(s_idx) = xy_plane_bricks[x][y] {
                            s_idxes.insert(s_idx);
                        }
                    }
                    xy_plane_height[x][y] = buttom + brick.size.z;
                    xy_plane_bricks[x][y] = Some(idx);
                }
            }
            for s_idx in s_idxes.iter() {
                if !supports.contains_key(s_idx) {
                    supports.insert(*s_idx, HashSet::new());
                }
                supports.get_mut(s_idx).unwrap().insert(idx);
            }
            relies.insert(idx, s_idxes);
        }

        Connetions {
            supports,
            relies
        }
    }

}


impl Connetions {
    fn calculate_disintegrate_bricks(&self) -> Vec<usize> {
        let mut disintegrate_bricks = Vec::new();
        for (&s_idx, s) in self.supports.iter() {
            let mut can_disintegrate = true;
            for rely_idx in s.iter() {
                if self.relies.get(rely_idx).unwrap().len() == 1 {
                    can_disintegrate = false;
                    break;
                }
            }
            if can_disintegrate {
                disintegrate_bricks.push(s_idx);
            }
        }
        disintegrate_bricks
    }

    fn calculate_fall_bricks(&self, dis_brick: &usize) -> usize {
        let mut removed_bricks = HashSet::new();
        removed_bricks.insert(*dis_brick);
        let mut bfs = VecDeque::new();
        bfs.push_back(*dis_brick);
        while let Some(s_idx) = bfs.pop_front() {
            for &rely_idx in self.supports.get(&s_idx).unwrap().iter() {
                if removed_bricks.contains(&rely_idx) {
                    continue;
                }
                if removed_bricks.is_superset(self.relies.get(&rely_idx).unwrap()) {
                    removed_bricks.insert(rely_idx);
                    bfs.push_back(rely_idx);
                }
            }
        }
        removed_bricks.len() - 1
    }
}


fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut builder = SnapshotBuilder::default();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        builder.add_brick(&line);
        line.clear();
    }
    let snapshot = builder.build();
    println!("{:?}", snapshot.space);
    let connection = snapshot.simulate();
    let disintegrate_bricks = connection.calculate_disintegrate_bricks();
    println!("Part1 {}", disintegrate_bricks.len());
    let mut part2 = 0;
    for idx in 0..snapshot.bricks.len() {
        part2 += connection.calculate_fall_bricks(&idx);
    }
    println!("Part2 {}", part2);
}
