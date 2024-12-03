use crate::util::*;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 2, None);

    lines
        .iter()
        .filter(|l| is_valid(&get_integers_in_string(l)))
        .count() as i32
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 2, None);

    lines
        .iter()
        .filter(|l| remove_one_value_from_array(&get_integers_in_string(l))
            .iter()
            .any(|x| is_valid(x))
        )
        .count() as i32
}

fn is_valid(array: &[i32]) -> bool {
    (is_uniformly_increasing(array) || is_uniformly_decreasing(array)) && adjacent_level_do_not_differ_too_much(array)
}

fn is_uniformly_increasing(array: &[i32]) -> bool {
    array.windows(2).all(|w| w[0] < w[1])
}

fn is_uniformly_decreasing(array: &[i32]) -> bool {
    array.windows(2).all(|w| w[0] > w[1])
}

fn adjacent_level_do_not_differ_too_much(array: &[i32]) -> bool {
    const MAX_DIFF: i32 = 3;

    array.windows(2).all(|w| (w[0] - w[1]).abs() <= MAX_DIFF)
}

fn remove_one_value_from_array(array: &[i32]) -> Vec<Vec<i32>> {
    let array_length = array.len();
    let mut result_arrays = Vec::new();

    for i in 0..array_length {
        let mut output_array = vec![0; array_length - 1];

        let mut k = 0;

        for j in 0..array_length {
            if j == i {
                continue;
            }

            output_array[k] = array[j];
            k = k + 1;
        }

        result_arrays.push(output_array);
    }

    return result_arrays;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![7, 6, 4, 2, 1], false)]
    #[test_case(vec![1, 2, 7, 8, 9], true)]
    #[test_case(vec![9, 7, 6, 2, 1], false)]
    #[test_case(vec![1, 3, 2, 4, 5], false)]
    #[test_case(vec![8, 6, 4, 4, 1], false)]
    #[test_case(vec![1, 3, 6, 7, 9], true)]
    fn test_is_uniformly_increasing(input: Vec<i32>, expected: bool) {
        assert_eq!(is_uniformly_increasing(&input), expected);
    }

    #[test_case(vec![7, 6, 4, 2, 1], true)]
    #[test_case(vec![1, 2, 7, 8, 9], false)]
    #[test_case(vec![9, 7, 6, 2, 1], true)]
    #[test_case(vec![1, 3, 2, 4, 5], false)]
    #[test_case(vec![8, 6, 4, 4, 1], false)]
    #[test_case(vec![1, 3, 6, 7, 9], false)]
    fn test_is_uniformly_decreasing(input: Vec<i32>, expected: bool) {
        assert_eq!(is_uniformly_decreasing(&input), expected);
    }

    #[test_case(vec![7, 6, 4, 2, 1], true)]
    #[test_case(vec![1, 2, 7, 8, 9], false)]
    #[test_case(vec![9, 7, 6, 2, 1], false)]
    #[test_case(vec![1, 3, 2, 4, 5], true)]
    #[test_case(vec![8, 6, 4, 4, 1], true)]
    #[test_case(vec![1, 3, 6, 7, 9], true)]
    fn test_adjacent_level_do_not_differ_too_much(input: Vec<i32>, expected: bool) {
        assert_eq!(adjacent_level_do_not_differ_too_much(&input), expected);
    }

    #[test_case(vec![1], vec![Vec::new()])]
    #[test_case(vec![1, 2], vec![vec![2], vec![1]])]
    #[test_case(vec![2, 4, 7], vec![vec![4, 7], vec![2, 7], vec![2, 4]])]
    fn test_remove_one_value_from_array(input: Vec<i32>, expected: Vec<Vec<i32>>) {
        assert_eq!(remove_one_value_from_array(&input), expected);
    }

    #[test]
    fn test_run_one() {
        assert_eq!(run_first(false), 2);
    }

    #[test]
    fn real_run_one() {
        assert_eq!(run_first(true), 332);
    }

    #[test]
    fn test_run_two() {
        assert_eq!(run_second(false), 4);
    }
}
