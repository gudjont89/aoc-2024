use crate::util::read_from_file;

pub fn run_first(is_real: bool) -> usize {
    let lines = read_from_file(is_real, 7, None);
    lines
        .iter()
        .filter_map(|x| parse_calibration_result_line(x))
        .filter(|cl| cl.result_can_be_calculated_from_inputs())
        .map(|cl| cl.result)
        .sum::<usize>()
}

// pub fn run_second(is_real: bool) -> i32 {
//     let lines = read_from_file(is_real, 7, None);
    
//     0
// }

#[derive(Debug)]
struct CalibrationLine {
    result: usize,
    inputs: Vec<usize>,
}

impl CalibrationLine {
    fn result_can_be_calculated_from_inputs(&self) -> bool {
        ResultTree::new(&self.inputs)
            .get_all_results()
            .contains(&self.result)
    }
}

struct ResultTree {
    initial_node: ResultNode,
}

impl ResultTree {
    fn new(inputs: &[usize]) -> Self {
        Self {
            initial_node: ResultNode::populate(0, inputs),
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
    fn populate(temporary_result: usize, rest_of_inputs: &[usize]) -> Self {
        if rest_of_inputs.len() == 0 {
            return Self {
                node_type: ResultNodeType::Result(temporary_result),
            };
        }

        let addition_result = temporary_result + rest_of_inputs[0];
        let multiplication_result = temporary_result * rest_of_inputs[0];

        Self {
            node_type: ResultNodeType::PartiallyProcessed(
                Box::new(ResultNode::populate(addition_result, &rest_of_inputs[1..])),
                Box::new(ResultNode::populate(multiplication_result, &rest_of_inputs[1..])),
            ),
        }
    }
    
    fn get_all_results(&self) -> Vec<usize> {
        match &self.node_type {
            ResultNodeType::Result(result) => vec![*result],
            ResultNodeType::PartiallyProcessed(addition_node, multiplication_node) => {
                return addition_node
                    .get_all_results()
                    .iter()
                    .chain(multiplication_node
                        .get_all_results()
                        .iter()
                    )
                    .map(|x| *x).collect::<Vec<usize>>();
            },
        }
    }
}

enum ResultNodeType {
    Result(usize),
    PartiallyProcessed(Box<ResultNode>, Box<ResultNode>),
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

    // #[test]
    // fn test_run_second() {
    //     assert_eq!(run_second(false), 6);
    // }
}
