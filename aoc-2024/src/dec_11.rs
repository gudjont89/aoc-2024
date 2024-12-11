use crate::util::read_single_string_from_file;

const STONE_MULTIPLIER: usize = 2024;

pub fn run_first(is_real: bool) -> usize {
    const NUMBER_OF_BLINKS: usize = 25;

    let line = read_single_string_from_file(is_real, 11, None);

    convert_stones(
        get_numbers_in_string(&line), 
        NUMBER_OF_BLINKS)
    .len()
}

pub fn run_second(is_real: bool) -> usize {
    const NUMBER_OF_BLINKS: usize = 75;
    // todo: this is too large a number, we need to rethink this. maybe a tree would be a good idea

    let line = read_single_string_from_file(is_real, 11, None);

    convert_stones(
        get_numbers_in_string(&line), 
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

    // #[test]
    // fn real_run_second() {
    //     assert_eq!(run_second(true), xx);
    // }
}
