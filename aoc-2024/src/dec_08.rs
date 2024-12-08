use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{get_position_map_dimensions, position_map_from_text_lines, positions_on_map_with_value, read_from_file, Dimensions, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 8, None);

    // todo: could we possibly define a trait for string iterators?
    let antenna_map = position_map_from_text_lines(
        &lines, 
        |c| AntennaLocation::parse_from_char(c));

    let antenna_map_dimensions = get_position_map_dimensions(&antenna_map).unwrap();
    let unique_antenna_frequencies = antenna_map
        .values()
        .filter_map(|p| p.frequency())
        .unique()
        .collect::<Vec<char>>();
    
    unique_antenna_frequencies
        .iter()
        .flat_map(|f| 
            positions_on_map_with_value(&antenna_map, AntennaLocation::Antenna(*f))
                .iter()
                .combinations(2)
                .map(|x| AntennaCouple { first: *x[0], second: *x[1]})
                .flat_map(|ac| ac.get_antinode_positions(&antenna_map_dimensions))
                .collect::<Vec<Position>>()
        )
        .unique()
        .count()
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 8, None);

    0
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AntennaLocation {
    Free,
    Antenna(char),
}

impl AntennaLocation {
    fn parse_from_char(c: char) -> Self {
        match c.is_alphanumeric() {
            true => AntennaLocation::Antenna(c),
            false => AntennaLocation::Free,
        }
    }

    fn frequency(&self) -> Option<char> {
        match self {
            AntennaLocation::Free => None,
            AntennaLocation::Antenna(c) => Some(*c),
        }
    }
}

struct AntennaCouple {
    first: Position,
    second: Position,
}

impl AntennaCouple {
    fn get_antinode_positions(&self, map_dimensions: &Dimensions) -> Vec<Position> {
        let separation = self.first.separated_from_by(&self.second);

        let first_antinode = self
            .first
            .new_position(&separation)
            .filter(|p| map_dimensions.includes(p));
        let second_antinode = self
            .second
            .new_position(&separation.negative())
            .filter(|p| map_dimensions.includes(p));

        let mut antinodes = Vec::new();

        if let Some(first_antinode) = first_antinode { antinodes.push(first_antinode); }
        if let Some(second_antinode) = second_antinode { antinodes.push(second_antinode); }

        antinodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 14);
    }

    #[test]
    fn test_antenna_couple_get_antinode_positions() {
        let antenna_couple = AntennaCouple {
            first: Position { x: 4, y: 3 },
            second: Position { x: 5, y: 5 }
        };

        let expected_result = vec![
            Position { x: 3, y: 1 },
            Position { x: 6, y: 7 }
        ];
        let result = antenna_couple.get_antinode_positions(
            &Dimensions { width: 11, height: 11 }
        );
        
        assert!(result.contains(&expected_result[0]));
        assert!(result.contains(&expected_result[1]));
    }

    // #[test]
    // fn real_run_first() {
    //     assert_eq!(run_first(true), 42283209483350)
    // }

    // #[test]
    // fn test_run_second() {
    //     assert_eq!(run_second(false), 11387);
    // }
}
