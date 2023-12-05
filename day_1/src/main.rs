use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

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
    let numbers_spelled = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    let mut numbers: Vec<(char, usize)> = vec![];

    for key in numbers_spelled.keys() {
        let indicies = line.match_indices(key);

        for idx in indicies {
            numbers.push((*numbers_spelled.get(key).unwrap(), idx.0));
        }
    }

    for (i,c) in line.chars().enumerate() {
        if c.is_numeric() {
            numbers.push((c,i));
        }
    }

    numbers.sort_by(|a, b| a.1.cmp(&b.1));

    let mut value = numbers.first().unwrap().0.to_string();
    value.push(numbers.last().unwrap().0);
    let value = value.parse::<u32>().unwrap();

    println!("Line: {}, Numbers: {:?}, Value: {}", line, numbers, value);

    value
}