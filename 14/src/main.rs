const W: i64 = 101;
const H: i64 = 103;

type Vector = (i64, i64);

fn parse_vector(vector: &str) -> Vector {
    let (_, vector) = vector.split_once('=').unwrap();
    let (x, y) = vector.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse(input: &str) -> Vec<(Vector, Vector)> {
    input.trim().lines().map(|line| {
        let (p, v) = line.trim().split_once(' ').unwrap();
        (parse_vector(p), parse_vector(v))
    }).collect()
}

fn part_1(robots: &[(Vector, Vector)]) -> i64 {
    let robots = robots.iter().map(|&(p, v)| {
        ((p.0 + 100*v.0).rem_euclid(W), (p.1 + 100*v.1).rem_euclid(H))
    });

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots {
        if robot.0 < 50 && robot.1 < 51 { q1 += 1; }
        if robot.0 > 50 && robot.1 < 51 { q2 += 1; }
        if robot.0 < 50 && robot.1 > 51 { q3 += 1; }
        if robot.0 > 50 && robot.1 > 51 { q4 += 1; }
    }

    q1 * q2 * q3 * q4
}

fn part_2(robots: &[(Vector, Vector)]) -> i64 {
    for i in 0..100 {
        let i = 88 + 103 * i; // remove line to find 88

        let robots = robots.iter().map(|&(p, v)| {
            ((p.0 + i*v.0).rem_euclid(W), (p.1 + i*v.1).rem_euclid(H))
        });
        let mut map = vec![vec![' '; W as usize]; H as usize];
        for robot in robots {
            map[robot.1 as usize][robot.0 as usize] = '#';
        }
        let map = String::from_iter(map.into_iter().map(|line| String::from_iter(line) + "\n"));
        println!("{}", map);
        println!("i = {}", i);
        let mut line= String::new();

        std::io::stdin().read_line(&mut line).unwrap();
    }

    0
}

fn main() {
    let robots = parse(&std::fs::read_to_string("input").unwrap());

    println!("part_1 = {}", part_1(&robots));
    println!("part_2 = {}", part_2(&robots));
}