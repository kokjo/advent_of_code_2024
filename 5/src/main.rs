use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules : Vec<(i64, i64)> = rules
        .trim()
        .split('\n')
        .map(|l| l.trim().split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let updates: Vec<Vec<i64>> = updates
        .trim()
        .split('\n')
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn part_1_check_update(after_sets: &mut HashMap<i64, HashSet<i64>>, before_sets: &mut HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
    for (i, page) in update.iter().enumerate() {
        let before = &update[..i];
        let after = &update[i+1..];
        let after_set = after_sets.entry(*page).or_default();
        let before_set = before_sets.entry(*page).or_default();
        if before.iter().any(|before_page| after_set.contains(before_page)) {
            return false
        }
        if after.iter().any(|after_page| before_set.contains(after_page)) {
            return false
        }
    }

    true
}

fn part_1(input: &str) -> i64 {
    let (rules, updates) = parse(input);

    let mut middle_sum = 0;

    let mut after_sets: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut before_sets: HashMap<i64, HashSet<i64>> = HashMap::new();

    for &(before, after) in &rules {
        after_sets.entry(before).or_default().insert(after);
        before_sets.entry(after).or_default().insert(before);
    }

    for update in &updates {
        if part_1_check_update(&mut after_sets, &mut before_sets, &update) {
            middle_sum += update[update.len() / 2];
        }

    }

    middle_sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(input.as_str()));
    println!("Hello, world!");
}
