fn part_1(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.trim().split("\n\n").for_each(|sch| {
        let lines = sch
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|ch| (ch == '#').then_some(1).unwrap_or(0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let heights = lines[1..6].into_iter().fold(vec![0; 5], |b, l| {
            b.iter().zip(l).map(|(a, b)| a + b).collect()
        });
        if lines[0].iter().all(|&x| x == 1) && lines[6].iter().all(|&x| x == 0) {
            locks.push(heights);
        } else if lines[0].iter().all(|&x| x == 0) && lines[6].iter().all(|&x| x == 1) {
            keys.push(heights);
        } else {
            panic!("invalid schematic");
        }
    });

    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.iter().zip(lock).map(|(a, b)| a + b).all(|x| x < 6) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
}
