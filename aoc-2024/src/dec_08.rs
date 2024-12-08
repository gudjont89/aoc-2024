use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{get_position_map_dimensions, position_map_from_text_lines, positions_on_map_with_value, read_from_file, Dimensions, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 8, None);

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
                .flat_map(|ac| ac.get_antinode_positions_without_regard_to_resonant_harmonics(&antenna_map_dimensions))
                .collect::<Vec<Position>>()
        )
        .unique()
        .count()
}

pub fn run_second(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 8, None);

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
                .flat_map(|ac| ac.get_antinode_positions_with_regard_to_resonant_harmonics(&antenna_map_dimensions))
                .collect::<Vec<Position>>()
        )
        .unique()
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AntennaLocation {
    Free,
    Antenna(char),
}

impl AntennaLocation {
    fn parse_from_char(c: char) -> Self {
        if c.is_alphanumeric() {
            return AntennaLocation::Antenna(c);
        }

        if c == '.' {
            return AntennaLocation::Free;
        }

        panic!("Unknown character");
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
    fn get_antinode_positions_without_regard_to_resonant_harmonics(
        &self, 
        map_dimensions: &Dimensions,
    ) -> Vec<Position> {
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
    
    fn get_antinode_positions_with_regard_to_resonant_harmonics(
        &self, 
        map_dimensions: &Dimensions,
    ) -> Vec<Position> {
        let mut antinodes = vec![self.first, self.second];

        let separation = self.first.separated_from_by(&self.second);
        let mut positive_antinode = self.first;
        let mut negative_antinode = self.second;

        while let Some(new_positive_antinode) = positive_antinode
            .new_position(&separation)
            .filter(|p| map_dimensions.includes(p)) {
                antinodes.push(new_positive_antinode);
                positive_antinode = new_positive_antinode;
            }

        while let Some(new_negative_antinode) = negative_antinode
            .new_position(&separation.negative())
            .filter(|p| map_dimensions.includes(p)) {
                antinodes.push(new_negative_antinode);
                negative_antinode = new_negative_antinode;
            }

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
    fn test_antenna_couple_get_antinode_positions_without_regard_to_resonant_harmonics() {
        let antenna_couple = AntennaCouple {
            first: Position { x: 4, y: 3 },
            second: Position { x: 5, y: 5 }
        };

        let expected_result = vec![
            Position { x: 3, y: 1 },
            Position { x: 6, y: 7 }
        ];
        let result = antenna_couple.get_antinode_positions_without_regard_to_resonant_harmonics(
            &Dimensions { width: 12, height: 12 },
        );
        
        assert!(result.contains(&expected_result[0]));
        assert!(result.contains(&expected_result[1]));
    }

    #[test]
    fn test_antenna_couple_get_antinode_positions_with_regard_to_resonant_harmonics() {
        let antenna_couple = AntennaCouple {
            first: Position { x: 4, y: 3 },
            second: Position { x: 5, y: 5 }
        };

        let expected_result = vec![
            Position { x: 3, y: 1 },
            Position { x: 4, y: 3 },
            Position { x: 5, y: 5 },
            Position { x: 6, y: 7 },
            Position { x: 7, y: 9 },
            Position { x: 8, y: 11 },
        ];
        let result = antenna_couple.get_antinode_positions_with_regard_to_resonant_harmonics(
            &Dimensions { width: 12, height: 12 },
        );
        
        assert!(result.contains(&expected_result[0]));
        assert!(result.contains(&expected_result[1]));
        assert!(result.contains(&expected_result[2]));
        assert!(result.contains(&expected_result[3]));
        assert!(result.contains(&expected_result[4]));
        assert!(result.contains(&expected_result[5]));
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 409)
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 34);
    }
}
