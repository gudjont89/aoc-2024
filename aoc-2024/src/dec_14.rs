use crate::util::{read_from_file, Dimensions, Position, Separation};

pub fn run_first(is_real: bool) -> usize {
    let map_dimensions: Dimensions = match is_real { 
        true => Dimensions { width: 101, height: 103 },
        false => Dimensions { width: 11, height: 7},
    };
    const NUMBER_OF_MOVEMENTS: usize = 100;

    let lines = read_from_file(is_real, 14, None);
    let moved_robots = lines
        .iter()
        .filter_map(|l| parse_robot_from_line(l))
        .map(|r| r.move_wrapping_around(&map_dimensions, NUMBER_OF_MOVEMENTS))
        .collect::<Vec<Robot>>();

    let quadrants = Quadrant::from_dimensions(&map_dimensions).unwrap();
    quadrants
        .iter()
        .map(|q| {
            moved_robots.iter().filter(|mr| q.includes(&mr.position)).count()
        })
        .fold(1, |a, qc| a*qc)
}

pub fn run_second(is_real: bool) -> usize {
    0
}

struct Robot {
    position: Position,
    velocity: Separation,
}

impl Robot {
    fn move_wrapping_around(self, dimensions: &Dimensions, number_of_movements: usize) -> Self {
        let position = self.position.move_wrapping_around(
            &self.velocity.multiply(number_of_movements), 
            dimensions
        );

        Robot { position, velocity: self.velocity }
    }
}

fn parse_robot_from_line(line: &str) -> Option<Robot> {
    let rgx = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").ok()?;
    let (_, [x, y, dx, dy]) = rgx.captures(line)?.extract();

    let x = x.parse::<usize>().ok()?;
    let y = y.parse::<usize>().ok()?;
    let dx = dx.parse::<i32>().ok()?;
    let dy = dy.parse::<i32>().ok()?;

    let position = Position { x, y };
    let velocity = Separation { dx, dy };

    Some(Robot { position, velocity })
}

struct Quadrant {
    upper_left: Position,
    lower_right: Position,
}

impl Quadrant {
    fn includes(&self, position: &Position) -> bool {
        return self.upper_left.x <= position.x 
            && self.upper_left.y <= position.y
            && position.x <= self.lower_right.x
            && position.y <= self.lower_right.y
    }
}

impl Quadrant {
    fn from_dimensions(dimensions: &Dimensions) -> Option<Vec<Quadrant>> {
        if dimensions.width % 2 == 0 || dimensions.height % 2 == 0 {
            return None;
        }

        let quadrant_width = (dimensions.width - 1) / 2;
        let quadrant_height = (dimensions.height - 1) / 2;

        let nw_quadrant = Quadrant {
            upper_left: Position { x: 0, y: 0 },
            lower_right: Position { x: quadrant_width - 1, y: quadrant_height - 1 },
        };

        let ne_quadrant = Quadrant {
            upper_left: Position { x: quadrant_width + 1, y: 0 },
            lower_right: Position { x: 2 * quadrant_width, y: quadrant_height - 1 },
        };

        let sw_quadrant = Quadrant {
            upper_left: Position { x: 0, y: quadrant_height + 1 },
            lower_right: Position { x: quadrant_width - 1, y: 2 * quadrant_height },
        };

        let se_quadrant = Quadrant {
            upper_left: Position { x: quadrant_width + 1, y: quadrant_height + 1 },
            lower_right: Position { x: 2 * quadrant_width, y: 2 * quadrant_height },
        };

        Some(vec![nw_quadrant, ne_quadrant, sw_quadrant, se_quadrant])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 12);
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
