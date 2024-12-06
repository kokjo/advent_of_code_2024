use std::collections::HashSet;

pub enum Direction {
    U, D, L, R
}

fn find_gaurd(map: &Vec<Vec<char>>) -> Option<(isize, isize, Direction)> {
    for (x, line) in map.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            match ch {
                '^' => return Some((x as isize, y as isize, Direction::U)),
                '>' => return Some((x as isize, y as isize, Direction::R)),
                'v' => return Some((x as isize, y as isize, Direction::D)),
                '<' => return Some((x as isize, y as isize, Direction::L)),
                _ => {}
            }
        }
    }
    None
}
trait MapGet {
    fn map_get(&self, x: isize, y: isize) -> Option<char>;
}

impl MapGet for Vec<Vec<char>> {
    fn map_get(&self, x: isize, y: isize) -> Option<char> {
        let max_x = self.len() as isize;
        let max_y = self[0].len() as isize;
        if 0 <= x && x < max_x && 0 <= y && y < max_y {
            Some(self[x as usize][y as usize])
        } else {
            None
        }
    }
}
fn solve_map(map: &Vec<Vec<char>>) -> Option<HashSet<(isize, isize)>> {
    let mut guard = find_gaurd(&map)?;
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    let max_x = map.len() as isize;
    let max_y = map[0].len() as isize;
    let mut i = 0;

    while 0 <= guard.0 && guard.0 < max_x && 0 <= guard.1 &&  guard.1 < max_y {
        positions.insert((guard.0, guard.1));
        let next = match guard {
            (x, y, Direction::U) => (x-1, y, Direction::U),
            (x, y, Direction::R) => (x, y+1, Direction::R),
            (x, y, Direction::D) => (x+1, y, Direction::D),
            (x, y, Direction::L) => (x, y-1, Direction::L),
        };

        guard = match map.map_get(next.0, next.1) {
            Some('#') => (guard.0, guard.1, match guard.2 {
                Direction::U => Direction::R,
                Direction::R => Direction::D,
                Direction::D => Direction::L,
                Direction::L => Direction::U,
            }),
            Some(_) => next,
            None => break,
        };
        i += 1;
        if i > max_x * max_y {
            return None
        }
    }

    Some(positions)
}

fn part_1(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input.trim().split('\n').map(|line| line.trim().chars().collect()).collect();
    solve_map(&map).unwrap().len() as i64
}

fn part_2(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input.trim().split('\n').map(|line| line.trim().chars().collect()).collect();
    let mut count = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '.' {
                let mut map = map.clone();
                map[x][y] = '#';
                if solve_map(&map).is_none() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(input.as_str()));
    println!("part_2 = {}", part_2(input.as_str()));
}
