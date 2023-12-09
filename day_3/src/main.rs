use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(usize,usize);

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    
    let gears = parse_file(reader);


    let total = gears.values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().fold(1, |acc, g| {
            acc * g
        }))
        .fold(0, |acc, v| {
            acc + v
        });

    println!("{}", total);
}

fn parse_file(reader: BufReader<File>) -> HashMap<Point, Vec<u32>> {
    let mut gears: HashMap<Point, Vec<u32>> = HashMap::new();

    let split_file: Vec<String> = reader.lines()
        .map(|l| l.unwrap().trim().to_owned())
        .collect();

    let mut curr_num = String::new();
    let mut start_idx: Option<usize> = None;
    let mut end_idx: Option<usize> = None;

    for (line_idx, line) in split_file.iter().enumerate() {        
        for (char_idx, char) in line.chars().enumerate() {
            if char.is_numeric() {
                //println!("Char: {}, Line: {}, Column: {}", char, line_idx, char_idx);
                update_indices(&mut start_idx, &mut end_idx, char_idx);
                curr_num.push(char);
            }
            else {
                if !curr_num.is_empty() {
                    match (start_idx, end_idx) {
                        (Some(start), Some(end)) => {
                            let num = curr_num.parse::<u32>().unwrap();

                            fill_adjecent_gears(line_idx, start, end, &split_file, num, &mut gears);
                            
                            clear_indices(&mut start_idx, &mut end_idx);
                            curr_num = String::new();
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    gears
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

fn fill_adjecent_gears(line_idx: usize, start: usize, end: usize, file: &Vec<String>, num: u32, gears: &mut HashMap<Point, Vec<u32>>) -> bool {
    let width = file[line_idx].len();

    let (left, right) = clamp_char_bounds(
        width,
        (start as isize) - 1,
        end + 1
    );

    let curr_line_chars = &file[line_idx][left..=right];

    fill_gears(curr_line_chars, line_idx, left, gears, num);

    if line_idx != 0 {
        let top_line = &file[line_idx - 1];
        let top_chars = &top_line[left..=right];

        fill_gears(top_chars, line_idx - 1, left, gears, num);
    }

    if line_idx != file.len() - 1 {
        let bot_line = &file[line_idx + 1];
        let bot_chars = &bot_line[left..=right];

        fill_gears(bot_chars, line_idx + 1, left, gears, num);
    }

    false
}

fn clamp_char_bounds(width: usize, left: isize, right: usize) -> (usize, usize) {
    (left.clamp(0, width as isize) as usize, right.clamp(0, width - 1))
}

fn fill_gears(
    line: &str,
    line_idx: usize, 
    start_idx: usize,
    gears: &mut HashMap<Point, Vec<u32>>, 
    num: u32
) -> Option<Point> 
{
    for (i, char) in line.chars().enumerate() {
        if char == '*' {
            let column_idx = start_idx + i;
            let gear_point = Point(line_idx, column_idx);
            let gear_nums = gears.get_mut(&gear_point);
            match gear_nums {
                Some(nums) => {
                    nums.push(num)
                },
                None => {
                    gears.insert(gear_point, vec![num]);
                }
            }
        }
    }

    None
}