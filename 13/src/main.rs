pub fn parse(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    input
        .trim()
        .split("\n\n")
        .map(|problem| {
            let numbers = problem.chars().filter(|&ch| ch.is_ascii_digit() || ch == ',' || ch == '\n').collect::<String>();
            let (button_a, rest) = numbers.split_once("\n").unwrap();
            let (button_b, prize) = rest.split_once("\n").unwrap();
            let (button_a_x, button_a_y) = button_a.split_once(',').unwrap();
            let (button_b_x, button_b_y) = button_b.split_once(',').unwrap();
            let (prize_x, prize_y) = prize.split_once(',').unwrap();
            (
                (button_a_x.parse().unwrap(), button_a_y.parse().unwrap()),
                (button_b_x.parse().unwrap(), button_b_y.parse().unwrap()),
                (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            )
        }).collect()
}

pub fn part_1(input: &str) -> i64 {
    let problems = parse(input);

    let mut sum = 0;
    for (button_a, button_b, prize) in problems {
        let m = (
            (button_a.0, button_a.1),
            (button_b.0, button_b.1)
        );

        let det = m.0.0 * m.1.1 - m.0.1 * m.1.0;

        if det == 0 { continue; }

        let a = (prize.0 * m.1.1 - prize.1 * m.1.0) / det;
        let b = (prize.1 * m.0.0 - prize.0 * m.0.1) / det;

        if a*button_a.0 + b*button_b.0 != prize.0 { continue; }

        if a*button_a.1 + b*button_b.1 != prize.1 { continue; }

        if a > 100 || b > 100 { continue; }

        sum += a*3 + b; 
    }

    sum
}

pub fn part_2(input: &str) -> i64 {
    let problems = parse(input);

    let mut sum = 0;
    for (button_a, button_b, prize) in problems {
        let prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);

        let m = (
            (button_a.0, button_a.1),
            (button_b.0, button_b.1)
        );

        let det = m.0.0 * m.1.1 - m.0.1 * m.1.0;

        if det == 0 { continue; }

        let a = (prize.0 * m.1.1 - prize.1 * m.1.0) / det;
        let b = (prize.1 * m.0.0 - prize.0 * m.0.1) / det;

        if a*button_a.0 + b*button_b.0 != prize.0 { continue; }

        if a*button_a.1 + b*button_b.1 != prize.1 { continue; }

        sum += a*3 + b; 
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
