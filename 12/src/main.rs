use std::collections::{BTreeSet, HashSet};

type Point = (isize, isize);
type Map = BTreeSet<(Point, char)>;

fn part_1(map: &Map) -> usize {
    let mut map = map.clone();

    let mut sum = 0;
    while let Some(plot) = map.pop_first() {
        let mut perimeter = 0;
        let mut area = 0;

        let mut queue = vec![plot];
        let mut region = HashSet::new();
        while let Some((point, ch)) = queue.pop() {
            region.insert(point);
            area += 1;
            for (i, j) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let point = (point.0 + i, point.1 + j);
                let neighbor = (point, ch);
                if map.remove(&neighbor) {
                    queue.push(neighbor);
                    region.insert(point);
                } else if ! region.contains(&point) {
                    perimeter += 1;
                }
            }
        }

        // println!("plot = {:?}", plot);
        // println!("perimeter = {:?}", perimeter);
        // println!("area = {:?}", area);
        // println!("region = {:?}", region);
        sum += perimeter*area;
    }

    sum
}

fn part_2(map: &Map) -> usize {
    let mut map = map.clone();

    let mut sum = 0;
    while let Some(plot) = map.pop_first() {
        let mut region = BTreeSet::new();

        let mut queue = vec![plot];
        while let Some((point, ch)) = queue.pop() {
            region.insert(point);
            for (i, j) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let point = (point.0 + i, point.1 + j);
                let neighbor = (point, ch);
                if map.remove(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }

        let upper_left_cornor = region.iter()
            .fold((isize::MAX, isize::MAX), |cornor, point| (cornor.0.min(point.0), cornor.1.min(point.1)));
        let lower_right_cornor = region.iter()
            .fold((isize::MIN, isize::MIN), |cornor, point| (cornor.0.max(point.0), cornor.1.max(point.1)));

        let mut sides = 0;

        let mut upper_fence = false;
        let mut lower_fence = false;
        for x in upper_left_cornor.0..=lower_right_cornor.0 {
            for y in upper_left_cornor.1..=lower_right_cornor.1 {
                if region.contains(&(x, y)) {
                    if region.contains(&(x-1, y)) {
                        upper_fence = false;
                    } else if ! upper_fence {
                        upper_fence = true;
                        sides += 1;
                    }

                    if region.contains(&(x+1, y)) {
                        lower_fence = false;
                    } else if ! lower_fence {
                        lower_fence = true;
                        sides += 1;
                    }
                } else {
                    upper_fence = false;
                    lower_fence = false;
                }
            }
            upper_fence = false;
            lower_fence = false;
        }

        let mut right_fence = false;
        let mut left_fence = false;
        for y in upper_left_cornor.1..=lower_right_cornor.1 {
            for x in upper_left_cornor.0..=lower_right_cornor.0 {
                if region.contains(&(x, y)) {
                    if region.contains(&(x, y-1)) {
                        right_fence = false;
                    } else if ! right_fence {
                        right_fence = true;
                        sides += 1;
                    }

                    if region.contains(&(x, y+1)) {
                        left_fence = false;
                    } else if ! left_fence {
                        left_fence = true;
                        sides += 1;
                    }
                } else {
                    right_fence = false;
                    left_fence = false;
                }
            }
            right_fence = false;
            left_fence = false;
        }

        // println!("plot = {:?}", plot);
        // println!("sides = {:?}", sides);
        // println!("area = {:?}", region.len());
        // println!("region = {:?}", region);

        sum += sides*region.len();
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(x, line)| line
            .chars()
            .enumerate()
            .map(move |(y, ch)| ((x as isize, y as isize), ch))
        )
        .collect::<Map>();
    println!("part_1 = {}", part_1(&map));
    println!("part_2 = {}", part_2(&map));
}
