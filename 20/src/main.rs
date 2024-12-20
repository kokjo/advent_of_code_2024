use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

type Pos = (isize, isize);

fn parse(input: &str) -> (HashSet<Pos>, Pos, Pos) {
    let mut start = None;
    let mut end = None;
    let mut map = HashSet::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            let pos = (x as isize, y as isize);
            if ch == 'S' {
                start = Some(pos);
            } else if ch == 'E' {
                end = Some(pos);
            }
            if ch != '#' {
                map.insert(pos);
            }
        }
    }
    (
        map,
        start.expect("Could not find start"),
        end.expect("could not find end"),
    )
}

fn dist(p: &Pos, q: &Pos) -> isize {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn solve_map(map: &HashSet<Pos>, start: Pos) -> HashMap<Pos, isize> {
    let mut score_map = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(Reverse(0), start)]);

    while let Some((score, pos)) = queue.pop() {
        if !map.contains(&pos) {
            continue;
        }
        let old_score = score_map.entry(pos).or_insert(isize::MAX);
        if score.0 < *old_score {
            *old_score = score.0;
            queue.extend([
                (Reverse(score.0 + 1), (pos.0 + 1, pos.1)),
                (Reverse(score.0 + 1), (pos.0 - 1, pos.1)),
                (Reverse(score.0 + 1), (pos.0, pos.1 + 1)),
                (Reverse(score.0 + 1), (pos.0, pos.1 - 1)),
            ])
        }
    }

    score_map
}

pub fn solve(map: &HashSet<Pos>, start: Pos, end: Pos, cheat_length: isize) -> usize {
    let score_map = solve_map(map, end);
    let mut cur_pos = start;
    let mut count = 0;
    let cheats = (-cheat_length..=cheat_length)
        .flat_map(|x| (-cheat_length..=cheat_length).map(move |y| (x, y)))
        .filter(|pos| dist(&(0, 0), pos) <= cheat_length)
        .collect::<Vec<Pos>>();
    while cur_pos != end {
        let cur_score = score_map.get(&cur_pos).unwrap().clone();
        for &cheat in cheats.iter() {
            let cheat_pos = (cur_pos.0 + cheat.0, cur_pos.1 + cheat.1);
            if let Some(&cheat_score) = score_map.get(&cheat_pos) {
                if (cur_score - cheat_score) - dist(&cur_pos, &cheat_pos) >= 100 {
                    count += 1;
                }
            }
        }

        cur_pos = score_map.iter()
            .filter(|&(next_pos, next_score)| next_score + 1 == cur_score && dist(next_pos, &cur_pos) == 1)
            .map(|(&pos, _)| pos)
            .next()
            .unwrap();
    }

    count

}

fn part_1(map: &HashSet<Pos>, start: Pos, end: Pos) -> usize {
    solve(map, start, end, 2)
}

fn part_2(map: &HashSet<Pos>, start: Pos, end: Pos) -> usize {
    solve(map, start, end, 20)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (map, start, end) = parse(&input);
    println!("part_1 = {}", part_1(&map, start, end));
    println!("part_2 = {}", part_2(&map, start, end));
}
