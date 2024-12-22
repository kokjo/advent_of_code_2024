use std::collections::{HashMap, HashSet};

fn evolve(mut secret: u64) -> u64 {
    secret ^= (secret << 6) & 0xffffff;
    secret ^= (secret >> 5) & 0xffffff;
    secret ^= (secret << 11) & 0xffffff;
    secret
}

fn part_1(input: &str) -> u64 {
    let numbers = input.trim().lines().map(|num| num.parse::<u64>().unwrap());

    let mut sum = 0;
    for mut secret in numbers {
        secret &= 0xffffff;
        for _ in 0..2000 {
            secret = evolve(secret);
        }
        sum += secret;
    }

    sum
}

fn diff(xs: &[u64]) -> Vec<i64> {
    xs.windows(2).map(|w| w[1] as i64 - w[0] as i64).collect()
}

fn part_2(input: &str) -> u64 {
    let numbers = input.trim().lines().map(|num| num.parse::<u64>().unwrap());

    let mut change_score = HashMap::<Vec<i64>, u64>::new();
    for mut secret in numbers {
        let mut sequence = Vec::new();
        for _ in 0..2000 {
            sequence.push(secret % 10);
            secret = evolve(secret);
        }

        let mut seen = HashSet::<Vec<i64>>::new();
        for window in sequence.windows(5) {
            let diff = diff(window);
            if seen.insert(diff.clone()) {
                *change_score.entry(diff).or_default() += window.last().unwrap();
            }
        }
    }

    change_score.into_iter().map(|(_, score)| score).max().unwrap()

}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
