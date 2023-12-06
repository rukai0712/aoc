use std::{
    char,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader}, u64::MAX,
};

struct MapElement {
    dst: u64,
    src: u64,
    length: u64,
}

struct GardenMapper {
    mappings: Vec<MapElement>,
}

impl GardenMapper {
    fn build(mut input: Vec<MapElement>) -> Self {
        // calculate range gaps in destination
        let mut dst_gaps = VecDeque::<(u64, u64)>::with_capacity(input.len());
        input.sort_by(|a, b| a.dst.partial_cmp(&b.dst).unwrap());
        let mut gap_start = 0;
        for element in &input {
            if element.dst < gap_start {
                panic!("Invalid gap, overlap exist");
            }
            let gap = (gap_start, element.dst - gap_start);
            gap_start = element.dst + element.length;
            if gap.1 > 0 {
                dst_gaps.push_back(gap);
            }
        }
        let dst_end = gap_start;

        // calcuate range gaps in source
        let mut src_gaps = VecDeque::<(u64, u64)>::with_capacity(input.len());
        input.sort_by(|a, b| a.src.partial_cmp(&b.src).unwrap());
        let mut gap_start = 0;
        for element in &input {
            if element.src < gap_start {
                panic!("Invalid gap, overlap exist");
            }
            let gap = (gap_start, element.src - gap_start);
            gap_start = element.src + element.length;
            if gap.1 > 0 {
                src_gaps.push_back(gap);
            }
        }
        let src_end = gap_start;

        let mut mappings = Vec::<MapElement>::new();
        // create new elements based on gaps
        let mut s_gap = src_gaps.pop_front();
        let mut d_gap = dst_gaps.pop_front();
        while s_gap.is_some() && d_gap.is_some() {
            let (mut s_start, mut s_len) = s_gap.take().unwrap();
            let (mut d_start, mut d_len) = d_gap.take().unwrap();
            assert!(s_len > 0 && d_len > 0);
            let e = MapElement {
                dst: d_start,
                src: s_start,
                length: s_len.min(d_len),
            };
            s_start += e.length;
            s_len -= e.length;
            d_start += e.length;
            d_len -= e.length;
            if s_len > 0 {
                s_gap.replace((s_start, s_len));
            } else {
                s_gap = src_gaps.pop_front();
            }
            if d_len > 0 {
                d_gap.replace((d_start, d_len));
            } else {
                d_gap = dst_gaps.pop_front();
            }
            mappings.push(e);
        }
        assert!(s_gap.is_none() || d_gap.is_none());
        let mut d_start = dst_end;
        while s_gap.is_some() {
            let e = MapElement {
                src: s_gap.unwrap().0,
                dst: d_start,
                length: s_gap.unwrap().1,
            };
            d_start += e.length;
            s_gap = src_gaps.pop_front();
            mappings.push(e);
        }

        let mut s_start = src_end;
        while d_gap.is_some() {
            let e = MapElement {
                src: s_start,
                dst: d_gap.unwrap().0,
                length: d_gap.unwrap().1,
            };
            s_start += e.length;
            d_gap = dst_gaps.pop_front();
            mappings.push(e);
        }

        println!("SRC End {}; DST End {}", src_end, dst_end);
        mappings.extend(input);
        mappings.sort_by(|a, b| a.src.partial_cmp(&b.src).unwrap());
        Self { mappings }

    }

    fn map_source(&self, src: u64) -> u64 {
        let r = self.mappings.iter().find(|r| src >= r.src && src < r.src + r.length);
        if let Some(r) = r {
            r.dst + (src-r.src)
        } else {
            src
        }
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let seeds = read_seeds(&mut reader);

    let (header, mapper) = read_map(&mut reader);
    if header != "seed-to-soil" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let soils: Vec<u64> = seeds.into_iter().map(|e| mapper.map_source(e)).collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "soil-to-fertilizer" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let fertilizers: Vec<u64> = soils.into_iter().map(|e| mapper.map_source(e)).collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "fertilizer-to-water" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let waters: Vec<u64> = fertilizers
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "water-to-light" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let lights: Vec<u64> = waters.into_iter().map(|e| mapper.map_source(e)).collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "light-to-temperature" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let temperatures: Vec<u64> = lights.into_iter().map(|e| mapper.map_source(e)).collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "temperature-to-humidity" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let humidities: Vec<u64> = temperatures
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();

    let (header, mapper) = read_map(&mut reader);
    if header != "humidity-to-location" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let locations: Vec<u64> = humidities
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();

    let result = locations.iter().fold(u64::MAX, |cur, e| {
        if *e < cur {
            *e
        } else {
            cur
        }
    });

    println!("{}", result)
}

fn get_numbers(text: &str) -> Vec<u64> {
    text.split(&char::is_whitespace)
        .collect::<Vec<&str>>()
        .into_iter()
        .filter_map(|e| e.parse().ok())
        .collect()
}

fn read_seeds(reader: &mut BufReader<File>) -> Vec<u64> {
    let prefix = "seeds: ";
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read seeds");
    let mut text = line.trim();
    if text.len() == 0 {
        reader.read_line(&mut line).expect("Failed to read seeds");
        text = line.trim();
    }
    if !text.starts_with(prefix) {
        panic!("Invalid seeds line")
    }
    let text = &text[prefix.len()..];
    get_numbers(text)
}

fn read_map(reader: &mut BufReader<File>) -> (String, Vec<MapElement>) {
    let mut map_inputs = Vec::new();
    let mut line = String::new();
    // read prefix
    let suffix = " map:";
    reader.read_line(&mut line).expect("Failed to read seeds");
    let mut header = line.trim();
    if header.len() == 0 {
        reader.read_line(&mut line).expect("Failed to read seeds");
        header = line.trim();
    }
    if !header.ends_with(suffix) {
        panic!("Invalid map header")
    }
    let header = header[0..header.len() - suffix.len()].to_string();
    println!("Read map '{}' ", header);

    line.clear();
    while let Ok(_) = reader.read_line(&mut line) {
        let text = line.trim();
        if text.len() == 0 {
            break;
        }
        let numbers = get_numbers(text);
        if numbers.len() != 3 {
            panic!("Invalid map line, expect 3 numbers");
        }
        map_inputs.push(MapElement {
            dst: numbers[0],
            src: numbers[1],
            length: numbers[2],
        });
        line.clear();
    }
    (header, map_inputs)
}
