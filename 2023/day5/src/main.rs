use std::{
    char,
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    u64::MAX,
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
        input.sort_by_key(|k| k.dst);
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
        input.sort_by_key(|k| k.src);
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

        mappings.extend(input);
        mappings.sort_by_key(|k| k.src);
        Self { mappings }
    }

    fn map_source(&self, src: u64) -> u64 {
        let r = self
            .mappings
            .iter()
            .find(|r| src >= r.src && src < r.src + r.length);
        if let Some(r) = r {
            r.dst + (src - r.src)
        } else {
            src
        }
    }

    fn map_ranges(&self, ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut ranges = ranges.clone();
        ranges.sort_by_key(|k| k.0);
        let mut src_ranges = VecDeque::from(ranges);
        let mut idx = 0;
        let mut dst_ranges = Vec::new();
        let mut src = src_ranges.pop_front();
        while src.is_some() && idx < self.mappings.len() {
            let mapping = &self.mappings[idx];
            let (src_start, src_len) = src.take().unwrap();
            if mapping.src + mapping.length <= src_start {
                src = Some((src_start, src_len));
                idx += 1;
                continue;
            }
            assert!(src_start >= mapping.src && src_start < mapping.src + mapping.length);
            let len = u64::min(mapping.src + mapping.length, src_start + src_len) - src_start;
            
            let dst_range = (src_start- mapping.src + mapping.dst, len);
            dst_ranges.push(dst_range);
            if len < src_len {
                src = Some((src_start + len, src_len - len));
                idx += 1;
            } else {
                src = src_ranges.pop_front();
            }
        }
        while src.is_some() {
            // out of definition
            dst_ranges.push(src.clone().unwrap());
            src = src_ranges.pop_front();
        }
        dst_ranges
    }
}

fn main() {
    let f = File::open("./input").expect("Failed to open input file.");
    let mut reader = BufReader::new(f);
    let (seeds, seeds_ranges) = read_seeds(&mut reader);
    
    let (header, mapper) = read_map(&mut reader);
    if header != "seed-to-soil" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let soils: Vec<u64> = seeds.into_iter().map(|e| mapper.map_source(e)).collect();
    let soils_ranges = mapper.map_ranges(&seeds_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "soil-to-fertilizer" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let fertilizers: Vec<u64> = soils.into_iter().map(|e| mapper.map_source(e)).collect();
    let fertilizers_ranges = mapper.map_ranges(&soils_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "fertilizer-to-water" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let waters: Vec<u64> = fertilizers
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();
    let waters_ranges = mapper.map_ranges(&fertilizers_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "water-to-light" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let lights: Vec<u64> = waters.into_iter().map(|e| mapper.map_source(e)).collect();
    let lights_ranges = mapper.map_ranges(&waters_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "light-to-temperature" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let temperatures: Vec<u64> = lights.into_iter().map(|e| mapper.map_source(e)).collect();
    let temperatures_ranges = mapper.map_ranges(&lights_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "temperature-to-humidity" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let humidities: Vec<u64> = temperatures
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();
    let humidities_ranges = mapper.map_ranges(&temperatures_ranges);

    let (header, mapper) = read_map(&mut reader);
    if header != "humidity-to-location" {
        panic!("Invalid header")
    }
    let mapper = GardenMapper::build(mapper);
    let locations: Vec<u64> = humidities
        .into_iter()
        .map(|e| mapper.map_source(e))
        .collect();
    let locations_ranges = mapper.map_ranges(&humidities_ranges);

    let result_part1 = locations
        .iter()
        .fold(u64::MAX, |cur, e| if *e < cur { *e } else { cur });

    let result_part2 = locations_ranges.iter().fold(u64::MAX, |cur, e| if e.0 < cur { e.0 } else { cur });

    println!("{}", result_part1);
    println!("{}", result_part2);
}

fn get_numbers(text: &str) -> Vec<u64> {
    text.split(&char::is_whitespace)
        .collect::<Vec<&str>>()
        .into_iter()
        .filter_map(|e| e.parse().ok())
        .collect()
}

fn read_seeds(reader: &mut BufReader<File>) -> (Vec<u64>, Vec<(u64, u64)>) {
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
    let seeds = get_numbers(text);
    let mut seeds_by_range = Vec::<(u64, u64)>::with_capacity(seeds.len()/2);
    for idx in 0..seeds.len()/2 {
        let s_start = seeds[idx*2].clone();
        let s_len = seeds[idx*2+1].clone();
        seeds_by_range.push((s_start, s_len));
    }
    (seeds, seeds_by_range)
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
