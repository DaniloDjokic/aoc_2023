use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    thread,
};

mod islands;
mod pipe;
use pipe::MapElement;
use pipe::{Dir, ElementType, Point};

fn main() {
    let file_name = "test.txt";
    let is_test_input = file_name == "test.txt";

    let file = File::open(file_name).expect("Cannot open file");
    let reader = BufReader::new(file);

    let (mut map, start) = parse_file(reader, is_test_input);

    let num: u128 = 30000;

    thread::Builder::new()
        .stack_size(num as usize * 0xFF)
        .spawn(move || {
            let mut pipe_loop: HashSet<Point> = HashSet::new();
            _ = walk_map(&map, &start, &mut pipe_loop);

            let islands = islands::split_islands(&mut map);

            for i in islands.iter() {
                println!("Islands: {:?}", i);
                println!(" ");
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

fn walk_map(map: &Vec<Vec<MapElement>>, curr: &Point, visited: &mut HashSet<Point>) -> usize {
    visited.insert(curr.clone());

    let els = check_adj(map, &curr, visited);

    //println!("Adjacent to {:?} are {:?}", curr, els.iter().map(|e| &e.point).collect::<Vec<_>>());

    let first = &els.first();

    return match first {
        Some(first) => walk_map(map, &first.point, visited),
        None => visited.iter().count(),
    };
}

fn check_adj<'a>(
    map: &'a Vec<Vec<MapElement>>,
    curr: &Point,
    visited: &HashSet<Point>,
) -> Vec<&'a MapElement> {
    let mut points = vec![];
    let curr = &map[curr.0][curr.1];

    push_point(map, pipe::MapDirection::Up, curr, &mut points, visited);
    push_point(map, pipe::MapDirection::Down, curr, &mut points, visited);
    push_point(map, pipe::MapDirection::Left, curr, &mut points, visited);
    push_point(map, pipe::MapDirection::Right, curr, &mut points, visited);

    points
}

fn push_point<'a>(
    map: &'a Vec<Vec<MapElement>>,
    dir: pipe::MapDirection,
    curr: &MapElement,
    map_elements: &mut Vec<&'a MapElement>,
    visited: &HashSet<Point>,
) -> Option<Point> {
    if let Some(map_elemnt) = map.map_move(dir, curr) {
        if !visited.contains(&map_elemnt.point) {
            map_elements.push(map_elemnt);

            match map_elemnt.el_type {
                ElementType::BendPipe(_) => {
                    return Some(map_elemnt.point.clone());
                }
                _ => return None,
            }
        }
    }

    None
}

fn parse_file(reader: BufReader<File>, is_test_input: bool) -> (Vec<Vec<MapElement>>, Point) {
    let mut map = vec![];
    let mut start = Point(0, 0);

    for (i, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut row = vec![];

        for (j, char) in line.chars().enumerate() {
            if char == 'S' {
                let el_type = if is_test_input {
                    ElementType::BendPipe(pipe::PipeDir::SouthEast)
                } else {
                    ElementType::BendPipe(pipe::PipeDir::NorthEast)
                };

                row.push(MapElement::new_start(i, j, 'S', el_type));
                start = Point(i, j);
            } else {
                row.push(MapElement::new(i, j, char));
            }
        }

        map.push(row);
    }

    (map, start)
}
