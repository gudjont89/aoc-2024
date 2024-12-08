use crate::util::read_from_file;

pub fn run_first(is_real: bool) -> usize {
    solve(is_real, false)
}

pub fn run_second(is_real: bool) -> usize {
    solve(is_real, true)
}

fn solve(is_real: bool, allow_concatenation: bool) -> usize {
    read_from_file(is_real, 7, None)
        .iter()
        .filter_map(|x| parse_calibration_result_line(x))
        .filter(|cl| cl.result_can_be_calculated_from_inputs(allow_concatenation))
        .map(|cl| cl.result)
        .sum::<usize>()
}

#[derive(Debug)]
struct CalibrationLine {
    result: usize,
    inputs: Vec<usize>,
}

impl CalibrationLine {
    fn result_can_be_calculated_from_inputs(&self, allow_concatenation: bool) -> bool {
        ResultTree::new(&self.inputs, allow_concatenation)
            .get_all_results()
            .contains(&self.result)
    }
}

struct ResultTree {
    initial_node: ResultNode,
}

impl ResultTree {
    fn new(inputs: &[usize], allow_concatenation: bool) -> Self {
        Self {
            initial_node: ResultNode::populate(0, inputs, allow_concatenation),
        }
    }

    fn get_all_results(&self) -> Vec<usize> {
        self.initial_node.get_all_results()
    }
}

struct ResultNode {
    node_type: ResultNodeType,
}

impl ResultNode {
    fn populate(
        temporary_result: usize, 
        rest_of_inputs: &[usize],
        allow_concatenation: bool,
    ) -> Self {
        if rest_of_inputs.len() == 0 {
            return Self {
                node_type: ResultNodeType::Result(temporary_result),
            };
        }

        let addition_result = temporary_result + rest_of_inputs[0];
        let addition_node = Box::new(ResultNode::populate(
            addition_result, 
            &rest_of_inputs[1..],
            allow_concatenation,
        ));

        let multiplication_result = temporary_result * rest_of_inputs[0];
        let multiplication_node = Box::new(ResultNode::populate(
            multiplication_result, 
            &rest_of_inputs[1..],
            allow_concatenation,
        ));

        let concatenation_result = concatenate_usize(temporary_result, rest_of_inputs[0]);
        let concatenation_node = match allow_concatenation {
            true => {
                Some(Box::new(ResultNode::populate(
                    concatenation_result,
                    &rest_of_inputs[1..],
                    true,
                )))
            },
            false => None,
        };

        Self {
            node_type: ResultNodeType::PartiallyProcessed(
                addition_node,
                multiplication_node,
                concatenation_node,
            ),
        }
    }
    
    fn get_all_results(&self) -> Vec<usize> {
        match &self.node_type {
            ResultNodeType::Result(result) => vec![*result],
            ResultNodeType::PartiallyProcessed(
                addition_node, 
                multiplication_node,
                maybe_concatenation_node,
            ) => {
                let addition_node_results = addition_node.get_all_results();
                let multiplication_node_results = multiplication_node.get_all_results();
                let concatenation_node_results = match maybe_concatenation_node {
                    Some(concatenation_node) => concatenation_node.get_all_results(),
                    None => Vec::new(),
                };

                return addition_node_results.iter()
                    .chain(multiplication_node_results.iter())
                    .chain(concatenation_node_results.iter())
                    .map(|x| *x)
                    .collect::<Vec<usize>>();
            },
        }
    }
}

enum ResultNodeType {
    Result(usize),
    PartiallyProcessed(
        Box<ResultNode>, 
        Box<ResultNode>,
        Option<Box<ResultNode>>
    ),
}

fn concatenate_usize(first: usize, second: usize) -> usize {
    if second == 0 {
        return 10 * first;
    }

    let second_as_f64 = second as f64;
    let second_length = second_as_f64.log10().floor() as usize + 1;
    let first_multiplier = 10.0_f64.powi(second_length as i32) as usize;

    first * first_multiplier + second
}

fn parse_calibration_result_line(line: &str) -> Option<CalibrationLine> {
    let rgx = regex::Regex::new(r"(\d+): ((\d+) )+(\d+)").ok()?;
    let _m = rgx.find(line)?;

    let rgx = regex::Regex::new(r"(\d+)").ok()?;
    let mut matches = rgx.find_iter(line);

    let result = matches.next()?.as_str().parse::<usize>().ok()?;
    let inputs = matches
        .filter_map(|m| m.as_str().parse::<usize>().ok())
        .collect();

    Some(CalibrationLine { result, inputs, })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 3749);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 42283209483350)
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 11387);
    }

    #[test_case(12, 0, 120)]
    #[test_case(1, 1, 11)]
    #[test_case(12, 5, 125)]
    #[test_case(132, 243, 132243)]
    fn test_concatenate_usize(first: usize, second: usize, expected_result: usize) {
        let result = concatenate_usize(first, second);

        assert_eq!(result, expected_result);
    }
}
