use crate::util::*;

pub fn run_first(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 5, None);

    let order_rules = lines
        .iter()
        .filter_map(|l| parse_order_rule(l))
        .collect::<Vec<OrderRule>>();
    
    lines
        .iter()
        .filter_map(|l| parse_printing_update(&l))
        .filter(|pu| pu.fulfills_rules(&order_rules))
        .map(|pu| pu.get_middle_number())
        .sum()
}

pub fn run_second(is_real: bool) -> i32 {
    let lines = read_from_file(is_real, 5, None);

    let order_rules = lines
        .iter()
        .filter_map(|l| parse_order_rule(l))
        .collect::<Vec<OrderRule>>();
    let mut printing_updates = lines
        .iter()
        .filter_map(|l| parse_printing_update(&l))
        .filter(|pu| !pu.fulfills_rules(&order_rules))
        .collect::<Vec<PrintingUpdate>>();

    printing_updates
        .iter_mut()
        .for_each(|pu| pu.order(&order_rules));

    printing_updates
        .iter()
        .map(|pu| pu.get_middle_number())
        .sum()
}

fn parse_order_rule(rule_str: &str) -> Option<OrderRule> {
    let rgx = regex::Regex::new(r"(\d+)\|(\d+)").ok()?;
    let _m = rgx.find(rule_str)?;

    let rgx = regex::Regex::new(r"(\d+)").ok()?;
    let mut matches = rgx.find_iter(rule_str);

    let before = matches.next()?.as_str().parse::<i32>().ok()?;
    let after = matches.next()?.as_str().parse::<i32>().ok()?;
    
    Some(OrderRule { before, after })
}

fn parse_printing_update(rule_str: &str) -> Option<PrintingUpdate> {
    let rgx = regex::Regex::new(r"(\d+,+\d+)").ok()?;
    let _m = rgx.find(rule_str)?;

    let rgx = regex::Regex::new(r"\d+").ok()?;

    let pages = rgx
        .find_iter(rule_str)
        .filter_map(|m| m.as_str().parse::<i32>().ok())
        .collect::<Vec<i32>>();

    Some(PrintingUpdate { pages })
}

#[derive(Debug, PartialEq)]
struct OrderRule {
    before: i32,
    after: i32,
}

impl OrderRule {
    fn apply_to(&self, pages: &mut [i32]) {
        let Some(before_position) = pages.iter().position(|p| *p == self.before) else { return; };
        let Some(after_position) = pages.iter().position(|p| *p == self.after) else { return; };

        if before_position > after_position {
            pages.swap(before_position, after_position);
        }
    }
}

#[derive(Debug, PartialEq)]
struct PrintingUpdate {
    pages: Vec<i32>,
}

impl PrintingUpdate {
    fn fulfills_rules(&self, order_rules: &[OrderRule]) -> bool {
        order_rules.iter().all(|r| self.fulfills_rule(r))
    }

    fn fulfills_rule(&self, order_rule: &OrderRule) -> bool {
        let Some(before_position) = self.pages.iter().position(|p| *p == order_rule.before) else { return true };
        let Some(after_position) = self.pages.iter().position(|p| *p == order_rule.after) else { return true };

        before_position < after_position
    }

    fn get_middle_number(&self) -> i32 {
        let middle_index = (self.pages.len() - 1) / 2;

        self.pages[middle_index]
    }

    fn order(&mut self, order_rules: &[OrderRule]) {
        // todo: I'm not 100% happy with the while loop, this could be more elegant
        while !self.fulfills_rules(order_rules) {
            order_rules
                .iter()
                .for_each(|x| x.apply_to(&mut self.pages));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_run_first() {
        assert_eq!(run_first(false), 143);
    }

    #[test_case("47|53", Some( OrderRule { before: 47, after: 53 } ))]
    #[test_case("947|2", Some( OrderRule { before: 947, after: 2 } ))]
    #[test_case("", None)]
    #[test_case("75,47,61,53,29", None)]
    fn test_parse_order_rule(maybe_rule_str: &str, expected_result: Option<OrderRule>) {
        let result = parse_order_rule(maybe_rule_str);

        assert_eq!(result, expected_result)
    }

    #[test_case("75,47,61,353,29", Some(PrintingUpdate { pages: vec![75, 47, 61, 353, 29] }))]
    #[test_case("", None)]
    #[test_case("34|64", None)]
    fn test_parse_printing_update(maybe_printing_update_str: &str, expected_result: Option<PrintingUpdate>) {
        let result = parse_printing_update(maybe_printing_update_str);

        assert_eq!(result, expected_result)
    }

    #[test_case(PrintingUpdate { pages: vec![75, 47, 61, 353, 29] }, OrderRule { before: 47, after: 53 }, true)]
    fn test_fulfills_rule(printing_update: PrintingUpdate, order_rule: OrderRule, expected_result: bool) {
        let result = printing_update.fulfills_rule(&order_rule);

        assert_eq!(result, expected_result);
    }

    #[test_case(PrintingUpdate { pages: vec![1, 2, 53, 4, 5] }, 53)]
    #[test_case(PrintingUpdate { pages: vec![1, 4, 5] }, 4)]
    fn test_get_middle_number(printing_update: PrintingUpdate, expected_result: i32) {
        let result = printing_update.get_middle_number();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn real_run_first() {
        assert_eq!(run_first(true), 4814);
    }

    #[test]
    fn test_run_second() {
        assert_eq!(run_second(false), 123);
    }

    #[test_case(OrderRule { before: 47, after: 53 }, &mut [53, 2, 47], &[47, 2, 53])]
    fn test_apply_order_rule_to(order_rule: OrderRule, pages: &mut [i32], expected_pages: &[i32]) {
        order_rule.apply_to(pages);

        assert_eq!(pages, expected_pages)
    }

    #[test]
    fn test_order_printing_update() {
        let expected_result = vec![1, 4, 5];

        let mut printing_update = PrintingUpdate { pages: vec![4, 5, 1] };
        let order_rules = vec![OrderRule { before: 1, after: 4 }, OrderRule { before: 4, after: 5 }];

        printing_update.order(&order_rules);

        for (i, page) in printing_update.pages.iter().enumerate() {
            println!("{}", i);
            assert_eq!(*page, expected_result[i]);
        }
    }
}
