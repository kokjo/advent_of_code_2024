pub fn get(grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if x < 0 { return None }
    let x = x as usize;
    if x >= grid.len() { return None; }
    let line = &grid[x];
    if y < 0 { return None; }
    let y = y as usize;
    if y >= line.len() { return None; }
    Some(line[y])
}

pub fn part_1(grid: &Vec<Vec<char>>) {
    let mut count = 0;

    for x in (0..grid.len()) {
        for y in (0..grid[x].len()) {
            let offsetss: Vec<Vec<(isize, isize)>> = vec![
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 0), (0, -1), (0, -2), (0, -3)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
                vec![(0, 0), (1, 1), (2, 2), (3, 3)],
                vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
                vec![(0, 0), (1, -1), (2, -2), (3, -3)],
                vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            ];

            for offsets in offsetss {
                let chars = offsets.iter().map(|(a, b)| get(&grid, (x as isize)+a, (y as isize)+b)).collect::<Option<Vec<char>>>();
                if let Some(chars) = chars {
                    if chars == vec!['X', 'M', 'A', 'S'] {
                        count += 1;
                    }
                }
            }

        }
    }

    println!("count = {}", count);
}

pub fn part_2(grid: &Vec<Vec<char>>) {
    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'A' {
                let (x, y) = (x as isize, y as isize);
                match (get(grid, x-1, y-1), get(grid, x+1, y+1)) {
                    (Some('M'), Some('S')) => {},
                    (Some('S'), Some('M')) => {},
                    _ => { continue; }
                }
                match (get(grid, x-1, y+1), get(grid, x+1, y-1)) {
                    (Some('M'), Some('S')) => {},
                    (Some('S'), Some('M')) => {},
                    _ => { continue; }
                }
                count += 1;
            }
        }
    }

    println!("count = {}", count);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let grid: Vec<Vec<char>> = input.trim().split('\n').map(|line| line.chars().collect()).collect();
    part_1(&grid);
    part_2(&grid);
}
