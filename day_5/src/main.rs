use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
    mapping_ranges: Vec<((u64, u64),(u64, u64))>
}

impl Map {
    fn new() -> Self {
        Self { ranges: vec![], mapping_ranges: vec![] }
    }

    fn get_mapped(&self, seed: u64) -> u64 {
        for range in &self.mapping_ranges {
            if seed >= range.1.0 && seed < range.1.1 {
                let offset_from_start = seed - range.1.0;
                return range.0.0 + offset_from_start;
            } 
        }

        seed
    }

    fn load_mapping_ranges(&mut self) {
        self.mapping_ranges = self.ranges.iter().map(|r| (r.source(), r.dest())).collect()
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range)
    }
}

#[derive(Debug)]
struct Range {
    source: u64,
    dest: u64,
    len: u64,
}

impl Range {
    fn new(source: u64, dest: u64, len: u64) -> Self {
        Self { source, dest, len }
    }

    fn source(&self) -> (u64, u64) {
        (self.source, self.source + self.len)
    }

    fn dest(&self) -> (u64, u64) {
        (self.dest, self.dest + self.len)
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let lowest = parse_file(reader);

    println!("Lowest: {}", lowest);

    loop {}
}

fn parse_file(reader: BufReader<File>) -> u64 {
    let lowest: u64 = std::u64::MAX;

    let mut lines = reader.lines()
        .filter_map(|l| match l {
            Ok(line) => {
                if line.is_empty() {
                    return None;
                }
                Some(line)
            },
            Err(_) => None
        });

    let seeds_line: Vec<u64> = lines.next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let maps = parse_map(lines);
    let mut debug = 100000;

    for i in 0.. {
        let source_seed = calc_source_seed(i, &maps);

        for chunk in seeds_line.chunks(2) {
            let range = chunk[0] + chunk[1];

            if source_seed >= chunk[0] && source_seed < range {
                return i;
            }
        }

        // if i > debug {
        //     println!("I is: {}", debug);
        //     debug *= 2;
        // }
    }
    lowest
}

fn calc_source_seed(target: u64, maps: &HashMap<String, Map>) -> u64 {
    let hum = maps.get("humidity-to-location").unwrap().get_mapped(target);
    let temp = maps.get("temperature-to-humidity").unwrap().get_mapped(hum);
    let light = maps.get("light-to-temperature").unwrap().get_mapped(temp);
    let water = maps.get("water-to-light").unwrap().get_mapped(light);
    let fert = maps.get("fertilizer-to-water").unwrap().get_mapped(water);
    let soil = maps.get("soil-to-fertilizer").unwrap().get_mapped(fert);
    let seed = maps.get("seed-to-soil").unwrap().get_mapped(soil);
    
    seed
}

fn parse_map<I>(lines: I) -> HashMap<String, Map>
    where I : Iterator<Item = String> 
{
    let mut maps = init_maps();
    let mut current_key= String::new();

    for line in lines {
        let is_key_line = line.contains("map");

        if is_key_line {
            current_key = line.split(" ").next().unwrap().to_owned();
            continue;
        }

        let mut nums = line
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap());
        
        let dest = nums.next().unwrap();
        let source = nums.next().unwrap();
        let len = nums.next().unwrap();

        maps.get_mut(&current_key).unwrap().add_range(Range::new(source, dest, len));
    }

    for (_, map) in maps.iter_mut() {
        map.load_mapping_ranges();
    }

    maps
}

fn init_maps() -> HashMap<String, Map> {
    HashMap::from([
        (String::from("seed-to-soil"), Map::new()),
        (String::from("soil-to-fertilizer"), Map::new()),
        (String::from("fertilizer-to-water"), Map::new()),
        (String::from("water-to-light"), Map::new()),
        (String::from("light-to-temperature"), Map::new()),
        (String::from("temperature-to-humidity"), Map::new()),
        (String::from("humidity-to-location"), Map::new())
    ])
}