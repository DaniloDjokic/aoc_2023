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

            //println!("Added {:?}", map_el.point.clone());
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
        //println!("Current is {:?}", &current);

        if current.0 != 0 {
            let next = get_next_in_island(map, MapDirection::Up, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        let mut next = next_reachable_squeeze_left(map, current.clone(), MapDirection::Up, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        next = next_reachable_squeeze_right(map, current.clone() , MapDirection::Up, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        if current.0 != map_bounds.0 - 1 {
            let next = get_next_in_island(map, MapDirection::Down, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        next = next_reachable_squeeze_left(map, current.clone(), MapDirection::Down, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        next = next_reachable_squeeze_right(map, current.clone() , MapDirection::Down, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        if current.1 != 0 {
            let next = get_next_in_island(map, MapDirection::Left, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        if current.1 != map_bounds.1 - 1 {
            let next = get_next_in_island(map, MapDirection::Right, &current);
            add_to_island(map, &mut island, &mut queue, next);
        }

        next = next_reachable_squeeze_top(map, current.clone(), MapDirection::Left, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        next = next_reachable_squeeze_bot(map, current.clone(), MapDirection::Left, map_bounds);
        add_to_island(map, &mut island, &mut queue, next);

        // next = next_reachable_squeeze_top(map, current.clone(), MapDirection::Right, map_bounds);
        // add_to_island(map, &mut island, &mut queue, next);

        // next = next_reachable_squeeze_bot(map, current.clone(), MapDirection::Right, map_bounds);
        // add_to_island(map, &mut island, &mut queue, next);
    }

    island
}

fn get_next_in_island(map: &mut Vec<Vec<MapElement>>, dir: MapDirection, current: &Point) -> Option<Point> {
    let bounds = (map.len() as isize, map[0].len() as isize);

    let next = from_dir(&dir, current, 0);
    if next.0 < 0 || next.0 >= bounds.0 || next.1 < 0 || next.1 >= bounds.1 {
        return None
    }  
    
    let next = Point(next.0 as usize, next.1 as usize);
    let next_cell = &map[next.0][next.1];

    if next_cell.is_ground() {
        return Some(next)
    }

    None
}

fn can_squeeze(center_el: &ElementType, side_el: &ElementType, dir: &MapDirection) -> bool {
    return match (center_el, dir) {
        (ElementType::VerticalPipe, MapDirection::Left) => {
            match side_el { // || or 7| or J|
                ElementType::VerticalPipe | 
                ElementType::BendPipe(PipeDir::NorthWest) | 
                ElementType::BendPipe(PipeDir::SouthWest) => true,
                _ => false 
            }
        },
        (ElementType::VerticalPipe, MapDirection::Right) => {
            match side_el { // || or |L or |F
                ElementType::VerticalPipe |
                ElementType::BendPipe(PipeDir::SouthEast) |
                ElementType::BendPipe(PipeDir::NorthEast) => true,
                _ => false,
            }
        },
        (ElementType::BendPipe(PipeDir::NorthWest) | ElementType::BendPipe(PipeDir::SouthWest), MapDirection::Right) => {
            match side_el {
                ElementType::VerticalPipe |
                ElementType::BendPipe(PipeDir::NorthEast) |
                ElementType::BendPipe(PipeDir::SouthEast) => true,
                _ => false,
            }
        },
        (ElementType::BendPipe(PipeDir::NorthEast) | ElementType::BendPipe(PipeDir::SouthEast), MapDirection::Left) => {
            match side_el {
                ElementType::VerticalPipe |
                ElementType::BendPipe(PipeDir::NorthWest) |
                ElementType::BendPipe(PipeDir::SouthWest) => true,
                _ => false,
            }
        },
        (ElementType::HorizontalPipe, MapDirection::Up) => {
            match side_el {
                ElementType::HorizontalPipe | 
                ElementType::BendPipe(PipeDir::NorthWest) | 
                ElementType::BendPipe(PipeDir::NorthEast) => true,
                _ => false 
            }
        },
        (ElementType::HorizontalPipe, MapDirection::Down) => {
            match side_el {
                ElementType::HorizontalPipe | 
                ElementType::BendPipe(PipeDir::SouthWest) | 
                ElementType::BendPipe(PipeDir::SouthEast) => true,
                _ => false 
            }
        },
        (ElementType::BendPipe(PipeDir::SouthEast) | ElementType::BendPipe(PipeDir::SouthWest), MapDirection::Up) => {
            match side_el {
                ElementType::HorizontalPipe |
                ElementType::BendPipe(PipeDir::NorthEast) |
                ElementType::BendPipe(PipeDir::NorthWest) => true,
                _ => false,
            }
        },
        (ElementType::BendPipe(PipeDir::NorthEast) | ElementType::BendPipe(PipeDir::NorthWest), MapDirection::Down) => {
            match side_el {
                ElementType::HorizontalPipe |
                ElementType::BendPipe(PipeDir::SouthEast) |
                ElementType::BendPipe(PipeDir::SouthWest) => true,
                _ => false,
            }
        },
        _ => false
    }
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

fn next_reachable_squeeze_left(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection, bounds: (usize, usize)) -> Option<Point> {
    match dir {
        MapDirection::Up => {
            //println!("Going up, squeeze left");
            let mut curr = &map[start.0][start.1];

            while has_top(&curr.point) {
                let up = &map[curr.point.0 - 1][curr.point.1];

                if up.is_ground() {
                    return Some(up.point.clone()); 
                }

                if has_left(&up.point) {
                    let up_left = &map[up.point.0][up.point.1 - 1];
                    
                    if can_squeeze(&up.el_type, &up_left.el_type, &MapDirection::Left) {
                        curr = up;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        MapDirection::Down => {
            //println!("Going down, squeeze left");
            let mut curr = &map[start.0][start.1];

            while has_bot(&curr.point, bounds) {
                let bot = &map[curr.point.0 + 1][curr.point.1];

                if bot.is_ground() {
                    return Some(bot.point.clone()); 
                }

                if has_left(&bot.point) {
                    let bot_left = &map[bot.point.0][bot.point.1 - 1];
                    
                    if can_squeeze(&bot.el_type, &bot_left.el_type, &MapDirection::Left) {
                        curr = bot;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        _ => panic!("Moving left/right when trying to squueze top/bot")
    }
} 

fn next_reachable_squeeze_right(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection, bounds: (usize, usize)) -> Option<Point> {
    match dir {
        MapDirection::Up => {
            //println!("Going up, squeeze right");
            let mut curr = &map[start.0][start.1];

            while has_top(&curr.point) {
                let up = &map[curr.point.0 - 1][curr.point.1];

                if up.is_ground() {
                    return Some(up.point.clone()); 
                }

                if has_right(&up.point, bounds) {
                    let up_right = &map[up.point.0][up.point.1 + 1];
                    
                    if can_squeeze(&up.el_type, &up_right.el_type, &MapDirection::Right) {
                        curr = up;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        MapDirection::Down => {
            //println!("Going down, squeeze right");
            let mut curr = &map[start.0][start.1];

            while has_bot(&curr.point, bounds) {
                let bot = &map[curr.point.0 + 1][curr.point.1];

                if bot.is_ground() {
                    return Some(bot.point.clone()); 
                }

                if has_right(&bot.point, bounds) {
                    let bot_right = &map[bot.point.0][bot.point.1 + 1];
                    
                    if can_squeeze(&bot.el_type, &bot_right.el_type, &MapDirection::Right) {
                        curr = bot;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        _ => panic!("Moving left/right when trying to squueze top/bot")
    }
}

fn next_reachable_squeeze_top(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection, bounds: (usize, usize)) -> Option<Point> {
    match dir {
        MapDirection::Left => {
            //println!("Going left, squeeze top");
            let mut curr = &map[start.0][start.1];

            while has_left(&curr.point) {
                let left = &map[curr.point.0][curr.point.1 - 1];

                if left.is_ground() {
                    return Some(left.point.clone()); 
                }

                if has_top(&left.point) {
                    let left_up = &map[left.point.0 - 1][left.point.1];
                    
                    if can_squeeze(&left.el_type, &left_up.el_type, &MapDirection::Up) {
                        curr = left;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        MapDirection::Right => {
            //println!("Going right, squeeze top");
            let mut curr = &map[start.0][start.1];

            while has_right(&curr.point, bounds) {
                let right = &map[curr.point.0][curr.point.1 + 1];
                
                if right.is_ground() {
                    return Some(right.point.clone()); 
                }

                if has_top(&right.point) {
                    let right_up = &map[right.point.0 - 1][right.point.1];
                    
                    if can_squeeze(&right.el_type, &right_up.el_type, &MapDirection::Up) {
                        curr = right;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        _ => panic!("Moving top/bot when trying to squueze left/right")
    }
}

fn next_reachable_squeeze_bot(map: &mut Vec<Vec<MapElement>>, start: Point, dir: MapDirection, bounds: (usize, usize)) -> Option<Point> {
    match dir {
        MapDirection::Left => {
            //println!("Going left, squeeze bot");
            let mut curr = &map[start.0][start.1];

            while has_left(&curr.point) {
                let left = &map[curr.point.0][curr.point.1 - 1];

                if left.is_ground() {
                    return Some(left.point.clone()); 
                }

                if has_bot(&left.point, bounds) {
                    let left_down = &map[left.point.0 + 1][left.point.1];
                    
                    if can_squeeze(&left.el_type, &left_down.el_type, &MapDirection::Down) {
                        curr = left;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        MapDirection::Right => {
            //println!("Going right, squeeze bot");
            let mut curr = &map[start.0][start.1];

            while has_right(&curr.point, bounds) {
                let right = &map[curr.point.0][curr.point.1 + 1];

                if right.is_ground() {
                    return Some(right.point.clone()); 
                }

                if has_bot(&right.point, bounds) {
                    let right_down = &map[right.point.0 + 1][right.point.1];
                    
                    if can_squeeze(&right.el_type, &right_down.el_type, &MapDirection::Down) {
                        curr = right;
                    }
                    else {
                        break;
                    }
                }
                else { break; }
            }

            None
        },
        _ => panic!("Moving top/bot when trying to squueze left/right")
    }
}

fn has_left(point: &Point) -> bool {
    point.1 > 0
}

fn has_right(point: &Point, bounds: (usize, usize)) -> bool {
    point.1 < bounds.1 - 1
}

fn has_top(point: &Point) -> bool {
    point.0 > 0
}

fn has_bot(point: &Point, bounds: (usize, usize)) -> bool {
    point.0 < bounds.0 - 1
}

fn from_dir(dir: &MapDirection, current: &Point, mut offset: isize) -> (isize, isize) {
    offset = 1 + offset;

    match dir {
        MapDirection::Up => (current.0 as isize - offset, current.1 as isize),
        MapDirection::Down => (current.0 as isize + offset, current.1 as isize),
        MapDirection::Left => (current.0 as isize, current.1 as isize - offset),
        MapDirection::Right => (current.0 as isize, current.1 as isize + offset)
    }
}