use std::{collections::HashMap, usize};

use itertools::Itertools;

use crate::util::{cardinal_directions, position_map_from_text_lines, read_from_file, Direction, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 16, None);

    let maze_map = position_map_from_text_lines(&lines, |c| Location::from_char(c));
    
    let intersections = intersections_from_maze_map(&maze_map);
    let wormholes = wormholes_from_intersections(&maze_map, &intersections);

    // println!("Intersections: {}", intersections.len());
    // println!("Wormholes: {}", wormholes.len());

    // let positions_in_wormhole_path = find_positions_in_wormhole_path(&intersections, &wormholes);
    
    // println!("Positions in wormhole path: {}", positions_in_wormhole_path.len());
    
    // let intersections = remove_redundant_intersections(intersections, &positions_in_wormhole_path);
    // let wormholes = shorten_wormholes(&maze_map, wormholes, &positions_in_wormhole_path);

    // println!("Intersections: {}", intersections.len());
    // println!("Wormholes: {}", wormholes.len());

    // let positions_in_wormhole_path = find_positions_in_wormhole_path(&intersections, &wormholes);
    
    // println!("Positions in wormhole path: {}", positions_in_wormhole_path.len());
    
    // let intersections = remove_redundant_intersections(intersections, &positions_in_wormhole_path);
    // let wormholes = shorten_wormholes(&maze_map, wormholes, &positions_in_wormhole_path);

    // println!("Intersections: {}", intersections.len());
    // println!("Wormholes: {}", wormholes.len());

    // let positions_in_wormhole_path = find_positions_in_wormhole_path(&intersections, &wormholes);
    
    // println!("Positions in wormhole path: {}", positions_in_wormhole_path.len());
    
    // let intersections = remove_redundant_intersections(intersections, &positions_in_wormhole_path);
    // let wormholes = shorten_wormholes(&maze_map, wormholes, &positions_in_wormhole_path);

    println!("Intersections: {}", intersections.len());
    println!("Wormholes: {}", wormholes.len());

    // todo: find wormhole pairs with same start and stop positions, take the better pair
    
    let start_position = maze_map.iter().find(|(_, l)| l.is_start_tile()).map(|(p, _)| *p).unwrap();
    let initial_node = IntersectionNode::new(&start_position);

    initial_node.get_optimal_cost(&maze_map, &intersections, &wormholes).unwrap()

    // let path_tree = PathNode2::new(start_position);

    // // todo: list of locations of intersections
    // // todo: use the list and the maze map to define wormholes

    // path_tree.get_optimal_cost(&maze_map, &wormholes).unwrap()
    // 0
}

fn intersections_from_maze_map(maze_map: &HashMap<Position, Location>) -> Vec<Intersection> {
    maze_map
        .iter()
        .filter(|(_, l)| l.is_free())
        .filter_map(|(p, _)| get_intersection(&maze_map, p))
        .collect::<Vec<Intersection>>()
}

fn wormholes_from_intersections(maze_map: &HashMap<Position, Location>, intersections: &[Intersection]) -> Vec<Wormhole> {
    intersections
        .iter()
        .flat_map(|i| i.get_possible_states())
        .filter_map(|rs| Wormhole::from_state(rs, &maze_map, &intersections))
        .collect::<Vec<Wormhole>>()
}

fn find_positions_in_wormhole_path(intersections: &[Intersection], wormholes: &[Wormhole]) -> Vec<Position> {
    intersections
        .iter()
        .filter(|i| wormholes
            .iter()
            .filter(|w| w.one.position == i.position)
            .count() == 2
        ).map(|i| i.position)
        .collect::<Vec<Position>>()
}

fn remove_redundant_intersections(intersections: Vec<Intersection>, positions: &[Position]) -> Vec<Intersection> {
    intersections
        .into_iter()
        .filter(|i| !positions.contains(&i.position))
        .collect::<Vec<Intersection>>()
}

