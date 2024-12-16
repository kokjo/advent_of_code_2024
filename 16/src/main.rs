use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, isize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir { N, S, E, W, }

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::N => Pos(self.0, self.1-1),
            Dir::S => Pos(self.0, self.1+1),
            Dir::E => Pos(self.0+1, self.1),
            Dir::W => Pos(self.0-1, self.1),
        }
    }
}

impl std::ops::Sub<Dir> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::N => Pos(self.0, self.1+1),
            Dir::S => Pos(self.0, self.1-1),
            Dir::E => Pos(self.0-1, self.1),
            Dir::W => Pos(self.0+1, self.1),
        }
    }
}

fn parse(input: &str) -> (HashMap<Pos, bool>, Pos, Pos) {
    let mut map = HashMap::new();
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            let pos = Pos(x as isize, y as isize);
            map.insert(pos, ch == '#');
            if ch == 'S' {
                start = pos;
            }
            if ch == 'E' {
                end = pos;
            }
        }
    }

    (map, start, end)
}

fn make_score_map(map: HashMap<Pos, bool>, start: Pos) -> HashMap<(Pos, Dir), isize> {
    let mut score_map: HashMap<(Pos, Dir), isize> = HashMap::new();

    let mut queue = BinaryHeap::from(vec![(Reverse(0), start, Dir::E)]);
    while let Some((score, pos, dir)) = queue.pop() {
        if map.get(&pos).cloned().unwrap_or(true) {
            continue;
        }
        let old_score = score_map.entry((pos, dir)).or_insert(isize::MAX);
        if score.0 < *old_score {
            //dbg!(pos, dir, score, *old_score, queue.len());
            *old_score = score.0;
            queue.extend([
                (Reverse(score.0 + 1), pos + dir, dir),
                (Reverse(score.0 + 1000), pos, dir.turn_left()),
                (Reverse(score.0 + 1000), pos, dir.turn_right()),
            ])
        }
    }

    score_map
}

fn part_1(score_map: &HashMap<(Pos, Dir), isize>, end: Pos) -> isize {
    score_map.iter()
        .filter_map(|((pos, _), score)| (*pos == end).then_some(score))
        .min().unwrap().clone()
}

fn part_2(score_map: &HashMap<(Pos, Dir), isize>, end: Pos) -> usize {
    let best_score = part_1(score_map, end);
    let mut best_seats = HashSet::new();
    let mut queue = [Dir::N, Dir::S, Dir::E, Dir::W].into_iter()
        .map(|dir| (best_score, end, dir))
        .collect::<Vec<_>>();
    while let Some((best_score, pos, dir)) = queue.pop() {
        if score_map.get(&(pos, dir)).cloned().unwrap_or(isize::MAX) != best_score {
            continue;
        }
        best_seats.insert(pos);
        queue.extend([
            (best_score - 1, pos-dir, dir),
            (best_score - 1000, pos, dir.turn_left()),
            (best_score - 1000, pos, dir.turn_right()),
        ])
    }

    best_seats.len()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (map, start, end) = parse(&input);
    let score_map = make_score_map(map, start);
    println!("part_1 = {}", part_1(&score_map, end));
    println!("part_2 = {}", part_2(&score_map, end));
}
