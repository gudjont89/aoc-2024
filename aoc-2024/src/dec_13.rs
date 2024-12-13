use crate::util::{read_from_file, Position};

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 13, None);

    parse_prize_recipes(lines)
        .iter()
        .filter_map(|pr| pr.get_optimal_attempt())
        .map(|a| a.tokens())
        .sum()
}

pub fn run_second(is_real: bool) -> usize {
    0
}

fn parse_prize_recipes(lines: Vec<String>) -> Vec<PrizeRecipe> {
    let mut line_iterator = lines.iter().peekable();
    let mut prize_recipes = Vec::new();

    loop {
        let prize_recipe = match parse_prize_recipe(&mut line_iterator) {
            Some(pr) => pr,
            None => break,
        };
        prize_recipes.push(prize_recipe);

        if let None = line_iterator.next() {
            break;
        }
    }

    prize_recipes
}

fn parse_prize_recipe<'a>(i_tmp: &mut impl Iterator<Item = &'a String>) -> Option<PrizeRecipe> {
    let a_movement = parse_button_line(i_tmp.next()?)?;
    let b_movement = parse_button_line(i_tmp.next()?)?;
    let prize_position = parse_prize_position(i_tmp.next()?)?;

    Some(PrizeRecipe { a_movement, b_movement, prize_position })
}

struct Attempt {
    a: usize,
    b: usize,
}

impl Attempt {
    fn tokens(&self) -> usize {
        3 * self.a + self.b
    }
}

#[derive(Debug)]
struct ButtonMovement {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PrizeRecipe {
    a_movement: ButtonMovement,
    b_movement: ButtonMovement,
    prize_position: Position,
}

impl PrizeRecipe {
    fn get_optimal_attempt(&self) -> Option<Attempt> {
        let px = vec![self.a_movement.x, self.b_movement.x, self.prize_position.x];
        let py = vec![self.a_movement.y, self.b_movement.y, self.prize_position.y];

        let (a, b) = solve_linear_system(px, py)?;

        Some(Attempt{ a, b })
    }
}

fn parse_button_line(line: &str) -> Option<ButtonMovement> {
    let rgx = regex::Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").ok()?;
    let (_, [x, y]) = rgx.captures(line)?.extract();

    let x = x.parse::<usize>().ok()?;
    let y = y.parse::<usize>().ok()?;

    Some(ButtonMovement { x, y })
}

fn parse_prize_position(line: &str) -> Option<Position> {
    let rgx = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").ok()?;
    let (_, [x, y]) = rgx.captures(line)?.extract();

    let x = x.parse::<usize>().ok()?;
    let y = y.parse::<usize>().ok()?;

    Some(Position { x, y })
}

fn solve_linear_system(p1: Vec<usize>, p2: Vec<usize>) -> Option<(usize, usize)> {
    let p1_1 = p2[0] * p1[1];
    let p1_2 = p2[0] * p1[2];

    let p2_1 = p1[0] * p2[1];
    let p2_2 = p1[0] * p2[2];

    if (p1_2 > p2_2 && p1_1 <= p2_1) || (p1_2 <= p2_2 && p1_1 >= p2_1) {
        return None;
    }

    let (px_1, px_2) = match p1_2 > p2_2 {
        true => (p1_1 - p2_1, p1_2 - p2_2),
        false => (p2_1 - p1_1, p2_2 - p1_2),
    };

    if px_2 % px_1 != 0 {
        return None;
    }

    let b = px_2 / px_1;
    let p1_hat_1 = b * p1[1];

    if p1_hat_1 > p1[2] {
        return None;
    }

    let p1_hat_2 = p1[2] - p1_hat_1;

    if p1_hat_2 % p1[0] != 0 {
        return None;
    }

    let a = p1_hat_2 / p1[0];

    Some((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![94, 22, 8400], vec![34, 67, 5400], Some((80, 40)))]
    #[test_case(vec![26, 67, 12748], vec![66, 21, 12176], None)]
    #[test_case(vec![17, 84, 7870], vec![86, 37, 6450], Some((38, 86)))]
    #[test_case(vec![69, 27, 18641], vec![23, 71, 10279], None)]
    fn test_solve_linear_system(xp: Vec<usize>, yp: Vec<usize>, expected_result: Option<(usize, usize)>) {
        let result = solve_linear_system(xp, yp);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 480);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 29517);
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
