use std::{fs::File, io::{BufReader, BufRead, Lines}, collections::HashMap};


fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let steps = parse_file(reader);
    println!("Steps: {}", steps);
}

fn parse_file(reader: BufReader<File>) -> u128 {
    let mut lines = reader.lines();

    let directions = lines.next().unwrap().unwrap();
    lines.next();

    let map = parse_map(lines);

    let mut starts: Vec<(&String, u128)> = map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| (k, 0))
        .collect();

    for start in starts.iter_mut() {
        start.1 = find_next_z(start.0.clone(), &directions, &map);
    }

    println!("Starts: {:?}", starts);

    lcm_c(starts.iter().map(|s| s.1).collect())
}

fn find_next_z(mut start: String, directions: &String, map: &HashMap<String, (String, String)>) -> u128 {
    let mut steps = 0;

    for dir in directions.chars().cycle() {
        if start.ends_with('Z') {
            return steps;
        }

        steps += 1;

        let val = map.get(&start).unwrap();
        
        match dir {
            'L' => start = val.0.clone(),
            'R' => start = val.1.clone(),
            _ => panic!("Not L or R")
        }
    }

    panic!("Can't find Z for {start}")
}

fn lcm_c(col: Vec<u128>) -> u128 {
    col.iter().fold(1, |acc, &x| lcm(acc, x))
}

fn lcm(first: u128, second: u128) -> u128 {
    first * second / gcd(first, second)
}

fn gcd(first: u128, second: u128) -> u128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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