fn shorten_wormholes(
    maze_map: &HashMap<Position, Location>, 
    wormholes: Vec<Wormhole>, 
    positions: &[Position]
) -> Vec<Wormhole> {
    let mut wormholes_result = wormholes
        .iter()
        .filter(|w| positions
            .iter()
            .all(|p| w.one.position != *p && w.other.position != *p)
        )
        .cloned()
        .collect::<Vec<Wormhole>>();

    for p in positions {
        let location = maze_map.get(p).unwrap();

        if location.is_start_tile() || location.is_end_tile() {
            wormholes_result.append(&mut wormholes.iter().filter(|&w| w.one.position == *p || w.other.position == *p).cloned().collect::<Vec<Wormhole>>());
            continue;
        }
        
        let (wormholes_starting_at_position, wormholes_ending_at_position): (Vec<Wormhole>, Vec<Wormhole>) = wormholes
            .iter()
            .filter(|&w| w.one.position == *p || w.other.position == *p)
            .partition(|w| w.one.position == *p);

        // for w in wormholes_ending_at_position.iter() {
        //     println!("{:?}", w);
        // }

        // for w in wormholes_starting_at_position.iter() {
        //     println!("{:?}", w);
        // }

        let we0 = wormholes_ending_at_position[0];
        let ws0 = wormholes_starting_at_position.iter().find(|w| w.other.position != we0.one.position);

        let we1 = wormholes_ending_at_position[1];
        let ws1 = wormholes_starting_at_position.iter().find(|w| w.other.position != we1.one.position);
        
        if ws0.is_none() || ws1.is_none() {
            // wormholes_result.append(&mut wormholes.iter().filter(|&w| w.one.position == *p || w.other.position == *p).cloned().collect::<Vec<Wormhole>>());
            continue;
        }

        let ws0 = ws0.unwrap().clone();
        let ws1 = ws1.unwrap().clone();

        let w0 = Wormhole { one: we0.one, other: ws0.other, cost: we0.cost + ws0.cost };
        let w1 = Wormhole { one: we1.one, other: ws1.other, cost: we1.cost + ws1.cost };

        wormholes_result.push(w0);
        wormholes_result.push(w1);
    }

    wormholes_result
}

pub fn run_second(is_real: bool) -> usize {
    0
}

fn get_intersection(maze_map: &HashMap<Position, Location>, position: &Position) -> Option<Intersection> {
    let location = maze_map.get(position).unwrap();
    
    let n_free = maze_map.get(&position.walk_in_direction(&Direction::N).unwrap()).unwrap().is_free();
    let e_free = maze_map.get(&position.walk_in_direction(&Direction::E).unwrap()).unwrap().is_free();
    let s_free = maze_map.get(&position.walk_in_direction(&Direction::S).unwrap()).unwrap().is_free();
    let w_free = maze_map.get(&position.walk_in_direction(&Direction::W).unwrap()).unwrap().is_free();
    
    let total_free = vec![n_free, e_free, s_free, w_free].iter().filter(|x| **x).count();

    if total_free > 2 || location.is_start_tile() || location.is_end_tile() {
        return Some(Intersection { position: position.clone(), n_free, e_free, s_free, w_free });
    }

    None
}

#[derive(Debug, Clone)]
struct Intersection {
    position: Position,
    n_free: bool,
    e_free: bool,
    s_free: bool,
    w_free: bool,
}

