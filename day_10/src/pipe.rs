#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point(pub usize, pub usize);

#[derive(Debug)]
pub enum MapDirection {
    Up,
    Down,
    Left,
    Right,
}

pub trait Dir {
    fn map_move(&self, dir: MapDirection, curr: &MapElement) -> Option<&MapElement>;
}

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
                if point.1 == self.len() - 1 {
                    None
                } else {
                    Some(&self[point.0][point.1 + 1])
                }
            }
            MapDirection::Down => {
                if point.0 == self[point.0].len() - 1 {
                    None
                } else {
                    Some(&self[point.0 + 1][point.1])
                }
            }
        };

        if let Some(el) = el {
            if curr.can_continue(dir, &el.el_type) {
                Some(el)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapElement {
    pub point: Point,
    pub char: char,
    pub el_type: ElementType,
    pub inside: bool,
    pub processed: bool,
}

impl MapElement {
    pub fn new(x: usize, y: usize, char: char) -> Self {
        Self {
            point: Point(x, y),
            char: char,
            el_type: ElementType::from(&char),
            inside: false,
            processed: false,
        }
    }

    pub fn new_start(x: usize, y: usize, char: char, el_type: ElementType) -> Self {
        Self {
            point: Point(x, y),
            char: char,
            el_type: el_type,
            inside: false,
            processed: false,
        }
    }

    pub fn is_ground(&self) -> bool {
        return match self.el_type {
            ElementType::Ground => true,
            _ => false,
        };
    }

    pub fn can_continue(&self, dir: MapDirection, other: &ElementType) -> bool {
        //This really didn't have to be this complicated

        return match self.el_type {
            ElementType::VerticalPipe => match dir {
                MapDirection::Up => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::SouthEast)
                    | ElementType::BendPipe(PipeDir::SouthWest) => true,
                    _ => false,
                },
                MapDirection::Down => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::NorthEast)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::HorizontalPipe => match dir {
                MapDirection::Left => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthEast)
                    | ElementType::BendPipe(PipeDir::NorthEast) => true,
                    _ => false,
                },
                MapDirection::Right => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthWest)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::BendPipe(PipeDir::NorthEast) => match dir {
                MapDirection::Up => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::SouthWest)
                    | ElementType::BendPipe(PipeDir::SouthEast) => true,
                    _ => false,
                },
                MapDirection::Right => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthWest)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::BendPipe(PipeDir::NorthWest) => match dir {
                MapDirection::Up => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::SouthWest)
                    | ElementType::BendPipe(PipeDir::SouthEast) => true,
                    _ => false,
                },
                MapDirection::Left => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthEast)
                    | ElementType::BendPipe(PipeDir::NorthEast) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::BendPipe(PipeDir::SouthEast) => match dir {
                MapDirection::Down => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::NorthEast)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                MapDirection::Right => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthWest)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::BendPipe(PipeDir::SouthWest) => match dir {
                MapDirection::Down => match other {
                    ElementType::VerticalPipe
                    | ElementType::BendPipe(PipeDir::NorthEast)
                    | ElementType::BendPipe(PipeDir::NorthWest) => true,
                    _ => false,
                },
                MapDirection::Left => match other {
                    ElementType::HorizontalPipe
                    | ElementType::BendPipe(PipeDir::SouthEast)
                    | ElementType::BendPipe(PipeDir::NorthEast) => true,
                    _ => false,
                },
                _ => false,
            },
            ElementType::Start => panic!("Start should transform!"),
            ElementType::Ground => panic!("We went off course!"),
        };
    }
}

#[derive(Debug, Clone)]
pub enum PipeDir {
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl From<&char> for PipeDir {
    fn from(value: &char) -> Self {
        match value {
            'L' => PipeDir::NorthEast,
            'J' => PipeDir::NorthWest,
            '7' => PipeDir::SouthWest,
            'F' => PipeDir::SouthEast,
            _ => panic!("Cannot parse pipe direction: {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementType {
    HorizontalPipe,
    VerticalPipe,
    BendPipe(PipeDir),
    Ground,
    Start,
}

impl From<&char> for ElementType {
    fn from(value: &char) -> Self {
        match value {
            '|' => ElementType::VerticalPipe,
            '-' => ElementType::HorizontalPipe,
            '.' => ElementType::Ground,
            'S' => ElementType::Start,
            _ => {
                let dir = PipeDir::from(value);
                ElementType::BendPipe(dir)
            }
        }
    }
}
