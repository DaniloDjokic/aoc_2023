use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut total_value = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => { 
                let line_value = get_line_value(&line);
                total_value += line_value;
            }
            Err(e) => {
                eprintln!("Cannot read line, {}", e)
            }
        }
    }

    println!("The value is: {}", total_value);
}

fn get_line_value(line: &str) -> u32 {
    let numbers: Vec<char> = line
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    let mut value = numbers.first().unwrap().to_string();
    value.push(*numbers.last().unwrap());

    value.parse::<u32>().unwrap()
}