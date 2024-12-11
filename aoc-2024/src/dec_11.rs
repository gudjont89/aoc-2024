use std::{collections::HashMap, time::Instant, vec};

use itertools::Itertools;

use crate::util::read_single_string_from_file;

const STONE_MULTIPLIER: usize = 2024;

pub fn run_first(is_real: bool) -> usize {
    const NUMBER_OF_BLINKS: usize = 25;

    let line = read_single_string_from_file(is_real, 11, None);
    let initial_stones = get_numbers_in_string(&line);

    convert_stones(
        initial_stones,
        NUMBER_OF_BLINKS)
    .len()
}

fn convert_stones(mut stones: Vec<usize>, number_of_blinks: usize) -> Vec<usize> {
    for _ in 0..number_of_blinks {
        stones = stones
            .iter()
            .flat_map(|s| transform_stone(*s))
            .collect::<Vec<usize>>();
    }

    stones
}

pub fn run_second(is_real: bool) -> usize {
    const NUMBER_OF_BLINKS: usize = 75;
    
    let line = read_single_string_from_file(is_real, 11, None);
    let initial_stones = get_numbers_in_string(&line);
    
    let lookup_table_keys = get_lookup_table_keys(&initial_stones);

    let mut lookup_table: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for i in 1..NUMBER_OF_BLINKS {
        let value_to_score_table = populate_lookup_table(
            lookup_table_keys.clone(), 
            i,
            &lookup_table
        );
        lookup_table.insert(i, value_to_score_table);
    }

    count_stones_after_blinking_collection(initial_stones, NUMBER_OF_BLINKS, &lookup_table)
}

fn get_lookup_table_keys(initial_stones: &[usize]) -> Vec<usize> {
    let initial_stones = initial_stones
        .iter()
        .map(|x| *x)
        .collect::<Vec<usize>>();

    let converted_stones = convert_stones(initial_stones, 25);
    let unique_valued_stones = converted_stones
        .iter()
        .unique()
        .map(|&s| s)
        .collect::<Vec<usize>>();
    
    unique_valued_stones
        .iter()
        .map(|&sv| (sv, converted_stones.iter().filter(|&&csv| csv == sv).count()))
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .filter(|&(_, f)| f > 100)
        .map(|(v, _)| v)
        .collect::<Vec<usize>>()
}

fn populate_lookup_table(
    stone_values: Vec<usize>, 
    depth: usize,
    lookup_table: &HashMap<usize, HashMap<usize, usize>>,
) -> HashMap<usize, usize> {
    stone_values
        .iter()
        .map(|&sv| (sv, count_stones_after_blinking(sv, depth, lookup_table)))
        .collect()
}

fn count_stones_after_blinking_collection(
    stones: Vec<usize>, 
    number_of_blinks: usize,
    lookup_table: &HashMap<usize, HashMap<usize, usize>>
) -> usize {
    stones
        .iter()
        .map(|s| count_stones_after_blinking(*s, number_of_blinks, lookup_table))
        .sum()
}

fn count_stones_after_blinking(
    stone: usize, 
    number_of_blinks: usize,
    lookup_table: &HashMap<usize, HashMap<usize, usize>>
) -> usize {
    StoneNode::initial(stone).count_children_recursively(number_of_blinks, lookup_table)
}

fn get_numbers_in_string(number_str: &str) -> Vec<usize> {
    let rgx = regex::Regex::new(r"(\d+)").unwrap();

    rgx
        .find_iter(number_str)
        .map(|m| m.as_str())
        .filter_map(|ns| ns.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn transform_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }

    if let Some(split_stones) = split_stone(stone) {
        return split_stones;
    }

    vec![STONE_MULTIPLIER * stone]
}

fn split_stone(stone: usize) -> Option<Vec<usize>> {
    let number_of_digits = (stone as f64).log10().floor() as usize + 1;

    if number_of_digits % 2 == 1 {
        return None;
    }

    let number_of_digits_per_stone = number_of_digits / 2;

    let msn = stone / 10_usize.pow(number_of_digits_per_stone as u32);
    let lsn = stone - msn * 10_usize.pow(number_of_digits_per_stone as u32);

    Some(vec![msn, lsn])
}

struct StoneNode {
    depth: usize,
    value: usize,
}

impl StoneNode {
    fn initial(value: usize) -> Self {
        Self { depth: 0, value }
    }

    fn new(parent_depth: usize, value: usize) -> Self {
        Self { depth: parent_depth + 1, value }
    }

    fn count_children_recursively(
        &self, 
        maximum_depth: usize,
        lookup_table: &HashMap<usize, HashMap<usize, usize>>
    ) -> usize {
        if self.depth == maximum_depth {
            return 1;
        };

        let steps_until_maximum_depth = maximum_depth - self.depth;

        if let Some(number_of_children) = lookup_table
            .get(&steps_until_maximum_depth)
            .map(|lut| lut.get(&self.value))
            .flatten() 
        {
            return *number_of_children;
        };

        transform_stone(self.value)
            .iter()
            .map(|ns_value| Self::new(self.depth, *ns_value))
            .map(|sn| sn.count_children_recursively(maximum_depth, lookup_table))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 55312);
    }

    #[test_case(12, Some(vec![1, 2]))]
    #[test_case(253000, Some(vec![253, 0]))]
    #[test_case(28676032, Some(vec![2867, 6032]))]
    #[test_case(9, None)]
    #[test_case(123, None)]
    fn test_split_stones(stone: usize, expected_result: Option<Vec<usize>>) {
        let result = split_stone(stone);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 233875);
    }

    #[test]
    fn real_run_second() {
        assert_eq!(run_second(true), 277444936413293);
    }
}
