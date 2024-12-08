use std::collections::HashMap;

pub fn read_from_file(is_real: bool, day: u32, order: Option<u32>) -> Vec<String> {
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
    
    input.lines().map(|s| s.to_string()).collect()
}

pub fn read_from_file_and_combine_strings(is_real: bool, day: u32, order: Option<u32>) -> String {
    let lines = read_from_file(is_real, day, order);

    lines.iter().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    })
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

    pub fn new_position(&self, separation: &Separation) -> Option<Position> {
        let x = self.x as i32 + separation.dx;
        let y = self.y as i32 + separation.dy;
        
        if x < 0 || y < 0 {
            return None;
        }

        Some(Position { x: x as usize, y: y as usize, })
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
}

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
