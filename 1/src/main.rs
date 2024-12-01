fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let (mut xs, mut ys) = input
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .map(|(x, y)| (x.trim().parse().unwrap(), y.trim().parse().unwrap()))
        .unzip::<i64, i64, Vec<_>, Vec<_>>();

    xs.sort();
    ys.sort();

    let part_1 = std::iter::zip(xs.iter(), ys.iter())
        .map(|(x, y)| (x-y).abs())
        .sum::<i64>();
    println!("{}", part_1);

    let part_2 = xs
        .iter()
        .map(|x| ys
            .iter()
            .map(|y| 
                (x == y) as i64
            ).sum::<i64>() * x
        ).sum::<i64>();
    println!("{}", part_2);
}
