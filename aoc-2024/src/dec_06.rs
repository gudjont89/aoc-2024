use std::collections::HashMap;

use itertools::Itertools;

use crate::util::read_from_file;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 6, None);

    let (location_map, mut guard) = LocationMap::new(&lines);
    let mut positions = vec![guard.position];

    while let Some(guard_on_map) = guard.get_next(&location_map) {
        positions.push(guard_on_map.position);
        guard = guard_on_map;
    }

    positions.iter().unique().count() as i32
}

// We could possibly solve this by looking at all obstructions
// For each
//  - Start on the left side of the obstruction and go down
//  - Start underneath the obstruction and go right
//  - Start on the right side of the obstruction and go up
//  - Start over the obstruction and go left
// If we can keep going until reach the same place without turning more than 3 times, we have a candidate
// Make a list of the initial guard (position and bearing) for the first type of candidate
// If keeping going one more time results in a turn to the right, we have a different type of candidate
// For each candidate, keep track of the loop route
// Run through the first part of the problem, this time keeping a list of guards instead of positions
// If one of the guards matches an initial guard for the first type of candidate, we have a loop
// 

// We have to look at all squares, not just the obstructions
// An empty square would result in the first type of candidate
// An obstruction would result in the second type of candidate
pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 6, None);

    let (location_map, initial_guard) = LocationMap::new(&lines);

    // let three_obstruction_candidates: Vec<Guard> = Vec::new();
    // let four_obstruction_candidates = Vec::new();

    let three_obstruction_candidates = location_map.location_map
        .iter()
        .filter(|(k, v)| v.is_free())
        .flat_map(|(k, _)| k.guards_moving_from_obstruction())
        ;


    0
}

struct LocationMap {
    location_map: HashMap<Position, Location>,
}

impl LocationMap {
    fn new(lines: &[String]) -> (Self, Guard) {
        let mut maybe_guard: Option<Guard> = None;

        let location_map: HashMap<Position, Location> = lines
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let position = Position { x, y };
                    acc.insert(position, Location::from_char(c));

                    if let Some(direction) = Direction::from_char(c) {
                        maybe_guard = Some(Guard { position, bearing: direction })
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

    fn is_obstruction(&self) -> bool {
        match self {
            Location::Obstruction => true,
            Location::Free => false,
        }
    }

    fn is_free(&self) -> bool {
        match self {
            Location::Obstruction => false,
            Location::Free => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_by(&self, movement: Movement) -> Option<Self> {
        let new_x = (self.x as i32) + movement.dx;
        let new_y = (self.y as i32) + movement.dy;

        if new_x < 0 || new_y < 0 {
            return None;
        }

        Some(Position { 
            x: ((self.x as i32) + movement.dx) as usize, 
            y: ((self.y as i32) + movement.dy) as usize, 
        })
    }

    fn on_top_of(&self) -> Option<Self> {
        self.move_by(Direction::N.get_movement())
    }

    fn to_the_right(&self) -> Option<Self> {
        self.move_by(Direction::E.get_movement())
    }

    fn underneath(&self) -> Option<Self> {
        self.move_by(Direction::S.get_movement())
    }

    fn to_the_left(&self) -> Option<Self> {
        self.move_by(Direction::W.get_movement())
    }

    fn guards_moving_from_obstruction(&self) -> Vec<Guard> {
        let mut guards = Vec::new();

        if let Some(on_top_of) = self.on_top_of() {
            guards.push(Guard { position: on_top_of, bearing: Direction::W });
        }
        
        if let Some(to_the_right) = self.to_the_right() {
            guards.push(Guard { position: to_the_right, bearing: Direction::N });
        }

        if let Some(underneath) = self.underneath() {
            guards.push(Guard { position: underneath, bearing: Direction::E });
        }

        if let Some(to_the_left) = self.to_the_left() {
            guards.push(Guard { position: to_the_left, bearing: Direction::S });
        }

        guards
    }
}

struct Movement {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

    fn get_movement(&self) -> Movement {
        match self {
            Direction::N => Movement { dx: 0, dy: -1 },
            Direction::E => Movement { dx: 1, dy: 0 },
            Direction::S => Movement { dx: 0, dy: 1 },
            Direction::W => Movement { dx: -1, dy: 0 },
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

#[derive(Clone, Copy, PartialEq, Debug)]
struct Guard {
    position: Position,
    bearing: Direction,
}

impl Guard {
    fn get_next(
        &self, 
        location_map: &LocationMap,
    ) -> Option<Guard> {
        let next_square = self.position.move_by(self.bearing.get_movement())?;

        // If we don't find square, it's out of bounds
        match location_map.location_map.get(&next_square)? {
            Location::Obstruction => Some( Guard { position: self.position, bearing: self.bearing.turn_right() } ),
            Location::Free => Some( Guard { position: next_square, bearing: self.bearing }),
        }
    }
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
