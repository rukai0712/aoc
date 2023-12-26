use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul, Sub}, str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Coord<E> {
    x: E,
    y: E,
    z: E,
}

#[derive(Debug, Clone, Copy)]
struct Path<E> {
    start: Coord<E>,
    sp: Coord<E>,
}

struct SearchAreaXY {
    x_range: (i64, i64),
    y_range: (i64, i64),
}

struct Segment {
    start: Coord<i64>,
    end: Coord<i64>,
    sp: Coord<i64>,
}

impl Path<i64> {
    fn from(line: &str) -> Self {
        let mut f = line.trim().split('@');
        let start: Vec<i64> = f
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let speed: Vec<i64> = f
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        Path {
            start: Coord {
                x: start[0],
                y: start[1],
                z: start[2],
            },
            sp: Coord {
                x: speed[0],
                y: speed[1],
                z: speed[2],
            },
        }
    }
}

impl<E: Sub<Output = E>> Sub for Coord<E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<E> Coord<E>
where
    E: Clone + Copy + Sub<Output = E> + Add<Output = E> + Mul<Output = E>,
{
    fn dot_product(&self, other: &Coord<E>) -> E {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(&self, other: &Coord<E>) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Coord<f64> {
    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn norm(&self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl From<Coord<i64>> for Coord<f64> {
    fn from(value: Coord<i64>) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            z: value.z as f64,
        }
    }
}

impl From<Path<i64>> for Path<f64> {
    fn from(value: Path<i64>) -> Self {
        Self { start: value.start.into(), sp: value.sp.into() }
    }
}

impl SearchAreaXY {
    fn convert_path_to_segment(&self, path: &Path<i64>) -> Option<Segment> {
        let mut cross_points: Vec<Coord<i64>> = Vec::new();
        if path.sp.x != 0 {
            let y: i64 = path.sp.y * (self.x_range.0 - path.start.x) / path.sp.x + path.start.y;
            if y >= self.y_range.0 && y <= self.y_range.1 {
                cross_points.push(Coord {
                    x: self.x_range.0,
                    y,
                    z: path.start.z,
                });
            }
            let y: i64 = path.sp.y * (self.x_range.1 - path.start.x) / path.sp.x + path.start.y;
            if y >= self.y_range.0 && y <= self.y_range.1 {
                cross_points.push(Coord {
                    x: self.x_range.1,
                    y,
                    z: path.start.z,
                });
            }
        }
        if path.sp.y != 0 {
            let x = path.sp.x * (self.y_range.0 - path.start.y) / path.sp.y + path.start.x;
            if x >= self.x_range.0 && x <= self.x_range.1 {
                cross_points.push(Coord {
                    x,
                    y: self.y_range.0,
                    z: path.start.z,
                });
            }
            let x = path.sp.x * (self.y_range.1 - path.start.y) / path.sp.y + path.start.x;
            if x >= self.x_range.0 && x <= self.x_range.1 {
                cross_points.push(Coord {
                    x,
                    y: self.y_range.1,
                    z: path.start.z,
                });
            }
        }
        if cross_points.len() == 0 {
            return None;
        }
        let direction = path.sp;
        assert!(cross_points.len() == 2);

        let points: Vec<Coord<i64>> = cross_points
            .into_iter()
            .filter(|p| (*p - path.start).dot_product(&direction) >= 0)
            .collect();
        if points.len() == 0 {
            None
        } else if points.len() == 1 {
            Some(Segment {
                start: path.start,
                end: points[0],
                sp: path.sp,
            })
        } else {
            assert!(points.len() == 2);
            let v0: Coord<f64> = (points[0] - path.start).into();
            let v1: Coord<f64> = (points[1] - path.start).into();
            if v1.len() >= v0.len() {
                Some(Segment {
                    start: points[0],
                    end: points[1],
                    sp: path.sp,
                })
            } else {
                Some(Segment {
                    start: points[1],
                    end: points[0],
                    sp: path.sp,
                })
            }
        }
    }
}

impl Segment {
    fn intersect_xy(&self, other: &Segment) -> bool {
        if self.sp.cross_product(&other.sp).z == 0
            && self.start != other.start
            && (self.start - other.start).cross_product(&other.sp).z == 0
        {
            // in the same line, but start not the same
            return false;
        }
        let v1 = other.start - self.start;
        let v2 = other.end - self.start;
        if (self.sp.cross_product(&v1).z > 0) == (self.sp.cross_product(&v2).z > 0) {
            // on the same side
            return false;
        }
        let v1 = self.start - other.start;
        let v2 = self.end - other.start;
        if (other.sp.cross_product(&v1).z > 0) == (other.sp.cross_product(&v2).z > 0) {
            // on the same side
            return false;
        }
        return true;
    }
}

fn calculate_intersections(segments: &Vec<Segment>) -> usize {
    let mut counts = 0;
    for i in 0..segments.len() {
        for j in i + 1..segments.len() {
            if segments[i].intersect_xy(&segments[j]) {
                counts += 1;
            }
        }
    }
    counts
}

// constantly move start points along the path by the follow steps:
// 1. minimize the distance from points[0] to the line connected from poins[2]-points[3]
// 2. minimize the distance from points[1] to the line connected from poins[2]-points[3]
// 3. minimize the distance from points[2] to the line connected from poins[0]-points[1]
// 4. minimize the distance from points[3] to the line connected from poins[0]-points[1]
// and repeat the following steps until the space collapse to a single line.
fn search_in_paths(paths: [&Path<i64>;4]) -> Vec<(f64, Coord<f64>)> {
    let mut points: [Path<f64>;4] = [(*paths[0]).into(), (*paths[1]).into(), (*paths[2]).into(), (*paths[3]).into()];
    let mut converged = 0;
    while converged < 4 {
        converged = 0;
        for i in 0..4 {
            let mov_p: Coord<f64> = points[i].start;
            let mov_sp: Coord<f64> = points[i].sp;
            let s_p: Coord<f64> = points[(2+i)%4].start;    // 0=>2 1=>3 | 2=>0 3=>1
            let a_p: Coord<f64> = mov_p - s_p;
            let b_p: Coord<f64> = points[3-i].start;        // 0=>3 1=>2 | 2=>1 3=>0
            let b_p: Coord<f64> = (b_p - s_p).norm();
            let t = b_p.dot_product(&a_p);
            let e_p = Coord::<f64> {
                x: s_p.x + t * b_p.x,
                y: s_p.y + t * b_p.y,
                z: s_p.z + t * b_p.z,
            };
            let c_p = e_p - mov_p;
            let mov_sp = mov_sp.norm();
            let t2 = mov_sp.dot_product(&c_p);
            let mov = Coord::<f64> {
                x: t2 * mov_sp.x,
                y: t2 * mov_sp.y,
                z: t2 * mov_sp.z,
            };
            println!("Moved: {}", mov.len());
            if mov.x.abs() < 0.05 && mov.y.abs() < 0.05 && mov.z.abs() < 0.05 {
                converged += 1;
            } 
            points[i].start = Coord::<f64> {
                x: mov_p.x + mov.x,
                y: mov_p.y + mov.y,
                z: mov_p.z + mov.z,
            };
        }
    }
    let mut cross_points = Vec::new();
    for i in 0..4 {
        let tx = (points[i].start.x - paths[i].start.x as f64) / points[i].sp.x ;
        let ty = (points[i].start.y - paths[i].start.y as f64) / points[i].sp.y;
        let tz = (points[i].start.z - paths[i].start.z as f64) / points[i].sp.z;
        let t = tx / 3.0 + ty / 3.0 + tz / 3.0;
        println!("t={}, tx={}, ty={}, tz={}", t, tx, ty, tz);
        cross_points.push((t, points[i].start));
    }
    println!("{:?}", cross_points);
    cross_points
}

fn main() {
    let f = File::open("./input").expect("Failed to read input file");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut pathes = Vec::new();
    while let Ok(size) = reader.read_line(&mut line) {
        if size == 0 {
            break;
        }
        pathes.push(Path::<i64>::from(&line));
        line.clear();
    }
    let area = SearchAreaXY {
        x_range: (200000000000000, 400000000000000),
        y_range: (200000000000000, 400000000000000),
    };

    let segments: Vec<Segment> = pathes
        .iter()
        .filter_map(|v| area.convert_path_to_segment(v))
        .collect();
    println!("{}", segments.len());
    let part1 = calculate_intersections(&segments);
    println!("Part1 {}", part1);

    // select any four unparallel paths
    let mut cross_points = search_in_paths([&pathes[0], &pathes[1], &pathes[3], &pathes[4]]);
    // sort by the time
    cross_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let delta = cross_points[1].1 - cross_points[0].1;
    let delta_time = cross_points[1].0 - cross_points[0].0;
    let start = Coord::<f64> {
        x: cross_points[0].1.x - delta.x / delta_time * cross_points[0].0,
        y: cross_points[0].1.y - delta.y / delta_time * cross_points[0].0,
        z: cross_points[0].1.z - delta.z / delta_time * cross_points[0].0,
    };
    println!("Part2 {:.3}", start.x+start.y+start.z);    
}
