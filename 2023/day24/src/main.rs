use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord<E> {
    x: E,
    y: E,
    z: E,
}

#[derive(Debug, Clone, Copy)]
struct Path {
    start: Coord<i64>,
    sp: Coord<i64>,
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

impl Path {
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

impl SearchAreaXY {
    fn convert_path_to_segment(&self, path: &Path) -> Option<Segment> {
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
        for j in i+1..segments.len() {
            if segments[i].intersect_xy(&segments[j]) {
                counts += 1;
            }
        }
    }
    counts
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
        pathes.push(Path::from(&line));
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
    println!("Part1 {}", part1)
}
