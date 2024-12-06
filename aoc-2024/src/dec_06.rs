use std::collections::HashMap;

use itertools::Itertools;

use crate::util::read_from_file;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 6, None);

    let (location_map, mut guard) = LocationMap::new(&lines);
    let mut positions = vec![guard.position];

    // todo: I would like to do this with an iterator
    while let Some(guard_on_map) = guard.get_next(&location_map) {
        positions.push(guard_on_map.position);
        guard = guard_on_map;
    }

    positions.iter().unique().count() as i32
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 6, None);

    let (location_map, initial_guard) = LocationMap::new(&lines);

    let mut guard = initial_guard.clone();
    let mut previous_guards = vec![guard];
    let mut loop_obstructions = 0;

    // // Backtrack to the edge of the map or an obstruction
    // while let Some(guard_on_map) = guard.get_last(&location_map) {
    //     previous_guards.push(guard_on_map.clone());
    //     guard = guard_on_map;
    // }

    guard = initial_guard;

    while let Some(guard_on_map) = guard.get_next(&location_map) {
        // todo: instead of backtracking, we should turn right and then carry on for as long as we can, and see if the previous guards include that value
        if previous_guards.contains(&guard_on_map.turn_right_and_go_straight(&location_map)) {
            loop_obstructions = loop_obstructions + 1;
        }

        previous_guards.push(guard_on_map.clone());
        guard = guard_on_map;
    }

    loop_obstructions
}

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

struct LocationMap {
    location_map: HashMap<(usize, usize), Location>,
}

impl LocationMap {
    fn new(lines: &[String]) -> (Self, Guard) {
        let mut maybe_guard: Option<Guard> = None;

        let location_map: HashMap<(usize, usize), Location> = lines
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    acc.insert((x, y), Location::from_char(c));

                    if let Some(direction) = Direction::from_char(c) {
                        maybe_guard = Some(Guard { position: (x, y), bearing: direction })
                    }
                }
            );

            acc
        });

        let guard = maybe_guard.unwrap();
        
        ( 
            LocationMap { location_map, }, 
            guard,
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#'|'.' => None,
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            'v' => Some(Direction::S),
            '<' => Some(Direction::W),
            _ => panic!("Invalid char: {}", c),
        }
    }

    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Guard {
    position: (usize, usize),
    bearing: Direction,
}

impl Guard {
    fn get_next(&self, location_map: &LocationMap) -> Option<Guard> {
        let (x0, y0) = self.position;
        let (dx, dy) = self.bearing.get_movement();

        let next_square = ((x0 as i32 + dx) as usize, (y0 as i32 + dy) as usize);

        // If we don't find square, it's out of bounds
        match location_map.location_map.get(&next_square)? {
            Location::Obstruction => Some( Guard { position: self.position, bearing: self.bearing.turn_right() } ),
            Location::Free => Some( Guard { position: next_square, bearing: self.bearing }),
        }
    }

    // fn get_last(&self, location_map: &LocationMap) -> Option<Guard> {
    //     let (x0, y0) = self.position;
    //     let (dx, dy) = self.bearing.get_movement();

    //     let last_square = ((x0 as i32 - dx) as usize, (y0 as i32 - dy) as usize);

    //     match location_map.location_map.get(&last_square)? {
    //         Location::Obstruction => None,
    //         Location::Free => Some( Guard { position: last_square, bearing: self.bearing } ),
    //     }
    // }

    fn turn_right_and_go_straight(&self, location_map: &LocationMap) -> Guard {
        let mut guard = self.turn_right();

        while let Some(walking_guard) = self.forward_if_possible(location_map) {
            guard = walking_guard;
        }

        guard
    }

    fn turn_right(&self) -> Guard {
        Guard { position: self.position, bearing: self.bearing.turn_right() }
    }

    fn forward_if_possible(&self, location_map: &LocationMap) -> Option<Guard> {
        let (x0, y0) = self.position;
        let (dx, dy) = self.bearing.get_movement();

        let next_square = ((x0 as i32 + dx) as usize, (y0 as i32 + dy) as usize);

        match location_map.location_map.get(&next_square)? {
            Location::Obstruction => None,
            Location::Free => Some( Guard { position: next_square, bearing: self.bearing }),
        }
    }

    // // todo: this might not be enough, what if this movement causes a turn that will later lead us to retracing our steps?
    // //       this might be an edge case, though. Maybe we should start by backtracking all the way to the edge of the map or an obstruction
    // fn turn_right_and_move(&self, location_map: &LocationMap) -> Option<Guard> {
    //     // let (x0, y0) = self.position;
    //     // let (dx1, dy1) = self.bearing.get_movement();
    //     // let (dx2, dy2) = self.bearing.turn_right().get_movement();

    //     // let new_square = ((x0 as i32 + dx1 + dx2) as usize, (y0 as i32 + dy1 + dy2) as usize);

    //     // match location_map.location_map.get(&new_square)? {
    //     //     Location::Obstruction => None, // todo: maybe we should interpret this as an obstruction
    //     //     Location::Free => Some( Guard { position: new_square, bearing: self.bearing.turn_right() }),
    //     // }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
