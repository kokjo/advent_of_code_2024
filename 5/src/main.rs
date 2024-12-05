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

fn check_update(after_sets: &HashMap<i64, HashSet<i64>>, update: &[i64]) -> bool {
    ! update.iter()
        .enumerate()
        .filter_map(|(i, page)| after_sets.get(page).map(|set| (i, set)))
        .any(|(i, set)| update[..i].iter().any(|page| set.contains(page)))
}

fn preprocess_rules(rules: &[(i64, i64)]) -> HashMap<i64, HashSet<i64>> {
    let mut after_sets: HashMap<i64, HashSet<i64>> = HashMap::new();

    for &(before, after) in rules {
        after_sets.entry(before).or_default().insert(after);
    }

    after_sets
}

fn part_1(input: &str) -> i64 {
    let (rules, updates) = parse(input);
    let after_sets = preprocess_rules(&rules);

    let mut middle_sum = 0;

    for update in &updates {
        if check_update(&after_sets, &update) {
            middle_sum += update[update.len() / 2];
        }

    }

    middle_sum
}

fn part_2(input: &str) -> i64 {
    let (rules, updates) = parse(input);
    let after_sets = preprocess_rules(&rules);

    let mut middle_sum = 0;

    for update in &updates {
        if check_update(&after_sets, &update) {
            continue;
        }

        let mut pages: HashSet<i64> = HashSet::from_iter(update.iter().cloned());
        let mut new_update = vec![];

        while ! pages.is_empty() {
            let hash_set = pages.clone();
            for page in hash_set.iter() {
                if pages.iter().filter_map(|other_page| after_sets.get(other_page)).any(|after_set| after_set.contains(page)) {
                    continue;
                }
                pages.remove(page);
                new_update.push(*page);
                break
            }
        }

        assert!(check_update(&after_sets, &new_update));

        middle_sum += new_update[update.len() / 2];
    }

    middle_sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(input.as_str()));
    println!("part_2 = {}", part_2(input.as_str()));
}
