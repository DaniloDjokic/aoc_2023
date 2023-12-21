use crate::pipe::{MapElement, Point};
use queues::*;

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
            let up = Point(current.0 - 1, current.1);

            if !&map[up.0][up.1].processed && map[up.0][up.1].is_ground() {
                map[up.0][up.1].processed = true;
                island.push(map[up.0][up.1].point.clone());
                queue.add(map[up.0][up.1].point.clone()).unwrap();
            }
        }

        if current.0 != map_bounds.0 - 1 {
            let down = Point(current.0 + 1, current.1);

            if !&map[down.0][down.1].processed && map[down.0][down.1].is_ground() {
                map[down.0][down.1].processed = true;
                island.push(map[down.0][down.1].point.clone());
                queue.add(map[down.0][down.1].point.clone()).unwrap();
            }
        }

        if current.1 != 0 {
            let left = Point(current.0, current.1 - 1);

            if !&map[left.0][left.1].processed && map[left.0][left.1].is_ground() {
                map[left.0][left.1].processed = true;
                island.push(map[left.0][left.1].point.clone());
                queue.add(map[left.0][left.1].point.clone()).unwrap();
            }
        }

        if current.1 != map_bounds.1 - 1 {
            let right = Point(current.0, current.1 + 1);

            if !&map[right.0][right.1].processed && map[right.0][right.1].is_ground() {
                map[right.0][right.1].processed = true;
                island.push(map[right.0][right.1].point.clone());
                queue.add(map[right.0][right.1].point.clone()).unwrap();
            }
        }
    }

    island
}
