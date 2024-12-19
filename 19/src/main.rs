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
    let mut count = 0;
    for towel in available {
        if let Some(prefix) = desired.strip_suffix(towel) {
            count += count_solutions(memo, prefix, available);
        }
    }
    memo.insert(desired, count);
    return count
}

fn part_1(input: &str) -> usize {
    let (available, desired) = parse(input);
    let mut memo = HashMap::new();
    desired.iter().filter(|desired| count_solutions(&mut memo, desired, &available) != 0).count()
}

fn part_2(input: &str) -> usize {
    let (available, desired) = parse(input);
    let mut memo = HashMap::new();
    desired.iter().map(|desired| count_solutions(&mut memo, desired, &available)).sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
