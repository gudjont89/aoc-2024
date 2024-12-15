use std::collections::HashMap;

pub fn read_from_file(is_real: bool, day: u32, order: Option<u32>) -> Vec<String> {
    let input = read_single_string_from_file(is_real, day, order);
    
    input.lines().map(|s| s.to_string()).collect()
}

pub fn read_single_string_from_file(is_real: bool, day: u32, order: Option<u32>) -> String {
    let suffix = match order {
        Some(o) => format!("_{}", o),
        None => "".to_string()
    };

    let data_file = if is_real {
        format!("real_data{}.txt", suffix)
    } else {
        format!("test_data{}.txt", suffix)
    };
    
    let source_folder = std::env::current_dir().expect("Failed to get current directory").join("src");
    let date_folder = format!("dec_{:02}", day);

    let data_file = source_folder.join(date_folder).join(data_file);
    let input = std::fs::read_to_string(data_file).expect("Failed to read file");

    input
}

pub fn get_integers_in_string(s: &str) -> Vec<i32> {
    let rgx = regex::Regex::new(r"\d+").unwrap();

    rgx.find_iter(s).map(|m| m.as_str().parse::<i32>().unwrap()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn separated_from_by(&self, other: &Position) -> Separation {
        let dx = self.x as i32 - other.x as i32;
        let dy = self.y as i32 - other.y as i32;

        Separation { dx, dy }
    }

    pub fn walk_in_direction(&self, direction: &Direction) -> Option<Position> {
        let movement = direction.get_movement();

        self.new_position(&movement)
    }

    pub fn new_position(&self, separation: &Separation) -> Option<Position> {
        let x = self.x as i32 + separation.dx;
        let y = self.y as i32 + separation.dy;
        
        if x < 0 || y < 0 {
            return None;
        }

        Some(Position { x: x as usize, y: y as usize, })
    }

    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        let distance = self.x.abs_diff(other.x) + self.y.abs_diff(other.y);

        distance == 1
    }

    pub fn move_wrapping_around(
        self, 
        separation: &Separation, 
        dimensions: &Dimensions
    ) -> Self {
        let x = (self.x as i32 + separation.dx).rem_euclid(dimensions.width as i32) as usize;
        let y = (self.y as i32 + separation.dy).rem_euclid(dimensions.height as i32) as usize;

        Position { x, y }
    }
}

pub struct Separation {
    pub dx: i32,
    pub dy: i32,
}

impl Separation {
    pub fn negative(&self) -> Self {
        Separation { dx: -self.dx, dy: -self.dy }
    }

    pub fn multiply(&self, n: usize) -> Self {
        Separation { dx: (n as i32) * self.dx, dy: (n as i32) * self.dy }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '#'|'.' => None,
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            'v' => Some(Direction::S),
            '<' => Some(Direction::W),
            _ => None,
        }
    }

    pub fn get_movement(&self) -> Separation {
        match self {
            Direction::N => Separation { dx: 0, dy: -1 },
            Direction::NE => Separation { dx: 1, dy: -1 },
            Direction::E => Separation { dx: 1, dy: 0 },
            Direction::SE => Separation { dx: 1, dy: 1 },
            Direction::S => Separation { dx: 0, dy: 1 },
            Direction::SW => Separation { dx: -1, dy: 1 },
            Direction::W => Separation { dx: -1, dy: 0 },
            Direction::NW => Separation { dx: -1, dy: -1 },
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            _ => panic!(""),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            _ => panic!(""),
        }
    }

    pub fn get_total_movement(&self, order: usize) -> Separation {
        let single_separation = self.get_movement();

        Separation{ 
            dx: order as i32 * single_separation.dx, 
            dy: order as i32 * single_separation.dy, 
        }
    }
}

pub fn cardinal_directions() -> Vec<Direction> {
    vec![
        Direction::N, 
        Direction::E, 
        Direction::S, 
        Direction::W,
    ]
}

pub fn ordinal_directions() -> Vec<Direction> {
    vec![
        Direction::N, 
        Direction::NE, 
        Direction::E, 
        Direction::SE, 
        Direction::S, 
        Direction::SW, 
        Direction::W, 
        Direction::NW
    ]
}

// todo: add function for ordinal_directions

pub fn position_map_from_text_lines<T> (
    lines: &[String], 
    parse_from_char: fn(char) -> T
) -> HashMap<Position, T> {
    lines
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let position = Position { x, y };
                    acc.insert(position, parse_from_char(c));
                }
            );

            acc
        })
}

pub fn position_and_object_from_text_lines<T> (
    lines: &[String], 
    parse_from_char: fn(char) -> Option<T>
) -> Option<(Position, T)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)|
            line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let o = parse_from_char(c)?;
                    let position = Position { x, y };

                    Some((position, o))
                    }
                )
                .collect::<Vec<(Position, T)>>()
        )
        .next()
}

#[derive(Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn includes(&self, position: &Position) -> bool {
        position.x < self.width && position.y < self.height
    }
}

pub fn get_position_map_dimensions<T>(
    position_map: &HashMap<Position, T>
) -> Option<Dimensions> {
    let width = 1 + position_map
        .keys()
        .map(|p| p.x)
        .max()?;
    let height = 1 + position_map
        .keys()
        .map(|p| p.y)
        .max()?;
    
    Some( Dimensions { width, height, } )
}

pub fn positions_on_map_with_value<T>(
    position_map: &HashMap<Position, T>,
    value: T,
) -> Vec<Position> where T: PartialEq {
    position_map
        .iter()
        .filter(|(_p, v)| **v == value)
        .map(|(p, _)| *p)
        .collect::<Vec<Position>>()
}
