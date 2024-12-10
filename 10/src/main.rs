use std::collections::{HashMap, HashSet};

type Point = (isize, isize);
type Level = u32;
type Map = HashMap<Point, Level>;

fn parse(input: &str) -> Map {
    HashMap::from_iter(input.trim()
        .split('\n')
        .enumerate()
        .flat_map(|(x, line)| line.chars()
            .enumerate()
            .map(move |(y, ch)| ((x as isize, y as isize), ch.to_digit(10).unwrap()))
        )
    )
}

fn solve<Paths: Default + Extend<(Point, Point)>>(map: &Map, mut queue: Vec<(Point, Level, Point)>) -> Paths {
    let mut paths = Paths::default();

    while let Some((start, level, cur)) = queue.pop() {
        if level == 9 {
            paths.extend([(start, cur)]);
            continue;
        }
        for (i, j) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let cur = (cur.0 + i, cur.1 + j);
            if map.get(&cur).cloned() == Some(level + 1) {
                queue.push((start, level + 1, cur));
            }
        }
    }

    paths
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = parse(&input);
    let trailheads = map.iter()
        .filter(|&(_, ch)| *ch == 0)
        .map(|(p, ch)| (p.clone(), ch.clone(), p.clone()))
        .collect::<Vec<_>>();
    println!("part_1 = {}", solve::<HashSet<_>>(&map, trailheads.clone()).len());
    println!("part_2 = {}", solve::<Vec<_>>(&map, trailheads).len());
}
