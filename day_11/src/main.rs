use std::collections::HashSet;
#[allow(unused)]
use std::{fs::File, io::{BufReader, BufRead }};

impl Dir for Vec<Vec<MapElement>> {
    fn map_move(&self, dir: MapDirection, curr: &MapElement) -> Option<&MapElement> {
        let point = &curr.point;

        let el = match dir {
            MapDirection::Up => {
                if point.0 == 0 {
                    None
                } else {
                    Some(&self[point.0 - 1][point.1])
                }
            }
            MapDirection::Left => {
                if point.1 == 0 {
                    None
                } else {
                    Some(&self[point.0][point.1 - 1])
                }
            }
            MapDirection::Right => {
                if point.1 == self[point.0].len() - 1 {
                    None
                } else {
                    Some(&self[point.0][point.1 + 1])
                }
            }
            MapDirection::Down => {
                if point.0 == self.len() - 1 {
                    None
                } else {
                    Some(&self[point.0 + 1][point.1])
                }
            }
        };

        if let Some(el) = el {
            Some(el)
        } else {
            None
        }
    }
}

trait Dir {
    fn map_move(&self, dir: MapDirection, curr: &MapElement) -> Option<&MapElement>;
}

#[derive(Debug)]
pub enum MapDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point(usize, usize);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MapElement {
    point: Point,
    is_galaxy: bool,
    is_jump: bool,
    id: Option<usize>
}

impl MapElement {
    fn new(x: usize, y: usize, is_galaxy: bool, is_jump: bool, id: Option<usize>) -> Self {
        Self {
            point: Point(x, y),
            is_galaxy,
            is_jump,
            id
        }
    }
}

const FACTOR: u64 = 1000000;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut map = parse_file(reader);
    let column_expansion_points = expand_columns(&mut map);
    let row_expansion_points = expand_rows(&mut map);

    let map = convert_map(map);

    for row in map.iter() {
        for el in row.iter() {
            print!("{}", if el.is_galaxy {'#'} else { if el.is_jump {'X'} else {'.'}})
        }

        println!();
    }

    let pairs = create_pairs(&map);

    let sum = pairs.iter().fold(0, |acc, x| {
        acc + shortest_path_len(x.0, x.1, &row_expansion_points, &column_expansion_points)
    });

    println!("Sum: {}", sum);
}

fn get_cross_count(bound: &Vec<usize>, left: usize, right: usize) -> u64 {
    bound.iter().fold(0, |acc, b| {
        acc + if (*b > left && *b < right) || (*b < left && *b > right)
        { 1 } else { 0 }
    })
}

fn shortest_path_len(start: &MapElement, end: &MapElement, row_exp: &Vec<usize>, col_exp: &Vec<usize>) -> u64 {
    let x_cross = get_cross_count(row_exp, start.point.0, end.point.0);
    let y_cross = get_cross_count(col_exp, start.point.1, end.point.1);

   // println!("Start {:?}, End {:?}, crosses X {x_cross} times, crosses Y {y_cross} times", start.point, end.point);

    let left = (end.point.0 as isize - start.point.0 as isize).abs();
    let right = (end.point.1 as isize - start.point.1 as isize).abs();
    
    let x = left as u64 + (x_cross * FACTOR) + right as u64 + (y_cross * FACTOR);
   // println!("Length is {x}");

    x - (y_cross + x_cross)
}

fn create_pairs(map: &Vec<Vec<MapElement>>) -> HashSet<(&MapElement, &MapElement)> {
    let mut pairs: HashSet<(&MapElement, &MapElement)> = HashSet::new();

    let galaxies = map.iter()
        .flatten()
        .filter(|x| x.is_galaxy)
        .collect::<Vec<&MapElement>>();
        
    for gal in galaxies.iter() {
        for other in galaxies.iter().filter(|x| x.id != gal.id) {
            if should_insert(&pairs, gal, other) {
                pairs.insert((*gal, *other));
            }
        }
    }

    pairs
}

fn should_insert(pairs: &HashSet<(&MapElement, &MapElement)>, gal: &MapElement, other: &MapElement) -> bool {
    !(pairs.contains(&(gal, other)) || pairs.contains(&(other, gal)))
}

fn convert_map(map: Vec<Vec<char>>) -> Vec<Vec<MapElement>> {
    let mut new_map = vec![];
    let mut id: usize = 0;

    for (i, row) in map.iter().enumerate() {
        let mut new_row = vec![];
        
        for (j, ch) in row.iter().enumerate() {
            let is_galaxy = *ch == '#';
            let is_jump = *ch == 'X';
            if is_galaxy { id += 1; }

            new_row.push(MapElement::new(i, j, is_galaxy, is_jump, Some(id)))
        }

        new_map.push(new_row);
    }

    new_map
}

fn expand_rows(map: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut expansion_points = vec![];

    for (i, row) in map.iter_mut().enumerate() {
        if !row.iter().any(|x| *x == '#') {
            for ch in row.iter_mut() {
                *ch = 'X';
            }
            expansion_points.push(i);
        }
    }

    expansion_points
}

fn expand_columns(map: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut expansion_points = vec![];

    for i in 0..map[0].len() {
        if map.iter().map(|v| v[i]).all(|c| c == '.') {
            for el in map.iter_mut().map(|v| &mut v[i]) {
                *el = 'X';
            }

            expansion_points.push(i);
        }
    }

    expansion_points
}

fn parse_file(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut new_line = vec![];
        
        for char in line.chars() {
            new_line.push(char);
        }

        map.push(new_line.clone());
    }

    map
}
