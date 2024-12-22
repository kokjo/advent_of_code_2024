use std::{collections::BinaryHeap, ops::Deref, path::Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    U, D, L, R,
}

impl std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::U => Pos(self.0, self.1-1),
            Dir::D => Pos(self.0, self.1+1),
            Dir::L => Pos(self.0-1, self.1),
            Dir::R => Pos(self.0+1, self.1),
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Dir::U => '^',
            Dir::D => 'v',
            Dir::L => '<',
            Dir::R => '>',
        })
    }
}

trait KeyPad: Eq + Sized + Copy + Clone + Default {
    fn from_pos(pos: Pos) -> Option<Self>;
    fn to_pos(&self) -> Pos;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DirKeyPad {
    Dir(Dir),
    A,
}

impl std::fmt::Display for DirKeyPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirKeyPad::Dir(dir) => write!(f, "{}", dir),
            DirKeyPad::A => write!(f, "A"),
        }
    }
}

impl Default for DirKeyPad {
    fn default() -> Self {
        Self::A
    }
}

impl KeyPad for DirKeyPad {
    fn from_pos(pos: Pos) -> Option<Self> {
        Some(match pos {
            Pos(1, 0) => Self::Dir(Dir::U),
            Pos(2, 0) => Self::A,
            Pos(0, 1) => Self::Dir(Dir::L),
            Pos(1, 1) => Self::Dir(Dir::D),
            Pos(2, 1) => Self::Dir(Dir::R),
            _ => return None,

        })
    }

    fn to_pos(&self) -> Pos {
        match self {
            DirKeyPad::Dir(Dir::U) => Pos(1, 0),
            DirKeyPad::Dir(Dir::L) => Pos(0, 1),
            DirKeyPad::Dir(Dir::D) => Pos(1, 1),
            DirKeyPad::Dir(Dir::R) => Pos(2, 1),
            DirKeyPad::A => Pos(2, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NumKeyPad {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    A
}

impl std::fmt::Display for NumKeyPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NumKeyPad::N0 => '0',
            NumKeyPad::N1 => '1',
            NumKeyPad::N2 => '2',
            NumKeyPad::N3 => '3',
            NumKeyPad::N4 => '4',
            NumKeyPad::N5 => '5',
            NumKeyPad::N6 => '6',
            NumKeyPad::N7 => '7',
            NumKeyPad::N8 => '8',
            NumKeyPad::N9 => '9',
            NumKeyPad::A => 'A',
        })
    }
}

impl std::convert::TryFrom<char> for NumKeyPad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => NumKeyPad::N0,
            '1' => NumKeyPad::N1,
            '2' => NumKeyPad::N2,
            '3' => NumKeyPad::N3,
            '4' => NumKeyPad::N4,
            '5' => NumKeyPad::N5,
            '6' => NumKeyPad::N6,
            '7' => NumKeyPad::N7,
            '8' => NumKeyPad::N8,
            '9' => NumKeyPad::N9,
            'A' => NumKeyPad::A,
            _ => return Err(()),
        })
    }
}

impl Default for NumKeyPad {
    fn default() -> Self {
        Self::A
    }
}

impl KeyPad for NumKeyPad {
    fn from_pos(pos: Pos) -> Option<Self> {
        Some(match pos {
            Pos(0, 0) => Self::N7,
            Pos(1, 0) => Self::N8,
            Pos(2, 0) => Self::N9,
            Pos(0, 1) => Self::N4,
            Pos(1, 1) => Self::N5,
            Pos(2, 1) => Self::N6,
            Pos(0, 2) => Self::N1,
            Pos(1, 2) => Self::N2,
            Pos(2, 2) => Self::N3,
            // no button at Pos(0, 3)
            Pos(1, 3) => Self::N0,
            Pos(2, 3) => Self::A,
            _ => return None,
        })
    }

    fn to_pos(&self) -> Pos {
        match self {
            NumKeyPad::N7 => Pos(0, 0),
            NumKeyPad::N8 => Pos(1, 0),
            NumKeyPad::N9 => Pos(2, 0),
            NumKeyPad::N4 => Pos(0, 1),
            NumKeyPad::N5 => Pos(1, 1),
            NumKeyPad::N6 => Pos(2, 1),
            NumKeyPad::N1 => Pos(0, 2),
            NumKeyPad::N2 => Pos(1, 2),
            NumKeyPad::N3 => Pos(2, 2),
            // no button at Pos(0, 3)
            NumKeyPad::N0 => Pos(1, 3),
            NumKeyPad::A => Pos(2, 3),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]

struct Path<KP: KeyPad>(Vec<KP>);

impl<KP: KeyPad + std::fmt::Display> std::fmt::Display for Path<KP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().map(|kp| write!(f, "{}", kp)).collect()
    }
}

impl<KP: KeyPad> std::ops::Deref for Path<KP> {
    type Target=Vec<KP>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<KP: KeyPad> std::ops::DerefMut for Path<KP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<KP: KeyPad> std::cmp::Ord for Path<KP> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

impl<KP: KeyPad> std::cmp::PartialOrd for Path<KP> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn route_keypad<KP: KeyPad>(a: KP, b: KP) -> Vec<Path<DirKeyPad>> {
    let mut routes = vec![];
    let mut queue = BinaryHeap::from([(Path(vec![]), a.to_pos())]);
    while let Some((mut path, pos)) = queue.pop() {
        if let Some(c) = KP::from_pos(pos) {
            if c == b {
                path.push(DirKeyPad::A);
                if routes.first().map(Path::deref).map(Vec::len).unwrap_or(usize::MAX) < path.len() {
                    break;
                }
                routes.push(path);
                continue;
            }
            for dir in [Dir::U, Dir::D, Dir::L, Dir::R] {
                let mut path = path.clone();
                path.push(DirKeyPad::Dir(dir));
                queue.push((path, pos + dir));
            }
        }
    }

    routes
}

fn solve_nested_keypad<KP: KeyPad>(a: KP, b: KP, layers: usize) -> Path<DirKeyPad> {
    if layers == 0 {
        route_keypad(a, b).into_iter().next().unwrap()
    } else {
        let mut solutions = vec![];
        let mut cur = DirKeyPad::default();
        for path in route_keypad(a, b) {
            let mut solution = Path(vec![]);
            for &next in path.iter() {
                solution.extend(solve_nested_keypad(cur, next, layers-1).deref());
                cur = next;
            }
            solutions.push(solution);
        }
        solutions.sort_by_key(|path| (path.len() as isize));
        solutions.into_iter().next().unwrap()
    }
}

fn solve_keycode(keycode: Vec<NumKeyPad>, layers: usize) -> Path<DirKeyPad> {
    let mut cur = NumKeyPad::default();
    let mut solution = Path(vec![]);
    for &next in keycode.iter() {
        solution.extend(solve_nested_keypad(cur, next, layers).deref());
        cur = next;
    }
    println!("{}: {}", Path(keycode), solution);

    solution
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.trim().lines() {
        let keycode = line.chars().map(NumKeyPad::try_from).collect::<Result<Vec<NumKeyPad>, ()>>().unwrap();
        sum += solve_keycode(keycode, 2).len() * &line[0..3].parse::<usize>().unwrap();
    }
    sum
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.trim().lines() {
        let keycode = line.chars().map(NumKeyPad::try_from).collect::<Result<Vec<NumKeyPad>, ()>>().unwrap();
        sum += solve_keycode(keycode, 25).len() * &line[0..3].parse::<usize>().unwrap();
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
