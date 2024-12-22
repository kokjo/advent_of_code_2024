use std::collections::HashSet;

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
    let sequences: Vec<Vec<u64>> = numbers.map(|mut secret| {
        let mut sequence = vec![];
        for _ in 0..2000 {
            sequence.push(secret % 10);
            secret = evolve(secret);
        }
        sequence
    }).collect();

    let possible_changes = sequences.iter().flat_map(|sequence| sequence.windows(5).map(diff)).collect::<HashSet<Vec<i64>>>();
    dbg!(possible_changes.len());

    let mut most_bananas = 0;
    for (i, change) in possible_changes.iter().enumerate() {
        let mut bananas = 0;
        for sequence in sequences.iter() {
            for window in sequence.windows(5) {
                if &diff(window) == change {
                    bananas += window.last().unwrap();
                    break;
                }
            }
        }
        if bananas > most_bananas  {
            most_bananas = bananas;
            dbg!(i, most_bananas);
        }
    }

    most_bananas
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
