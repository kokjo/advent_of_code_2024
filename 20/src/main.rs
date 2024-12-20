use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

type Pos = (isize, isize);

fn parse(input: &str) -> (HashSet<Pos>, Pos, Pos, Pos) {
    let mut start = None;
    let mut end = None;
    let mut map = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
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
            if max_x < x {
                max_x = x;
            }
        }
        if max_y < y {
            max_y = y;
        }
    }
    (
        map,
        start.expect("Could not find start"),
        end.expect("could not find end"),
        (max_x as isize, max_y as isize)
    )
}

fn solve_map(map: &HashSet<Pos>, start: Pos) -> HashMap<Pos, usize> {
    let mut score_map = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(Reverse(0), start)]);

    while let Some((score, pos)) = queue.pop() {
        if !map.contains(&pos) {
            continue;
        }
        let old_score = score_map.entry(pos).or_insert(usize::MAX);
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

fn part_1(map: &HashSet<Pos>, start: Pos, end: Pos, bound: Pos) -> usize {
    let best_nocheat = solve_map(map, start).get(&end).unwrap().clone();

    let mut count = 0;
    for (x0, y0) in (0isize..bound.0).flat_map(|y| (0isize..bound.1).map(move |x| (x, y)) ){
        if map.contains(&(x0, y0)) {
            continue;
        }
        //dbg!((x0, y0));
        let mut map = map.clone();
        map.insert((x0, y0));
        if best_nocheat - solve_map(&map, start).get(&end).unwrap() >= 100 {
            count += 1;
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (map, start, end, bound) = parse(&input);
    println!("part_1 = {}", part_1(&map, start, end, bound))
}
