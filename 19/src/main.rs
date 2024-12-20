use std::collections::HashMap;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available, desired) = input.split_once("\n\n").unwrap();
    let available = available.split(',').map(|towel| towel.trim()).collect();
    let desired = desired.trim().split('\n').map(|towel| towel.trim()).collect();

    (available, desired)
}

fn count_solutions<'a>(memo: &mut HashMap<&'a str, usize>, desired: &'a str, available: &[&'a str]) -> usize {
    if desired.len() == 0 {
        return 1;
    }
    if let Some(&result) = memo.get(desired) {
        return result;
    }
    let solutions = available.iter()
        .filter_map(|suffix| desired.strip_suffix(suffix))
        .map(|prefix| count_solutions(memo, prefix, available))
        .sum();
    memo.insert(desired, solutions);
    return solutions
}

fn part_1<'a>(memo: &mut HashMap<&'a str, usize>, available: &[&'a str], desired: &[&'a str]) -> usize {
    desired.iter().filter(|desired| count_solutions(memo, desired, &available) != 0).count()
}

fn part_2<'a>(memo: &mut HashMap<&'a str, usize>, available: &[&'a str], desired: &[&'a str]) -> usize {
    desired.iter().map(|desired| count_solutions(memo, desired, &available)).sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (available, desired) = parse(&input);
    let mut memo = HashMap::new();
    println!("part_1 = {}", part_1(&mut memo, &available, &desired));
    println!("part_2 = {}", part_2(&mut memo, &available, &desired));
}
