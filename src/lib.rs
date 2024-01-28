pub mod itemsets;
pub mod rules;
pub mod types;
mod wrapper;

use std::collections::{HashMap, HashSet};
use types::{FrequentItemsets, Inventory, RawTransaction, RawTransactionId};

/// Apriori algorithm for association rules.
///
/// Args:
///     transactions (List[Set[str]]): A list of list of items.
///     min_support (float): The minimum support.
///     min_confidence (float): The minimum confidence.
///     max_length (int): Maximum no. of items in an association rule.
///
/// Returns:
///     A tuple of (i) a list of association rules and (ii) frequent itemsets by size.
pub fn apriori(
    raw_transactions: Vec<RawTransaction>,
    min_support: f32,
    min_confidence: f32,
    max_length: usize,
) -> (Vec<Rule>, FrequentItemsets) {
    let n = raw_transactions.len();
    let (itemset_counts, inventory) =
        itemsets::count::generate_frequent_itemsets(raw_transactions, min_support, max_length);

    let rules = rules::search::generate_rules(&min_confidence, &itemset_counts, n);

    (wrapper::convert_rules(rules, inventory), itemset_counts)
}

/// Generate frequent itemsets from a list of transactions.
///
/// Args:
///     transactions (List[Set[str]]): A list of list of items.
///     min_support (float): The minimum support.
///     max_length (int): Maximum no. of items in an association rule.
///
/// Returns:
///     A tuple of (i) frequent itemsets by size and (ii) a dictionary mapping of item ID to item name.
pub fn generate_frequent_itemsets(
    raw_transactions: Vec<RawTransaction>,
    min_support: f32,
    max_length: usize,
) -> (FrequentItemsets, Inventory) {
    let (itemset_counts, inventory) =
        itemsets::count::generate_frequent_itemsets(raw_transactions, min_support, max_length);

    (itemset_counts, inventory)
}

/// Generate frequent itemsets from a list of transactions.
///
/// Args:
///     transactions (List[Set[int]]): A list of list of items.
///     min_support (float): The minimum support.
///     max_length (int): Maximum no. of items in an association rule.
///
/// Returns:
///     Frequent itemsets by size.
pub fn generate_frequent_itemsets_id(
    raw_transactions: Vec<RawTransactionId>,
    min_support: f32,
    max_length: usize,
) -> FrequentItemsets {
    itemsets::count::generate_frequent_itemsets_id(raw_transactions, min_support, max_length)
}

pub struct Rule {
    pub antecedent: HashSet<String>,
    pub consequent: HashSet<String>,
    pub confidence: f32,
    pub lift: f32,
}
