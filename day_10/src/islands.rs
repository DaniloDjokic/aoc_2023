use crate::pipe::{MapElement, Point, MapDirection, PipeDir};
use queues::*;
use crate::pipe::ElementType;

pub fn split_islands(map: &mut Vec<Vec<MapElement>>) -> Vec<Vec<Point>> {
    let mut islands = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !map[i][j].processed && map[i][j].is_ground() {
                let island = flood_fill_island(map, map[i][j].point.clone());
                islands.push(island);
            }
        }
    }

    islands
}

fn add_to_island(map: &mut Vec<Vec<MapElement>>, island: &mut Vec<Point>, queue: &mut Queue<Point>, curr: Option<Point>) {
    if let Some(curr) = curr {
        let map_el = &mut map[curr.0][curr.1];
    
        if !map_el.processed {
            map_el.processed = true;
            island.push(map_el.point.clone());
            queue.add(map_el.point.clone()).unwrap();
        }
    }
}

fn flood_fill_island(map: &mut Vec<Vec<MapElement>>, start: Point) -> Vec<Point> {
    let mut island: Vec<Point> = vec![];
    let mut queue: Queue<Point> = queue![];
    let map_bounds = (map.len(), map[0].len());

    let start = &mut map[start.0][start.1];

    start.processed = true;
    island.push(start.point.clone());
    queue.add(start.point.clone()).unwrap();

    while queue.size() != 0 {
        let current = queue.remove().unwrap();

        if current.0 != 0 {
            let next = get_next_in_island(map, MapDirection::Up, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        if current.0 != map_bounds.0 - 1 {
            let next = get_next_in_island(map, MapDirection::Down, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        if current.1 != 0 {
            let next = get_next_in_island(map, MapDirection::Left, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        if current.1 != map_bounds.1 - 1 {
            let next = get_next_in_island(map, MapDirection::Right, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }
    }

    island
}

fn get_next_in_island(map: &mut Vec<Vec<MapElement>>, dir: MapDirection, current: &Point) -> Option<Point> {
    let bounds = (map.len() as isize, map[0].len() as isize);

    let next = from_dir(dir, current, 0);
    if next.0 < 0 || next.0 >= bounds.0 || next.1 < 0 || next.1 >= bounds.1 {
        return None
    }  
    
    let next = Point(next.0 as usize, next.1 as usize);
    let next_cell = &map[next.0][next.1];

    if next_cell.is_ground() {
        return Some(next)
    }

    return match dir {
        MapDirection::Up => {
            if let Some(next_reachable) = next_reachable_squeeze_left(map, next, MapDirection::Up) {
                Some(next_reachable);
            }

            if let Some(next_reachable) = next_reachable_squeeze_right(map, next, MapDirection::Up) {
                Some(next_reachable);
            }

            None
        },
        MapDirection::Down => {
            if let Some(next_reachable) = next_reachable_squeeze_left(map, next, MapDirection::Down) {
                Some(next_reachable);
            }

            if let Some(next_reachable) = next_reachable_squeeze_right(map, next, MapDirection::Down) {
                Some(next_reachable);
            }

            None 
        },
        MapDirection::Left => {
            if let Some(next_reachable) = next_reachable_squeeze_top(map, next, MapDirection::Left) {
                Some(next_reachable);
            }

            if let Some(next_reachable) = next_reachable_squeeze_bot(map, next, MapDirection::Left) {
                Some(next_reachable);
            }

            None 
        },
        MapDirection::Right => {
            if let Some(next_reachable) = next_reachable_squeeze_top(map, next, MapDirection::Right) {
                Some(next_reachable);
            }

            if let Some(next_reachable) = next_reachable_squeeze_bot(map, next, MapDirection::Right) {
                Some(next_reachable);
            }

            None 
        }
    };
}


/// Next 4 functions should do the same thing accept look for different pipe directions
/// and check only their respective up/down or left/right bounds 
///
/// from the starting point, keep moving until bounds is found or ground is found
/// while loop to update the start and check the end condition 
///
/// for top/down movement
/// squeeze on | can happen for J and 7 on the left and L and F on the right
/// squeeze on J and 7 can only happen on the right for |, L or F
/// squeeze on L and F can only happen on the left for | J or 7
/// 
/// for left/right movement
/// squueze on - can only happen for L or J on the top or F or 7 on the bot
/// squueze on F and 7 can only happen on the top for -, L or J
/// squueze on L and J can only happen on the bot for -, F or 7 

fn next_reachable_squeeze_left(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection) -> Option<Point> {
    match dir {
        MapDirection::Up => {},
        MapDirection::Down => {},
        _ => panic!("Moving left/right when trying to squueze top/bot")
    }
}

fn next_reachable_squeeze_right(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection) -> Option<Point> {
    match dir {
        MapDirection::Up => {},
        MapDirection::Down => {},
        _ => panic!("Moving left/right when trying to squueze top/bot")
    }
}

fn next_reachable_squeeze_top(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection) -> Option<Point> {
    match dir {
        MapDirection::Left => {},
        MapDirection::Right => {},
        _ => panic!("Moving top/bot when trying to squueze left/right")
    }
}

fn next_reachable_squeeze_bot(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection) -> Option<Point> {
    match dir {
        MapDirection::Left => {},
        MapDirection::Right => {},
        _ => panic!("Moving top/bot when trying to squueze left/right")
    }
}

fn has_left(point: &Point) -> bool {
    point.1 > 0
}

fn has_right(point: &Point, bounds: (usize, usize)) -> bool {
    point.1 < bounds.0 - 1
}

fn has_top(point: &Point) -> bool {
    point.0 > 0
}

fn has_bot(point: &Point, bounds: (usize, usize)) -> bool {
    point.0 < bounds.1 - 1
}

fn from_dir(dir: MapDirection, current: &Point, mut offset: isize) -> (isize, isize) {
    offset = 1 + offset;

    match dir {
        MapDirection::Up => (current.0 as isize - offset, current.1 as isize),
        MapDirection::Down => (current.0 as isize + offset, current.1 as isize),
        MapDirection::Left => (current.0 as isize, current.1 as isize - offset),
        MapDirection::Right => (current.0 as isize, current.1 as isize + offset)
    }
}