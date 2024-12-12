use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{position_map_from_text_lines, read_from_file, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 12, None);
    let garden_map = position_map_from_text_lines(&lines, |c| c);

    get_all_regions(&garden_map)
        .iter()
        .map(|r| r.price())
        .sum()
}

pub fn run_second(is_real: bool) -> usize {
    0
}

fn get_all_regions(garden_map: &HashMap<Position, char>) -> Vec<Region> {
    let unique_square_types = garden_map
        .values()
        .unique()
        .map(|&p| p)
        .collect::<Vec<char>>();

    unique_square_types
        .iter()
        .map(|st| garden_map
            .iter()
            .filter(|(_, &c)| c == *st)
            .map(|(p, _)| *p)
            .collect::<Vec<Position>>()
        )
        .flat_map(|pv| split_into_regions(&pv))
        .collect()
}

fn split_into_regions(squares: &[Position]) -> Vec<Region> {
    let number_of_squares = squares.len();
    let mut groups = squares
        .iter()
        .map(|s| vec![*s])
        .collect::<Vec<Vec<Position>>>();

    let mut no_change_in_last_iteration = false;

    while !no_change_in_last_iteration {
        no_change_in_last_iteration = true;

        for i in 1..number_of_squares {
            for j in 0..i {
                let any_adjacent_squares = groups[j]
                    .iter()
                    .any(|g1| groups[i]
                        .iter()
                        .any(|g2| g1.is_adjacent_to(g2))
                    );

                if any_adjacent_squares {
                    groups[i] = groups[i].iter().chain(groups[j].iter()).map(|p| *p).collect::<Vec<Position>>();
                    groups[j] = Vec::new();

                    no_change_in_last_iteration = false;
                }
                
            }
        }
    }

    groups
        .iter()
        .filter(|g| !g.is_empty())
        .map(|g| Region { squares: g.iter().map(|p| *p).collect::<Vec<Position>>() })
        .collect()
}

struct Region {
    squares: Vec<Position>,
}

impl Region {
    fn area(&self) -> usize {
        self.squares.len()
    }

    fn perimeter(&self) -> usize {
        self.squares
            .iter()
            .map(|p| self.perimeter_contribution(p))
            .sum()
    }

    fn perimeter_contribution(&self, position: &Position) -> usize {
        let number_of_adjacent_squares = self.squares
            .iter()
            .filter(|s| s.is_adjacent_to(position))
            .count();

        4 - number_of_adjacent_squares
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 1930);
    }

    #[test]
    fn divided_regions() {
        let lines: Vec<String> = vec![
            "OOOOO".to_string(),
            "OXOXO".to_string(),
            "OOOOO".to_string(),
            "OXOXO".to_string(),
            "OOOOO".to_string(),
        ];

        let garden_map = position_map_from_text_lines(&lines, |c| c);

        let combined_price = get_all_regions(&garden_map)
            .iter()
            .map(|r| r.price())
            .sum::<usize>();

        assert_eq!(772, combined_price);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 1546338);
    }

    // #[test]
    // fn test_run_second() {
    //     assert_eq!(run_second(false), xx);
    // }

    // #[test]
    // fn real_run_second() {
    //     assert_eq!(run_second(true), xx);
    // }
}
