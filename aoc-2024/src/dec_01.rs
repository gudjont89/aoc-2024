use std::collections::HashMap;

use itertools::Itertools;

use crate::util::*;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 1, None);
    let (mut first_array, mut second_array) = get_input_arrays(lines);

    first_array.sort();
    second_array.sort();

    let distance = get_distance(first_array, second_array);

    distance
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 1, None);
    let (first_array, second_array) = get_input_arrays(lines);

    let lookup_table = create_lookup_table(second_array);
    
    first_array
        .iter()
        .map(|x| calculate_similarity_score(*x, &lookup_table))
        .sum()
}

fn calculate_similarity_score(value: i32, lookup_table: &HashMap<i32, i32>) -> i32 {
    match lookup_table.get(&value) {
        None => 0,
        Some(frequency) => value * frequency,
    }
}

fn create_lookup_table(second_array: Vec<i32>) -> HashMap<i32, i32> {
    second_array
        .iter()
        .unique()
        .map(|fav| (*fav, second_array.iter().filter(|&sav| fav == sav).count() as i32))
        .collect()
}

fn get_input_arrays(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    lines
        .iter()
        .map(|l| get_integers_in_string(l))
        .map(|x| (x[0], x[1]))
        .unzip()
}

fn get_distance(first_array: Vec<i32>, second_array: Vec<i32>) -> i32 {
    first_array
        .iter()
        .zip(second_array.iter())
        .map(|x| (x.0 - x.1).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_one() {
        assert_eq!(run_first(false), 11);
    }

    #[test]
    fn real_run_one() {
        assert_eq!(run_first(true), 2756096);
    }

    #[test]
    fn test_create_lookup_table() {
        let second_array = vec![4, 3, 5, 3, 9, 3];
        
        let lookup_table = create_lookup_table(second_array);

        assert_eq!(lookup_table.get(&3), Some(&3));
        assert_eq!(lookup_table.get(&4), Some(&1));
        assert_eq!(lookup_table.get(&2), None);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let mut lookup_table = HashMap::new();
        lookup_table.insert(3, 3);
        lookup_table.insert(4, 1);
        lookup_table.insert(5, 1);
        lookup_table.insert(9, 1);

        assert_eq!(calculate_similarity_score(1, &lookup_table), 0);
        assert_eq!(calculate_similarity_score(2, &lookup_table), 0);
        assert_eq!(calculate_similarity_score(3, &lookup_table), 9);
        assert_eq!(calculate_similarity_score(4, &lookup_table), 4);
    }

    #[test]
    fn test_run_two() {
        assert_eq!(run_second(false), 31);
    }
}