impl Intersection {
    fn get_possible_states(&self) -> Vec<ReindeerState> {
        let mut possible_states = Vec::new();

        if self.n_free { possible_states.push(ReindeerState { position: self.position.clone(), direction: Direction::N }); }
        if self.e_free { possible_states.push(ReindeerState { position: self.position.clone(), direction: Direction::E }); }
        if self.s_free { possible_states.push(ReindeerState { position: self.position.clone(), direction: Direction::S }); }
        if self.w_free { possible_states.push(ReindeerState { position: self.position.clone(), direction: Direction::W }); }

        possible_states
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct ReindeerState {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
struct Wormhole {
    one: ReindeerState,
    other: ReindeerState,
    cost: usize,
}

impl Wormhole {
    fn from_state(
        state: ReindeerState, 
        maze_map: &HashMap<Position, Location>, 
        intersections: &[Intersection]
    ) -> Option<Self> {
        let mut current_position = state.position.walk_in_direction(&state.direction).unwrap();
        let mut current_direction = state.direction;
        let mut current_cost = 1;

        loop {
            if let Some(intersection) = intersections.iter().find(|i| i.position == current_position) {
                return Some(Self { 
                    one: state, 
                    other: ReindeerState { 
                        position: intersection.position, 
                        direction: current_direction 
                    }, 
                    cost: current_cost 
                })
            }

            let forward_position = current_position.walk_in_direction(&current_direction).unwrap();
            if maze_map.get(&forward_position).unwrap().is_free() {
                current_position = forward_position;
                current_cost = current_cost + 1;
                continue;
            }

            let left_position = current_position.walk_in_direction(&current_direction.turn_left()).unwrap();
            if maze_map.get(&left_position).unwrap().is_free() {
                current_position = left_position;
                current_direction = current_direction.turn_left();
                current_cost = current_cost + 1001;
                continue;
            }

            let right_position = current_position.walk_in_direction(&current_direction.turn_right()).unwrap();
            if maze_map.get(&right_position).unwrap().is_free() {
                current_position = right_position;
                current_direction = current_direction.turn_right();
                current_cost = current_cost + 1001;
                continue;
            }

            return None;
        }
    }

    fn travel_to_other_side(&self, state: &ReindeerState) -> Option<(ReindeerState, usize)> {
        if state == &self.one {
            Some((self.other.clone(), self.cost))
        } else {
            None
        }
    }
}

enum Location {
    Obstruction,
    Free,
    StartTile,
    EndTile,
}

impl Location {
    fn from_char(c: char) -> Location {
        match c {
            '#' => Location::Obstruction,
            '.' => Location::Free,
            'S' => Location::StartTile,
            'E' => Location::EndTile,
            _ => panic!("Unexpected character")
        }
    }

    fn is_start_tile(&self) -> bool {
        match self {
            Location::StartTile => true,
            _ => false,
        }
    }

    fn is_end_tile(&self) -> bool {
        match self {
            Location::EndTile => true,
            _ => false,
        }
    }

    fn is_free(&self) -> bool {
        match self {
            Location::Free | Location::StartTile | Location::EndTile => true,
            _ => false,
        }
    }
}

// todo: start by finding paths from start tile to intersections
struct IntersectionNode<'a> {
    state: ReindeerState,
    cost_from_start: usize,
    parent_node: Option<&'a IntersectionNode<'a>>
}

impl<'a> IntersectionNode<'a> {
    fn new(position: &Position) -> Self {
        Self { 
            state: ReindeerState { 
                position: position.clone(), 
                direction: Direction::E 
            }, 
            cost_from_start: 0, 
            parent_node: None 
        }
    }

    fn new_from_direction(parent_node: &'a IntersectionNode, bearing: Bearing, wormholes: &[Wormhole]) -> Option<Self> {
        let parent_state = parent_node.state;
        let state = match bearing {
            Bearing::Forward => parent_state,
            Bearing::Left => ReindeerState { position: parent_state.position, direction: parent_state.direction.turn_left() },
            Bearing::Right => ReindeerState { position: parent_state.position, direction: parent_state.direction.turn_right() },
        };

        if let Some((new_state, wormhole_cost)) = wormholes.iter().filter_map(|wh| wh.travel_to_other_side(&state)).next() {
            if ancestor_has_been_here(parent_node, &new_state.position) {
                return None;
            }

            return Some(
                IntersectionNode { 
                    state: new_state, 
                    cost_from_start: parent_node.cost_from_start + wormhole_cost + bearing.cost(), 
                    parent_node: Some(parent_node) 
                }
            )
        };
        
        None
    }

    fn get_optimal_cost(
        &self, 
        maze_map: &HashMap<Position, Location>, 
        intersections: &[Intersection], 
        wormholes: &[Wormhole]
    ) -> Option<usize> {
        let location = maze_map.get(&self.state.position).unwrap();
        
        if location.is_end_tile() {
            return Some(self.cost_from_start);
        }

        bearings()
            .into_iter()
            .filter_map(|b| IntersectionNode::new_from_direction(self, b, wormholes))
            .filter_map(|n| n.get_optimal_cost(maze_map, intersections, wormholes))
            .min()
    }
}

enum Bearing {
    Forward,
    Left,
    Right
}

impl Bearing {
    fn cost(&self) -> usize {
        match self {
            Bearing::Forward => 0,
            Bearing::Left | Bearing::Right => 1000,
        }
    }
}

fn bearings() -> Vec<Bearing> {
    vec![Bearing::Forward, Bearing::Left, Bearing::Right]
}

fn ancestor_has_been_here(mut parent_node: &IntersectionNode, position: &Position) -> bool {
    loop {
        if &parent_node.state.position == position {
            return true;
        }

        parent_node = match parent_node.parent_node {
            Some(pn) => pn,
            None => return false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 7036);
    }

    // #[test]
    // fn real_run_first() {
    //     assert_eq!(run_first(true), xx);
    // }

    // #[test]
    // fn test_run_second() {
    //     assert_eq!(run_second(false), xx);
    // }

    // #[test]
    // fn real_run_second() {
    //     assert_eq!(run_second(true), xx);
    // }
}
