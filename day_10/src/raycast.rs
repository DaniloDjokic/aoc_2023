use std::collections::HashSet;

use crate::pipe::{MapElement, Point, ElementType, PipeDir};

pub fn raycast_map(map: Vec<Vec<MapElement>>, pipe_loop: HashSet<Point>) -> HashSet<Point> {
    let mut inside = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let el = &map[i][j];

            if !pipe_loop.contains(&el.point) {
                let cross = is_inside(&map[i][j+1..], &pipe_loop);
                if cross.0 {
                    println!("Adding: {:?}, crossed {} times", &el.point, cross.1);
                    inside.insert(el.point.clone());
                }
            }
        }
    }

    inside
}

fn is_inside(row: &[MapElement], pipe_loop: &HashSet<Point>) -> (bool, i32) {
    let mut cross_count = 0;
    let mut entered_edge: Option<ElementType> = None;

    for i in row.iter() {
        if pipe_loop.contains(&i.point) {
            match &i.el_type {
                ElementType::VerticalPipe => cross_count += 1,
                ElementType::HorizontalPipe | ElementType::Ground | ElementType::Start => (),
                current_pipe => {
                    match &entered_edge {
                        Some(enter_edge) => {
                            match (enter_edge, current_pipe) {
                                (ElementType::BendPipe(PipeDir::NorthEast), ElementType::BendPipe(PipeDir::SouthWest)) => {
                                    cross_count += 1;
                                    entered_edge = None;
                                },
                                (ElementType::BendPipe(PipeDir::SouthEast), ElementType::BendPipe(PipeDir::NorthWest)) => {
                                    cross_count += 1;
                                    entered_edge = None;
                                },
                                _ => {
                                    if let Some(_) = entered_edge {
                                        entered_edge = None
                                    }
                                }
                            }
                        },
                        None => entered_edge = Some(current_pipe.clone())
                    }
                },
            }
        }
    }

    (cross_count % 2 != 0, cross_count)
}