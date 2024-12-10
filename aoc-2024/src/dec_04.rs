use std::collections::HashMap;

use crate::util::*;

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 4, None);

    let letter_map = LetterMap::new(lines);

    letter_map.count_all_words_in_all_direction()
}

pub fn run_second(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 4, None);

    let letter_map = LetterMap::new(lines);

    letter_map
        .letter_map
        .keys()
        .filter(|x| letter_map.check_cross_mas_pattern(*x))
        .count()
}

const FIRST_LETTER: char = 'X';
const SECOND_LETTER: char = 'M';
const THIRD_LETTER: char = 'A';
const FOURTH_LETTER: char = 'S';

// todo: use LocationMap from util.rs
struct LetterMap {
    letter_map: HashMap<Position, char>,
    width: usize,
    height: usize,
}

impl LetterMap {
    fn new(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();

        let letter_map = position_map_from_text_lines(
            &lines, 
            |c| c,
        );
        
        LetterMap { letter_map, width, height, }
    }

    fn get_char(&self, position: &Position) -> Option<char> {
        self.letter_map.get(position).map(|x| *x)
    }

    fn count_all_words_in_all_direction(&self) -> usize {
        self.letter_map.keys().into_iter().map(|position| self.count_words_in_all_directions(position)).sum()
    }

    fn count_words_in_all_directions(&self, first_letter_index: &Position) -> usize {
        ordinal_directions().iter().filter(|x| self.check_word(first_letter_index, x)).count()
    }

    fn check_word(&self, first_letter_index: &Position, direction: &Direction) -> bool {
        self.check_letter(first_letter_index, 0, direction)
            && self.check_letter(first_letter_index, 1, direction)
            && self.check_letter(first_letter_index, 2, direction)
            && self.check_letter(first_letter_index, 3, direction)
    }

    fn check_letter(&self, position: &Position, order: usize, direction: &Direction) -> bool {
        let Some(new_position) = position.new_position(&direction.get_total_movement(order)) else { return false };

        if let Some(letter) = self.letter_map.get(&new_position) {
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

    fn check_cross_mas_pattern(&self, position: &Position) -> bool {
        let Some(c_letter) = self.get_char(position) else { return false };
        
        if c_letter != 'A' {
            return false;
        }
        
        // todo: this is too complicated
        let nw_letter = position.new_position(&Direction::NW.get_movement()).map(|x| self.get_char(&x)).flatten();
        let ne_letter = position.new_position(&Direction::NE.get_movement()).map(|x| self.get_char(&x)).flatten();
        let se_letter = position.new_position(&Direction::SE.get_movement()).map(|x| self.get_char(&x)).flatten();
        let sw_letter = position.new_position(&Direction::SW.get_movement()).map(|x| self.get_char(&x)).flatten();

        let Some(nw_letter) = nw_letter else { return false };
        let Some(ne_letter) = ne_letter else { return false };
        let Some(se_letter) = se_letter else { return false };
        let Some(sw_letter) = sw_letter else { return false };

        let nw_to_se_correct = nw_letter == 'M' && se_letter == 'S' || nw_letter == 'S' && se_letter == 'M';
        let sw_to_ne_correct = sw_letter == 'M' && ne_letter == 'S' || sw_letter == 'S' && ne_letter == 'M';

        nw_to_se_correct && sw_to_ne_correct
    }
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
