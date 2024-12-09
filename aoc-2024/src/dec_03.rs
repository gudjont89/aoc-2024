use crate::util::*;

pub fn run_first(is_real: bool) -> i32 {
    let order = match is_real {
        true => None,
        false => Some(1),
    };
    let all_text = read_single_string_from_file(is_real, 3, order);
    
    find_multiplications(&all_text)
}

pub fn run_second(is_real: bool) -> i32 {
    let order = match is_real {
        true => None,
        false => Some(2),
    };
    let all_text = read_single_string_from_file(is_real, 3, order);

    find_multiplication_strings(&all_text)
}

fn find_multiplications(input_str: &str) -> i32 {
    let rgx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    rgx
        .find_iter(input_str)
        .map(|m| m.as_str())
        .map(|s| parse_single_multiplication_string(s).unwrap())
        .map(|v| v.0 * v.1)
        .sum()
}

fn parse_single_multiplication_string(mul_str: &str) -> Option<(i32, i32)> {
    let rgx = regex::Regex::new(r"\d+").unwrap();
    let mut match_iterator = rgx.find_iter(mul_str);

    let first_value = match_iterator.next().map(|m| m.as_str().parse::<i32>().unwrap())?;
    let second_value = match_iterator.next().map(|m| m.as_str().parse::<i32>().unwrap())?;

    Some((first_value, second_value))
}

fn find_multiplication_strings(input_str: &str) -> i32 {
    let rgx = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    rgx
        .find_iter(input_str)
        .map(|m| m.as_str())
        .map(|s| try_parse_match_str(s).unwrap())
        .fold(ProgramState::new(), |state, v| {
            match v {
                MultiplicationMatch::Activate => state.activate(),
                MultiplicationMatch::Deactivate => state.deactivate(),
                MultiplicationMatch::Multiply(x, y) => state.multiply(x, y),
            }
        })
        .aggregate_value
}

struct ProgramState {
    active: bool,
    aggregate_value: i32,
}

impl ProgramState {
    fn new() -> Self {
        Self { active: true, aggregate_value: 0 }
    }

    fn activate(self) -> Self {
        Self { active: true, aggregate_value: self.aggregate_value }
    }

    fn deactivate(self) -> Self {
        Self { active: false, aggregate_value: self.aggregate_value }
    }

    fn multiply(self, x: i32, y: i32) -> Self {
        let aggregate_value = match &self.active {
            true => self.aggregate_value + x * y,
            false => self.aggregate_value,
        };

        Self { active: self.active, aggregate_value, }
    }
}

#[derive(PartialEq, Debug)]
enum MultiplicationMatch {
    Activate,
    Deactivate,
    Multiply(i32, i32),
}

fn try_parse_match_str(match_str: &str) -> Option<MultiplicationMatch> { // todo: think about using Match directly in input
    // todo: try to improve
    let activate_match = parse_activate_str(match_str);

    if activate_match.is_some() {
        return activate_match;
    }

    let deactivate_match = parse_deactivate_str(match_str);

    if deactivate_match.is_some() {
        return deactivate_match;
    }
    
    parse_multiply_str(match_str)
}

fn parse_activate_str(match_str: &str) -> Option<MultiplicationMatch> {
    let rgx = regex::Regex::new(r"do\(\)").unwrap();
    
    rgx.find(match_str).map(|_x| MultiplicationMatch::Activate)
}

fn parse_deactivate_str(match_str: &str) -> Option<MultiplicationMatch> {
    let rgx = regex::Regex::new(r"don't\(\)").unwrap();
    
    rgx.find(match_str).map(|_x| MultiplicationMatch::Deactivate)
}

fn parse_multiply_str(match_str: &str) -> Option<MultiplicationMatch> {
    parse_single_multiplication_string(match_str).map(|(x, y)| MultiplicationMatch::Multiply(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 161);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 191183308);
    }

    #[test_case("mul(2,4)", (2, 4))]
    #[test_case("mul(5,5)", (5, 5))]
    #[test_case("mul(11,8)", (11, 8))]
    #[test_case("mul(8,5)", (8, 5))]
    fn test_parse_single_multiplication_string(mul_str: &str, expected_result: (i32, i32)) {
        let result = parse_single_multiplication_string(mul_str).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 48);
    }

    #[test_case("do()", Some(MultiplicationMatch::Activate))]
    #[test_case("don't()", Some(MultiplicationMatch::Deactivate))]
    #[test_case("mul(2,4)", Some(MultiplicationMatch::Multiply(2, 4)))]
    #[test_case("fedsu#$", None)]
    fn test_try_parse_match_str(match_str: &str, expected_result: Option<MultiplicationMatch>) {
        assert_eq!(try_parse_match_str(match_str), expected_result);
    }
}
