use std::collections::HashMap;

fn count_stones(memo: &mut HashMap<(u64, usize), usize>, number: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(count) = memo.get(&(number, blinks)) {
        return *count;
    }

    let count = if number == 0 {
        count_stones(memo, 1, blinks - 1)
    } else {
        let number_string = number.to_string();
        if number_string.len() % 2 == 0 {
            let a: u64 = number_string[..number_string.len()/2].parse().unwrap();
            let b: u64 = number_string[number_string.len()/2..].parse().unwrap();
            let a_count = count_stones(memo, a, blinks - 1);
            let b_count = count_stones(memo, b, blinks - 1);
            a_count + b_count
        } else {
            count_stones(memo, 2024*number, blinks - 1)
        }
    };
    memo.insert((number, blinks), count);
    count
}

fn part_1(input: &[u64]) -> usize {
    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();
    let mut sum = 0;
    for number in input {
        sum += count_stones(&mut memo, *number, 25);
    }
    dbg!(memo.len());
    sum
}

fn part_2(input: &[u64]) -> usize {
    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();
    let mut sum = 0;
    for number in input {
        sum += count_stones(&mut memo, *number, 75);
    }
    dbg!(memo.len());
    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.trim().split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<u64>>();

    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
