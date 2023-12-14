use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(Debug)]
struct Map {
    source: u64,
    dest: u64,
    len: u64,
}

impl Map {
    fn new() -> Self {
        Self { source: 0, dest: 0, len: 0}
    }

    fn source_bound(&self) -> u64 {
        self.source + self.len
    }

    fn dest_bound(&self) -> u64 {
        self.dest + self.len
    }
}

fn map_map(source_maps: &Vec<Map>, dest_maps: &Vec<Map>) -> Vec<Map> {
    let mut new_maps = vec![];

    for source_map in source_maps {
        let source_dest = (source_map.dest, source_map.dest_bound());

        for dest_map in dest_maps {
            let dest_source = (dest_map.source, dest_map.source_bound());

            if let Some(overlap_bounds) = cross_bounds(source_dest, dest_source) {
                if overlap_bounds.0 == overlap_bounds.1 {
                    continue;
                } 

                let mut new_map = Map::new();

                new_map.source = overlap_bounds.0;
                new_map.len = overlap_bounds.1 - overlap_bounds.0;
                new_map.dest = dest_map.dest;

                //println!("{:?}", new_map);

                new_maps.push(new_map);
            }
        }
    }

    new_maps
}

fn cross_bounds(source: (u64, u64), dest: (u64, u64)) -> Option<(u64, u64)> {
    let start = source.0.max(dest.0);
    let end = source.1.min(dest.1);

    if start <= end {
        Some((start, end))
    } else {
        None 
    }
}

fn main() {
    let file = File::open("test.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    parse_file(reader);
}

fn parse_file(reader: BufReader<File>) {
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

    let mut seeds: Vec<u64> = vec![];

    let seeds_line: Vec<u64> = lines.next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    for chunk in seeds_line.chunks(2) {
        let range = chunk[0] + chunk[1];
        for i in chunk[0]..range {
            seeds.push(i);
        }
    }

    let maps = parse_map(lines);

    let seed_fert = map_map(maps.get("seed-to-soil").unwrap(), maps.get("soil-to-fertilizer").unwrap());
    let seed_water = map_map(&seed_fert, maps.get("fertilizer-to-water").unwrap());
    let seed_light = map_map(&seed_water, maps.get("water-to-light").unwrap());
    let seed_temp = map_map(&seed_light, maps.get("light-to-temperature").unwrap());
    let seed_hum = map_map(&seed_temp, maps.get("temperature-to-humidity").unwrap());
    let mut seed_loc = map_map(&seed_hum, maps.get("humidity-to-location").unwrap());
   
    println!("Seed to fert {:?}", seed_fert);
    println!("Seed to water {:?}", seed_water);
    println!("Seed to light {:?}", seed_light);


    //let mut target = create_target(seeds, &mut seed_loc);
    //target.sort();

   // println!("Lowest is {}", target.first().unwrap());
}

fn parse_map<I>(lines: I) -> HashMap<String, Vec<Map>>
    where I : Iterator<Item = String> 
{
    let mut maps = init_map();
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
        
        let mut map = Map::new();
        map.dest = nums.next().unwrap();
        map.source = nums.next().unwrap();
        map.len = nums.next().unwrap();

        maps.get_mut(&current_key).unwrap().push(map);
    }

    maps
}

fn init_map() -> HashMap<String, Vec<Map>> {
    HashMap::from([
        (String::from("seed-to-soil"), vec![]),
        (String::from("soil-to-fertilizer"), vec![]),
        (String::from("fertilizer-to-water"), vec![]),
        (String::from("water-to-light"), vec![]),
        (String::from("light-to-temperature"), vec![]),
        (String::from("temperature-to-humidity"), vec![]),
        (String::from("humidity-to-location"), vec![])
    ])
}

fn create_target(source: Vec<u64>, maps: &mut Vec<Map>) -> Vec<u64> {
    let mut target: Vec<u64> = vec![];

    maps.sort_by(|a, b| a.source.cmp(&b.source));    

    for source in source {
        let mut has_pushed = false;

        for map in &*maps {
            if source >= map.source && source < map.source_bound() {
                let delta = source - map.source;
                target.push(map.dest + delta);
                has_pushed = true;
            }
        }    
        
        if !has_pushed {
            target.push(source);
        }    
    }

    target
}