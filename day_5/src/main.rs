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
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
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

    let seeds: Vec<u64> = lines.next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut maps = parse_map(lines);

    let soil = create_target(seeds, &mut maps.get_mut("seed-to-soil").unwrap());
    let fert = create_target(soil, &mut maps.get_mut("soil-to-fertilizer").unwrap());
    let water = create_target(fert, &mut maps.get_mut("fertilizer-to-water").unwrap());
    let light = create_target(water, &mut maps.get_mut("water-to-light").unwrap());
    let temp = create_target(light, &mut maps.get_mut("light-to-temperature").unwrap());
    let hum = create_target(temp, &mut maps.get_mut("temperature-to-humidity").unwrap());
    let mut loc = create_target(hum, &mut maps.get_mut("humidity-to-location").unwrap());

    loc.sort();

    println!("Lowest is {}", loc.first().unwrap());
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