use std::collections::HashSet;
use queues::*;
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
struct Point(usize, usize, Option<usize>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MapElement {
    point: Point,
    is_galaxy: bool,
    id: Option<usize>
}

impl MapElement {
    fn new(x: usize, y: usize, is_galaxy: bool, id: Option<usize>) -> Self {
        Self {
            point: Point(x, y, None),
            is_galaxy,
            id
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut map = parse_file(reader);
    expand_columns(&mut map);

    let map = convert_map(map);

    let pairs = create_pairs(&map);

    let sum = pairs.iter().fold(0, |acc, x| {
        acc + shortest_path_len(x.0, x.1)
    });

    println!("Sum: {}", sum);
}

fn shortest_path_len(start: &MapElement, end: &MapElement) -> usize {
    ((end.point.0 as isize - start.point.0 as isize).abs() + (end.point.1 as isize - start.point.1 as isize).abs()) as usize
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

            if is_galaxy { id += 1; }

            new_row.push(MapElement::new(i, j, is_galaxy, Some(id)))
        }

        new_map.push(new_row);
    }

    new_map
}

fn expand_columns(map: &mut Vec<Vec<char>>) {
    let mut width = map[0].len();

    let mut skip = false;
    let mut i = 0;

    while i < width {
        if skip {
            skip = false;
            i += 1;
            continue;
        }

        if map.iter().map(|v| v[i]).all(|c| c == '.') {
            for row in map.iter_mut() {
                row.insert(i, '.');
                width = row.len();
                skip = true;
            }
        }

        i += 1;
    }
}

fn parse_file(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut new_line = vec![];
        
        for char in line.chars() {
            new_line.push(char);
        }

        map.push(new_line.clone());

        if line.chars().all(|c| c == '.') {
            map.push(new_line);
        }
    }

    map
}
