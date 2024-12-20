use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{cardinal_directions, position_map_from_text_lines, read_from_file, Direction, Position};

pub fn run_first(is_real: bool) -> usize {
    let trail_map = get_trail_map(is_real);

    calculate_from_trail_map(&trail_map, |trailhead| trailhead.calculate_score())
}

pub fn run_second(is_real: bool) -> usize {
    let trail_map = get_trail_map(is_real);

    calculate_from_trail_map(&trail_map, |trailhead| trailhead.calculate_rating())
}

fn get_trail_map(is_real: bool) -> HashMap<Position, usize> {
    let lines = read_from_file(is_real, 10, None);
    let parse_from_char= |c: char| c.to_digit(10).unwrap() as usize;

    position_map_from_text_lines(&lines, parse_from_char)
}

fn calculate_from_trail_map(trail_map: &HashMap<Position, usize>, calculator: fn(Trailhead) -> usize) -> usize {
    trail_map
        .iter()
        .filter(|(_, h)| **h == TRAILHEAD_HEIGHT)
        .map(|(p, _)| Trailhead::new(&trail_map, *p) )
        .map(|th| calculator(th))
        .sum::<usize>()
}

const TRAILHEAD_HEIGHT: usize = 0;
const SUMMIT_HEIGHT: usize = 9;

struct Trailhead {
    tile: Tile,
}

impl Trailhead {
    fn new(trail_map: &HashMap<Position, usize>, position: Position) -> Self {
        let children = Tile::populate_children(trail_map, &position, None);
        let tile = Tile::GradualIncline(children);

        Self { tile, }
    }

    fn calculate_score(&self) -> usize {
        self.tile.summit_positions()
            .iter()
            .unique()
            .count()
    }

    fn calculate_rating(&self) -> usize {
        self.tile.summit_positions()
            .iter()
            .count()
    }
}

enum Tile {
    GradualIncline(Vec<Box<Tile>>),
    Summit(Position),
}

impl Tile {
    fn populate(
        trail_map: &HashMap<Position, usize>, 
        parent_position: &Position, 
        hiking_direction: Direction,
    ) -> Option<Self> {
        let movement_from_parent_tile = hiking_direction.get_movement();
        let tile_position = parent_position.new_position(&movement_from_parent_tile)?;

        let parent_tile_height = *trail_map.get(parent_position)?;
        let tile_height = *trail_map.get(&tile_position)?;

        if (tile_height as i32) - (parent_tile_height as i32) != 1 {
            return None;
        }

        if tile_height == SUMMIT_HEIGHT {
            return Some(Self::Summit(tile_position))
        }

        let banned_direction = &hiking_direction.reverse();
        let children = Tile::populate_children(trail_map, &tile_position, Some(banned_direction));

        Some(Self::GradualIncline(children))
    }
    
    fn populate_children(
        trail_map: &HashMap<Position, usize>, 
        position: &Position,
        banned_direction: Option<&Direction>,
    ) -> Vec<Box<Self>> {
        cardinal_directions()
            .into_iter()
            .filter(|d| {
                match banned_direction {
                    Some(bd) => d != bd,
                    None => true,
                }
            })
            .filter_map(|d| Tile::populate(trail_map, position, d))
            .map(|t| Box::new(t))
            .collect()
    }

    fn summit_positions(&self) -> Vec<Position> {
        match self {
            Tile::Summit(position) => vec![*position],
            Tile::GradualIncline(child_tiles) => {
                child_tiles
                    .iter()
                    .flat_map(|ct| ct.summit_positions())
                    .collect::<Vec<Position>>()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 36);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 557);
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 81);
    }

    #[test]
    fn real_run_second() {
        assert_eq!(run_second(true), 1062);
    }
}
