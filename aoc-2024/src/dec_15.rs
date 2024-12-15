use std::collections::HashMap;

use crate::util::{position_map_from_text_lines, read_from_file, Direction, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 15, None);

    let mut map_lines = Vec::new();
    let mut line_iterator = lines.into_iter().peekable();

    while is_map_line(line_iterator.peek()) {
        map_lines.push(line_iterator.next().unwrap());
    }

    let warehouse_map = position_map_from_text_lines(&map_lines, |c| parse_location_from_char(c));

    _ = line_iterator.next();

    let directions = line_iterator
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .filter_map(|c| Direction::from_char(c))
        .collect::<Vec<Direction>>();

    let warehouse_map = directions
        .into_iter()
        .fold(warehouse_map, |map_before, d| move_robot_in_direction(map_before, d));

    warehouse_map
        .iter()
        .filter(|(_, l)| l == &&Location::Crate)
        .map(|(p, _)| p.x + 100 * p.y)
        .sum::<usize>()
}

pub fn run_second(is_real: bool) -> usize {
    0
}

fn is_map_line(maybe_line: Option<&String>) -> bool {
    let line = match maybe_line {
        Some(line) => line,
        None => return false,
    };

    match line.chars().next() {
        Some(first_char) => first_char == '#',
        None => false,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Location {
    Obstruction,
    Crate,
    Robot,
    Free,
}

fn parse_location_from_char(c: char) -> Location {
    match c {
        '#' => Location::Obstruction,
        'O' => Location::Crate,
        '@' => Location::Robot,
        '.' => Location::Free,
        _ => panic!("Unexpected character"),
    }
}

fn move_robot_in_direction(
    mut warehouse_map: HashMap<Position, Location>,
    direction: Direction,
) -> HashMap<Position, Location> {
    let initial_robot_position = warehouse_map
        .iter()
        .find(|(_p, l)| *l == &Location::Robot)
        .map(|(p, _)| p.clone()).unwrap();

    let mut last_position = initial_robot_position;
    let mut last_location = Location::Robot;
    let mut modified_locations = Vec::new();

    loop {
        let end_position = last_position.walk_in_direction(&direction).unwrap();
        let end_location = warehouse_map.get(&end_position).unwrap().clone();

        if end_location == Location::Obstruction {
            return warehouse_map;
        }

        modified_locations.push((end_position, last_location));

        if end_location == Location::Free {
            break;
        }

        last_location = end_location;
        last_position = end_position;
    }

    modified_locations.push((initial_robot_position, Location::Free));

    for (position, location) in modified_locations {
        warehouse_map.insert(position, location);
    }

    // if the location into which the robot is moving is an obstruction, stop and return the same map
    // if the location into which the robot is moving is free, move there and stop
    // if the location into which the robot is moving has a crate, check the next one, until we run into either a free location or an obstruction
    
    warehouse_map
}

// fn warehouse_map_has_crate(warehouse_map: &HashMap<Position, Location>, position: &Position) -> bool {
//     if let Some(location) = warehouse_map.get(position) {
//         return location == &Location::Crate;
//     };

//     false
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 10092);
    }

    #[test]
    fn test_move_robot() {
        let warehouse_map = vec![
            (Position { x: 0, y: 0 }, Location::Obstruction),
            (Position { x: 1, y: 0 }, Location::Free),
            (Position { x: 2, y: 0 }, Location::Robot),
        ].into_iter().collect::<HashMap<Position, Location>>();
        let modified_warehouse_map = move_robot_in_direction(warehouse_map, Direction::W);

        assert_eq!(modified_warehouse_map.get(&Position { x: 0, y: 0 }).unwrap(), &Location::Obstruction);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 0 }).unwrap(), &Location::Robot);
        assert_eq!(modified_warehouse_map.get(&Position { x: 2, y: 0 }).unwrap(), &Location::Free);
    }

    #[test]
    fn test_robot_moves_a_single_crate() {
        let warehouse_map = vec![
            (Position { x: 0, y: 0 }, Location::Robot),
            (Position { x: 1, y: 0 }, Location::Crate),
            (Position { x: 2, y: 0 }, Location::Free),
            (Position { x: 3, y: 0 }, Location::Obstruction),
        ].into_iter().collect::<HashMap<Position, Location>>();
        let modified_warehouse_map = move_robot_in_direction(warehouse_map, Direction::E);

        assert_eq!(modified_warehouse_map.get(&Position { x: 0, y: 0 }).unwrap(), &Location::Free);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 0 }).unwrap(), &Location::Robot);
        assert_eq!(modified_warehouse_map.get(&Position { x: 2, y: 0 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 3, y: 0 }).unwrap(), &Location::Obstruction);
    }

    #[test]
    fn test_robot_moves_multiple_crates() {
        let warehouse_map = vec![
            (Position { x: 1, y: 0 }, Location::Obstruction),
            (Position { x: 1, y: 1 }, Location::Free),
            (Position { x: 1, y: 2 }, Location::Crate),
            (Position { x: 1, y: 3 }, Location::Crate),
            (Position { x: 1, y: 4 }, Location::Robot),
        ].into_iter().collect::<HashMap<Position, Location>>();
        let modified_warehouse_map = move_robot_in_direction(warehouse_map, Direction::N);

        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 0 }).unwrap(), &Location::Obstruction);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 1 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 2 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 3 }).unwrap(), &Location::Robot);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 4 }).unwrap(), &Location::Free);
    }

    #[test]
    fn test_robot_blocked_by_obstruction() {
        let warehouse_map = vec![
            (Position { x: 5, y: 4 }, Location::Robot),
            (Position { x: 5, y: 5 }, Location::Obstruction),
        ].into_iter().collect::<HashMap<Position, Location>>();
        let modified_warehouse_map = move_robot_in_direction(warehouse_map, Direction::S);

        assert_eq!(modified_warehouse_map.get(&Position { x: 5, y: 4 }).unwrap(), &Location::Robot);
        assert_eq!(modified_warehouse_map.get(&Position { x: 5, y: 5 }).unwrap(), &Location::Obstruction);
    }

    #[test]
    fn test_robot_and_crates_blocked_by_obstruction() {
        let warehouse_map = vec![
            (Position { x: 0, y: 0 }, Location::Obstruction),
            (Position { x: 1, y: 0 }, Location::Crate),
            (Position { x: 2, y: 0 }, Location::Crate),
            (Position { x: 3, y: 0 }, Location::Crate),
            (Position { x: 4, y: 0 }, Location::Robot),
        ].into_iter().collect::<HashMap<Position, Location>>();
        let modified_warehouse_map = move_robot_in_direction(warehouse_map, Direction::W);

        assert_eq!(modified_warehouse_map.get(&Position { x: 0, y: 0 }).unwrap(), &Location::Obstruction);
        assert_eq!(modified_warehouse_map.get(&Position { x: 1, y: 0 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 2, y: 0 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 3, y: 0 }).unwrap(), &Location::Crate);
        assert_eq!(modified_warehouse_map.get(&Position { x: 4, y: 0 }).unwrap(), &Location::Robot);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 1486930);
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 9021);
    }

    // #[test]
    // fn real_run_second() {
    //     assert_eq!(run_second(true), xx);
    // }
}
