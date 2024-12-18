use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

fn part_1(input: &str) -> usize {
    let locations = input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(isize, isize)>>();

    let map: HashSet<(isize, isize)> = HashSet::from_iter(locations[..1024].iter().copied());

    let mut score_map = HashMap::<(isize, isize), usize>::new();
    let mut queue = BinaryHeap::<(Reverse<usize>, (isize, isize))>::from_iter([(Reverse(0), (0, 0))]);

    while let Some((score, pos)) = queue.pop() {
        if pos.0 < 0 || pos.0 > 70 || pos.1 < 0 || pos.1 > 70 {
            continue
        }
        if map.contains(&pos) {
            continue
        }
        let old_score = score_map.entry(pos).or_insert(usize::MAX);
        if *old_score > score.0 {
            *old_score = score.0;
            queue.extend([
                (Reverse(score.0 + 1), (pos.0 + 1, pos.1)),
                (Reverse(score.0 + 1), (pos.0 - 1, pos.1)),
                (Reverse(score.0 + 1), (pos.0, pos.1 + 1)),
                (Reverse(score.0 + 1), (pos.0, pos.1 - 1)),
            ])
        }
    }

    *score_map.get(&(70, 70)).unwrap()
}

fn part_2(input: &str) -> String {
    let locations = input
        .trim()
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(isize, isize)>>();

    for i in 0..locations.len() {
        let map: HashSet<(isize, isize)> = HashSet::from_iter(locations[..i].iter().copied());

        let mut score_map = HashMap::<(isize, isize), usize>::new();
        let mut queue = BinaryHeap::<(Reverse<usize>, (isize, isize))>::from_iter([(Reverse(0), (0, 0))]);

        while let Some((score, pos)) = queue.pop() {
            if pos.0 < 0 || pos.0 > 70 || pos.1 < 0 || pos.1 > 70 {
                continue
            }
            if map.contains(&pos) {
                continue
            }
            let old_score = score_map.entry(pos).or_insert(usize::MAX);
            if *old_score > score.0 {
                *old_score = score.0;
                queue.extend([
                    (Reverse(score.0 + 1), (pos.0 + 1, pos.1)),
                    (Reverse(score.0 + 1), (pos.0 - 1, pos.1)),
                    (Reverse(score.0 + 1), (pos.0, pos.1 + 1)),
                    (Reverse(score.0 + 1), (pos.0, pos.1 - 1)),
                ])
            }
        }

        if score_map.get(&(70, 70)) == None {
            return format!("{},{}", locations[i-1].0, locations[i-1].1)
        }
    }

    panic!("Should never happen!")

}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
