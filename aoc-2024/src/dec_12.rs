use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{cardinal_directions, position_map_from_text_lines, read_from_file, Direction, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 12, None);
    let garden_map = position_map_from_text_lines(&lines, |c| c);

    get_all_regions(&garden_map)
        .iter()
        .map(|r| r.price())
        .sum()
}

pub fn run_second(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 12, None);
    let garden_map = position_map_from_text_lines(&lines, |c| c);

    get_all_regions(&garden_map)
        .iter()
        .map(|r| r.discount_price())
        .sum()
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
    find_adjoining_groups_in_square_collections(squares)
        .iter()
        .filter(|g| !g.is_empty())
        .map(|g| Region { squares: g.iter().map(|p| *p).collect::<Vec<Position>>() })
        .collect()
}

fn find_adjoining_groups_in_square_collections(squares: &[Position]) -> Vec<Vec<Position>> {
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

    let mut result = Vec::new();

    for group in groups {
        if !group.is_empty() {
            result.push(group);
        }
    }

    result
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

    fn number_of_sides(&self) -> usize {
        let all_side_contributions = self.squares
            .iter()
            .flat_map(|s| self.side_contributions(s))
            .collect::<Vec<SideContribution>>();

        let columns = self.squares
            .iter()
            .map(|s| s.x)
            .unique()
            .collect::<Vec<usize>>();
        let rows = self.squares
            .iter()
            .map(|s| s.y)
            .unique()
            .collect::<Vec<usize>>();

        let mut number_of_sides = 0;

        for column in columns {
            for direction in vec![Direction::W, Direction::E] {
                let sides = Side::from_side_contributions(&all_side_contributions, column, &direction);
                number_of_sides = number_of_sides + sides.len();
            }
        }
        
        for row in rows {
            for direction in vec![Direction::N, Direction::S] {
                let sides = Side::from_side_contributions(&all_side_contributions, row, &direction);
                number_of_sides = number_of_sides + sides.len();
            }
        }

        number_of_sides
    }

    fn side_contributions(&self, position: &Position) -> Vec<SideContribution> {
        cardinal_directions()
            .iter()
            .filter_map(|d| {
                if let Some(adjacent_square) = position.walk_in_direction(d) {
                    if self.squares.contains(&adjacent_square) {
                        return None;
                    }
                }

                return Some(SideContribution { position: *position, side: *d });
            })
            .collect()
    }

    fn discount_price(&self) -> usize {
        self.area() * self.number_of_sides()
    }
}

struct Side {
    positions: Vec<Position>,
    direction: Direction,
}

impl Side {
    fn from_side_contributions(
        side_contributions: &[SideContribution],
        order: usize,
        direction: &Direction,
    ) -> Vec<Side> {
        let position_filtering_closure: Box<dyn Fn(Position) -> bool> = match direction {
            Direction::N | Direction::S => Box::new(|p: Position| p.y == order),
            Direction::E | Direction::W => Box::new(|p: Position| p.x == order),
            _ => panic!("no"),
        };

        let matching_side_contribution_positions = side_contributions
            .iter()
            .filter(|sc| sc.side == *direction && position_filtering_closure(sc.position))
            .map(|sc| sc.position)
            .collect::<Vec<Position>>();
        
        find_adjoining_groups_in_square_collections(&matching_side_contribution_positions)
            .iter()
            .map(|g| Side { positions: g.to_vec(), direction: *direction })
            .collect::<Vec<Side>>()
    }
}

#[derive(Debug)]
struct SideContribution {
    position: Position,
    side: Direction,
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

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 1206);
    }

    #[test]
    fn test_number_of_sides() {
        let region = Region { squares: vec![Position { x: 0, y: 0 }] };
        let result = region.number_of_sides();

        assert_eq!(result, 4);
    }

    #[test]
    fn real_run_second() {
        assert_eq!(run_second(true), 978590);
    }
}
