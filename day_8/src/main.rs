use std::{fs::File, io::{BufReader, BufRead, Lines}, collections::HashMap};

fn main() {
    let file = File::open("test.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    parse_file(reader);
}

fn parse_file(reader: BufReader<File>) {
    let mut lines = reader.lines();

    let directions = lines.next().unwrap().unwrap();
    lines.next();

    println!("Dirs: {}", directions);

    parse_map(lines);
}

fn parse_map(lines: Lines<BufReader<File>>) -> HashMap<String, (Option<&'static str>, Option<&'static str>)> {
    let mut map: HashMap<String, (Option<&str>, Option<&str>)> = HashMap::new();

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

        fill_missing_source(&mut map, &edges, 0);
        fill_missing_source(&mut map, &edges, 1);
        fill_edges(&mut map, &edges, source.to_owned());

       // println!("Source: {}, edges: {:?}", source, edges);
    }

    map
}

fn fill_edges<'a>(map: &'a mut HashMap<String, (Option<&'a str>, Option<&'a str>)>, edges: &Vec<&'a str>, source: String) {
    let (k_1, _) = map.get_key_value(edges[0]).unwrap();
    let (k_2, _) = map.get_key_value(edges[1]).unwrap();

    map.entry(source.clone()).or_insert_with(|| (Some(k_1), Some(k_2)));
}


fn fill_missing_source(map: &mut HashMap<String, (Option<&str>, Option<&str>)>, edges: &Vec<&str>, i: usize) {
    if !map.contains_key(edges[i]) {
        map.insert(edges[i].to_owned(), (None, None)).unwrap();
    }
}