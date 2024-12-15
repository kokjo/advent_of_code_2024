use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile { Wall, Box, Empty, }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Dir { U, D, L, R, }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::U => Pos(self.0, self.1 - 1),
            Dir::D => Pos(self.0, self.1 + 1),
            Dir::L => Pos(self.0 - 1, self.1),
            Dir::R => Pos(self.0 + 1, self.1),
        }
    }
}

impl std::ops::Sub<Dir> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::U => Pos(self.0, self.1 + 1),
            Dir::D => Pos(self.0, self.1 - 1),
            Dir::L => Pos(self.0 + 1, self.1),
            Dir::R => Pos(self.0 - 1, self.1),
        }
    }
}

type Map = HashMap<Pos, Tile>;
type Moves = Vec<Dir>;

fn parse(input: &str) -> (Map, Moves, Pos) {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut robot = Pos(0, 0);
    let mut map = Map::new();

    for (y, line) in map_str.trim().lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            let pos = Pos(x as isize, y.clone() as isize);
            let tile = match ch {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '.' => Tile::Empty,
                '@' => {
                    robot = pos;
                    Tile::Empty
                },
                _ => panic!("Unknown map tile")
            };
            map.insert(pos, tile);
        }
    }

    let moves = moves_str.trim().chars().filter(|&ch| ch != '\n').map(|ch| match ch {
        '^' => Dir::U,
        'v' => Dir::D,
        '<' => Dir::L,
        '>' => Dir::R,
        _ => panic!("Unknown move direction"),
    }).collect::<Vec<_>>();

    (map, moves, robot)
}

fn part_1(input: &str) -> isize {
    let (mut map, moves, mut robot) = parse(input);
    for dir in moves {
        let robot_next = robot + dir;
        let tile = map.get(&robot_next).unwrap();
        robot = match tile {
            Tile::Box => {
                let mut box_pos = robot_next + dir;
                while *map.get(&box_pos).unwrap() == Tile::Box {
                    box_pos = box_pos + dir;
                }
                match map.get(&box_pos).unwrap() {
                    Tile::Empty => {
                        map.insert(box_pos, Tile::Box);
                        map.insert(robot_next, Tile::Empty);
                        robot_next
                    },
                    Tile::Wall => robot,
                    _ => panic!("wut?")
                }
            }
            Tile::Empty => robot_next,
            Tile::Wall => robot,
        };
    }

    map.into_iter()
        .filter(|&(_, tile)| tile == Tile::Box)
        .map(|(pos, _)| pos.1 * 100 + pos.0)
        .sum::<isize>()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile2 {
    Empty, BoxL, BoxR, Wall
}

impl Tile2 {
    pub fn is_box(&self) -> bool {
        matches!(self, Tile2::BoxL | Tile2::BoxR)
    }

    pub fn is_wall(&self) -> bool {
        matches!(self, Tile2::Wall)
    }

    pub fn other_half(&self, pos: Pos) -> Pos {
        match self {
            Tile2::BoxL => Pos(pos.0 + 1, pos.1),
            Tile2::BoxR => Pos(pos.0 - 1, pos.1),
            _ => panic!("not a box!"),
        }
    }
}

type Map2 = HashMap<Pos, Tile2>;
impl Pos {
    pub fn l(&self) -> Pos {
        Pos(2*self.0 + 0, self.1)
    }

    pub fn r(&self) -> Pos {
        Pos(2*self.0 + 1, self.1)
    }
}

fn part_2(input: &str) -> isize {
    let (map, moves, robot) = parse(input);

    let mut robot = Pos(2*robot.0 + 0, robot.1);

    let mut map = Map2::from_iter(map.iter().flat_map(|(pos, tile)| match tile {
        Tile::Wall => vec![(pos.l(), Tile2::Wall), (pos.r(), Tile2::Wall)],
        Tile::Box => vec![(pos.l(), Tile2::BoxL), (pos.r(), Tile2::BoxR)],
        Tile::Empty => vec![(pos.l(), Tile2::Empty), (pos.r(), Tile2::Empty)],
    }));

    'dir_loop: for dir in moves {

        // for y in (0..11) {
        //     for x in (0..20) {
        //         let pos = Pos(x, y);
        //         let tile_str = if pos == robot {
        //             "@"
        //         } else {
        //             match map.get(&pos) {
        //                 Some(Tile2::BoxL) => "[",
        //                 Some(Tile2::BoxR) => "]",
        //                 Some(Tile2::Wall) => "#",
        //                 Some(Tile2::Empty) => ".",
        //                 None => "",
        //             }
        //         };
        //         print!("{}", tile_str);
        //     }
        //     print!("\n");
        // }

        // let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();

        let mut map_next = map.clone();
        let robot_next = robot + dir;

        let mut seen = HashSet::new();

        let mut queue = vec![robot_next];
        while let Some(pos) = queue.pop() {
            if !seen.insert(pos) {
                continue;
            }

            let tile = *map_next.get(&pos).unwrap();
            if tile.is_wall() {
                continue 'dir_loop;
            }
            if tile.is_box() {
                queue.push(tile.other_half(pos));
                queue.push(pos + dir);
            }
        }

        for &pos in seen.iter() {
            if seen.contains(&(pos-dir)) {
                map_next.insert(pos, *map.get(&(pos-dir)).unwrap());
            } else {
                map_next.insert(pos, Tile2::Empty);
            }
        }

        map = map_next;
        robot = robot_next;
    }

    map.into_iter()
        .filter(|&(_, tile)| tile == Tile2::BoxL)
        .map(|(pos, _)| pos.1 * 100 + pos.0)
        .sum::<isize>()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
