use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{position_and_object_from_text_lines, position_map_from_text_lines, positions_on_map_with_value, read_from_file, Direction, Position, Separation};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 6, None);

    let location_map = position_map_from_text_lines(&lines, |c| Location::from_char(c));
    let (position, direction) = position_and_object_from_text_lines(&lines, |c| Direction::from_char(c)).unwrap();
    let guard = Guard { position, bearing: direction };

    count_number_of_unique_positions_on_way_out(
        &guard, 
        &location_map
    ).unwrap()
}

pub fn run_second(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 6, None);

    let mut location_map = position_map_from_text_lines(&lines, |c| Location::from_char(c));
    let (position, direction) = position_and_object_from_text_lines(&lines, |c| Direction::from_char(c)).unwrap();
    let guard = Guard { position, bearing: direction };

    let mut obstruction_loops = 0;

    let free_positions = positions_on_map_with_value(
        &location_map, 
        Location::Free
    );

    for free_position in free_positions {
        // We can't place an obstruction in the guard's initial position
        if free_position == guard.position {
            continue;
        }

        location_map.insert(free_position, Location::Obstruction).unwrap();

        if let None = count_number_of_unique_positions_on_way_out(&guard, &location_map) {
            obstruction_loops = obstruction_loops + 1;
        };

        location_map.insert(free_position, Location::Free).unwrap();
    }

    obstruction_loops
}

fn count_number_of_unique_positions_on_way_out(
    guard: &Guard,
    location_map: &HashMap<Position, Location>,
) -> Option<usize> {
    let mut guard = guard.clone();
    let mut guards = vec![guard];
    let mut guard_counter = 0;

    while let Some(guard_on_map) = guard.get_next(&location_map) {
        if guard_counter % 2000 == 0 && guards.contains(&guard_on_map) {
            return None;
        }

        guards.push(guard_on_map);
        guard_counter = guard_counter + 1;
        guard = guard_on_map;
    }

    Some(guards
        .iter()
        .map(|g| g.position)
        .unique()
        .count())
}

#[derive(PartialEq)]
enum Location {
    Obstruction,
    Free,
}

impl Location {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Location::Obstruction,
            '.'|'>'|'<'|'^'|'v' => Location::Free,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Guard {
    position: Position,
    bearing: Direction,
}

impl Guard {
    fn get_next(
        &self, 
        location_map: &HashMap<Position, Location>,
    ) -> Option<Guard> {
        let next_square = self.position.new_position(&self.bearing.get_movement())?;

        // If we don't find square, it's out of bounds
        match location_map.get(&next_square)? {
            Location::Obstruction => Some( Guard { position: self.position, bearing: self.bearing.turn_right() } ),
            Location::Free => Some( Guard { position: next_square, bearing: self.bearing }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 41);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 5453)
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 6);
    }
}
