use std::{fs::File, io::{BufReader, BufRead }};

fn main() {
    let file = File::open("test.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let res = parse_file(reader); 
}

fn parse_file(reader: BufReader<File>) -> Option<bool> {
    None
}
