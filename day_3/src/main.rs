use std::{fs::File, io::{BufReader, BufRead, Read}};

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    
    let symbols = vec!['*', '@', '=', '+', '%', '$', '&', '#', '/', '-'];

    let numbers = parse_file(reader, &symbols);

    println!("{:?}", numbers);

    let sum = numbers.iter().fold(0, |acc, n| {
        acc + n
    });

    println!("Sum is: {}", sum);
}

fn parse_file(reader: BufReader<File>, symbols: &Vec<char>) -> Vec<u32> {
    let mut numbers = vec![];

    let split_file: Vec<String> = reader.lines()
        .map(|l| l.unwrap().trim().to_owned())
        .collect();

    let mut curr_num = String::new();
    let mut start_idx: Option<usize> = None;
    let mut end_idx: Option<usize> = None;

    for (line_idx, line) in split_file.iter().enumerate() {        
        for (char_idx, char) in line.chars().enumerate() {
            if char.is_numeric() {
                update_indices(&mut start_idx, &mut end_idx, char_idx);
                curr_num.push(char);
            }
            else {
                if !curr_num.is_empty() {
                    match (start_idx, end_idx) {
                        (Some(start), Some(end)) => {
                            let has_adjacent = check_adjacent(line_idx, start, end, &split_file, symbols);

                            if has_adjacent {
                                numbers.push(curr_num.parse::<u32>().unwrap());
                            }

                            clear_indices(&mut start_idx, &mut end_idx);
                            curr_num = String::new();
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    numbers
}

fn update_indices(start: &mut Option<usize>, end: &mut Option<usize>, new: usize) {
    if let None = start {
        *start = Some(new);
    }

    *end = Some(new);
}

fn clear_indices(start: &mut Option<usize>, end: &mut Option<usize>) {
    *start = None;
    *end = None;
}

fn check_adjacent(line_idx: usize, start: usize, end: usize, file: &Vec<String>, symbols: &Vec<char>) -> bool {
    let width = file[line_idx].len();

    let (left, right) = clamp_char_bounds(
        width,
        (start as isize) - 1,
        end + 1
    );

    let curr_line_chars = &file[line_idx][left..=right];

    if has_symbol(curr_line_chars, symbols) {
        return true;
    }

    if line_idx != 0 {
        let top_line = &file[line_idx - 1];
        let top_chars = &top_line[left..=right];

        if has_symbol(top_chars, symbols) {
            return true;
        }
    }

    if line_idx != file.len() - 1 {
        let bot_line = &file[line_idx + 1];
        let bot_chars = &bot_line[left..=right];

        if has_symbol(bot_chars, symbols) {
            return true;
        }
    }

    false
}

fn clamp_char_bounds(width: usize, left: isize, right: usize) -> (usize, usize) {
    (left.clamp(0, width as isize) as usize, right.clamp(0, width - 1))
}

fn has_symbol(line: &str, symbols: &Vec<char>) -> bool {
    for char in line.chars() {
        if symbols.contains(&char) {
            return true;
        }
    }

    false
}