use std::collections::HashMap;

use crate::util::*;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 4, None);

    let letter_map = LetterMap::new(lines);

    letter_map.count_all_words_in_all_direction() as i32
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 4, None);

    let letter_map = LetterMap::new(lines);

    let tmp = letter_map
        .letter_map
        .keys()
        .filter(|x| letter_map.check_cross_mas_pattern(*x))
        .count();

    tmp as i32
}

const FIRST_LETTER: char = 'X';
const SECOND_LETTER: char = 'M';
const THIRD_LETTER: char = 'A';
const FOURTH_LETTER: char = 'S';

struct LetterMap {
    letter_map: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl LetterMap {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();

        let letter_map: HashMap<(usize, usize), char> = lines
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    acc.insert((x, y), c);
                }
            );

            acc
        });
        
        LetterMap { letter_map, width, height, }
    }

    fn get_char(&self, index: &(usize, usize)) -> Option<char> {
        self.letter_map.get(index).map(|x| *x)
    }

    fn count_all_words_in_all_direction(&self) -> usize {
        self.letter_map.keys().into_iter().map(|index| self.count_words_in_all_directions(index)).sum()
    }

    fn count_words_in_all_directions(&self, first_letter_index: &(usize, usize)) -> usize {
        get_all_directions().iter().filter(|x| self.check_word(first_letter_index, x)).count()
    }

    fn check_word(&self, first_letter_index: &(usize, usize), direction: &Direction) -> bool {
        self.check_letter(first_letter_index, 0, direction)
            && self.check_letter(first_letter_index, 1, direction)
            && self.check_letter(first_letter_index, 2, direction)
            && self.check_letter(first_letter_index, 3, direction)
    }

    fn check_letter(&self, index: &(usize, usize), order: usize, direction: &Direction) -> bool {
        let Some(shifted_index) = self.calculate_shifted_index(index, direction.get_total_shift(order)) else { return false };

        if let Some(letter) = self.letter_map.get(&shifted_index) {
            if order == 0 {
                return FIRST_LETTER == *letter;
            } else if order == 1 {
                return SECOND_LETTER == *letter;
            } else if order == 2 {
                return THIRD_LETTER == *letter;
            } else {
                return FOURTH_LETTER == *letter;
            }
        };

        false
    }

    fn calculate_shifted_index(&self, (x0, y0): &(usize, usize), shift: (i32, i32)) -> Option<(usize, usize)> {
        let (xs, ys) = shift;

        let x1 = *x0 as i32 + xs;
        let y1 = *y0 as i32 + ys;

        if x1 < 0 || x1 >= self.width as i32 || y1 < 0 || y1 >= self.height as i32 {
            return None;
        }

        Some((x1 as usize, y1 as usize))
    }

    fn check_cross_mas_pattern(&self, index: &(usize, usize)) -> bool {
        let Some(c_letter) = self.get_char(index) else { return false };
        
        if c_letter != 'A' {
            return false;
        }
        
        let nw_letter = self.calculate_shifted_index(index, Direction::NW.get_single_shift()).map(|x| self.get_char(&x)).flatten();
        let ne_letter = self.calculate_shifted_index(index, Direction::NE.get_single_shift()).map(|x| self.get_char(&x)).flatten();
        let se_letter = self.calculate_shifted_index(index, Direction::SE.get_single_shift()).map(|x| self.get_char(&x)).flatten();
        let sw_letter = self.calculate_shifted_index(index, Direction::SW.get_single_shift()).map(|x| self.get_char(&x)).flatten();

        let Some(nw_letter) = nw_letter else { return false };
        let Some(ne_letter) = ne_letter else { return false };
        let Some(se_letter) = se_letter else { return false };
        let Some(sw_letter) = sw_letter else { return false };

        let nw_to_se_correct = nw_letter == 'M' && se_letter == 'S' || nw_letter == 'S' && se_letter == 'M';
        let sw_to_ne_correct = sw_letter == 'M' && ne_letter == 'S' || sw_letter == 'S' && ne_letter == 'M';

        nw_to_se_correct && sw_to_ne_correct
    }
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn get_total_shift(&self, shift: usize) -> (i32, i32) {
        let (x, y) = self.get_single_shift();

        (shift as i32 * x, shift as i32 * y)
    }

    fn get_single_shift(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

fn get_all_directions() -> Vec<Direction> {
    vec![Direction::N, Direction::NE, Direction::E, Direction::SE, Direction::S, Direction::SW, Direction::W, Direction::NW]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_one() {
        assert_eq!(run_first(false), 18);
    }

    #[test]
    fn real_run_one() {
        assert_eq!(run_first(true), 2507);
    }

    #[test]
    fn test_run_two() {
        assert_eq!(run_second(false), 9);
    }

    #[test]
    fn real_run_two() {
        assert_eq!(run_second(true), 1969);
    }
}
