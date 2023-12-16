use std::{fs::File, io::{BufReader, BufRead, Lines}, collections::HashMap};

#[derive(Debug, Clone)]
struct Node {
    pub left: String,
    pub right: String,
    pub next_z: Option<String>,
    pub steps_to: Option<u32>
}

impl Node {
    fn new(left: String, right: String) -> Self {
        Self { left, right, next_z: None, steps_to: None }
    }

    fn set_next_z(&mut self, name: String, map: &HashMap<String, Node>, dir: &String) {
        let mut steps_to: u32 = 0;
        let mut next_z: String = name.clone();

        for dir in dir.chars().cycle() {
            if next_z.ends_with('Z') {
                break;
            }

            steps_to += 1;
            let val = map.get(&next_z).unwrap();

            match dir {
                'L' => next_z = val.left.clone(),
                'R' => next_z = val.right.clone(),
                _ => panic!("Not L or R")
            }
        }

        println!("For: {}, found: {}", name, next_z);

        self.next_z = Some(next_z);
        self.steps_to = Some(steps_to);
    }
}

fn main() {
    let file = File::open("test.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let steps = parse_file(reader);
    println!("Steps: {}", steps);
}

fn parse_file(reader: BufReader<File>) -> u32 {
    let mut lines = reader.lines();

    let dir = lines.next().unwrap().unwrap();
    lines.next();

    let mut map = parse_map(lines);
    let clone = map.clone();

    for (key, val) in map.iter_mut() {
        val.set_next_z(key.clone(), &clone, &dir);
    }

    

    0
}

fn parse_map(lines: Lines<BufReader<File>>) -> HashMap<String, Node> {
    let mut map: HashMap<String, Node> = HashMap::new();

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

        let node = Node::new(edges[0].to_owned(), edges[1].to_owned());
        map.insert(source.to_owned(), node);
    }

    map
}