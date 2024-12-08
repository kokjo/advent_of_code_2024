use std::collections::{HashMap, HashSet};

type Point = (isize, isize);

struct Map {
    map: Vec<Vec<char>>,
    antennas: HashMap<char, HashSet<Point>>,
    max_x: isize,
    max_y: isize,
}

impl std::ops::Deref for Map {
    type Target = HashMap<char, HashSet<Point>>;

    fn deref(&self) -> &Self::Target {
        &self.antennas
    }
}

fn parse(input: &str) -> Map {
    let map: Vec<Vec<char>> = input.trim().split('\n').map(|line| line.trim().chars().collect()).collect();
    let max_x = map.len() as isize;
    let max_y = map[0].len() as isize;
    let mut antennas: HashMap<char, HashSet<Point>> = HashMap::new();
    for (x, y, ch) in map.iter().enumerate().flat_map(|(x, line)| line.iter().enumerate().map(move |(y, ch)| (x, y, *ch))).filter(|&(_, _, ch)| ch != '.') {
        antennas.entry(ch).or_default().insert((x as isize, y as isize));
    }
    Map { map, antennas, max_x, max_y, }
}

fn part_1(input: &str) -> u64 {
    let map = parse(input);

    let mut antinodes = HashSet::new();

    for (ch, antennas) in map.antennas {
        antinodes.extend(
            antennas.iter()
            .flat_map(|a| antennas.iter()
                .flat_map(move |&b| {
                    if *a != b {
                        let d = (b.0 - a.0, b.1 - a.1);
                        let p = (a.0 - d.0, a.1 - d.1);
                        let q = (b.0 + d.0, b.1 + d.1);
                        vec![p, q]
                    } else {
                        vec![]
                    }
                })
            )
            .filter(|&p| 0 <= p.0 && p.0 < map.max_x && 0 <= p.1 && p.1 < map.max_y)
        );
    }

    antinodes.len() as u64
}

fn part_2(input: &str) -> u64 {
    let map = parse(input);

    let mut antinodes = HashSet::new();

    for (ch, antennas) in map.antennas {
        antinodes.extend(
            antennas.iter()
            .flat_map(|a| antennas.iter()
                .filter(move |&b| a != b)
                .flat_map(move |&b| {
                    (-100..100)
                        .filter_map(move |i| {
                            let d = (b.0 - a.0, b.1 - a.1);
                            let p = (a.0 + i*d.0, a.1 + i*d.1);
                            Some(p)
                        })
                })
            )
            .filter(|&p| 0 <= p.0 && p.0 < map.max_x && 0 <= p.1 && p.1 < map.max_y)
        );
    }

    antinodes.len() as u64
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
