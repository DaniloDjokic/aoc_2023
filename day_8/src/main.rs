use std::{fs::File, io::{BufReader, BufRead, Lines}, collections::HashMap};

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let steps = parse_file(reader);
    println!("Steps: {}", steps);
}

fn parse_file(reader: BufReader<File>) -> u32 {
    let mut lines = reader.lines();

    let directions = lines.next().unwrap().unwrap();
    lines.next();

    println!("Dirs: {}", directions);

    let map = parse_map(lines);

    let mut curr = "AAA";
    let mut steps = 0;

    for dir in directions.chars().cycle() {
        if curr == "ZZZ" {
            return steps;
        }

        steps += 1;

        let val = map.get(curr).unwrap();

        match dir {
            'L' => curr = &val.0,
            'R' => curr = &val.1,
            _ => panic!("Not L or R")
        }
    }

    panic!("How did we end a cycle?")
}

fn parse_map(lines: Lines<BufReader<File>>) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let split = line.split_once("=").unwrap();
        let source = split.0.trim();
        
        let edges_start = split.1.find("(").unwrap();
        let edges_end = split.1.find(")").unwrap();

        let edges: Vec<&str> = split.1[edges_start+1..edges_end]
            .split(",")
            .map(|s| s.trim())
            .collect();

        map.insert(source.to_owned(), (edges[0].to_owned(), edges[1].to_owned()));
    }

    map
}