use crate::rules;
use crate::types::Inventory;
use crate::Rule;
use std::cmp::Ordering::Equal;

pub fn convert_rules(rules: Vec<rules::rule::Rule>, inventory: Inventory) -> Vec<Rule> {
    let mut pyrules: Vec<Rule> = rules
        .into_iter()
        .map(|x| Rule {
            antecedent: x
                .get_antecedent()
                .iter()
                .map(|item_id| String::from(inventory[item_id]))
                .collect(),
            consequent: x
                .get_consequent()
                .iter()
                .map(|item_id| String::from(inventory[item_id]))
                .collect(),
            confidence: x.confidence,
            lift: x.lift,
        })
        .collect();
    pyrules.sort_by(|a, b| (-a.confidence).partial_cmp(&-b.confidence).unwrap_or(Equal));
    pyrules
}
