fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input.trim().split('\n').map(|line| {
        let (result, numbers) = line.split_once(':').unwrap();
        (result.parse().unwrap(), numbers.trim().split(' ').map(|number| number.parse().unwrap()).collect())
    }).collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Op { Add, Mul }

impl Op {
    pub fn inc(self) -> (bool, Self) {
        match self {
            Op::Add => (false, Op::Mul),
            Op::Mul => (true, Op::Add),
        }
    }
}

fn part_1(input: &str) -> i64 {
    let mut sum = 0;
    for (result, numbers) in parse(input) {
        let mut ops = vec![Op::Add; numbers.len()-1];
        for _ in 0..1 << ops.len() {
            for i in 0..ops.len() {
                let (carry, next) = ops[i].inc();
                ops[i] = next;
                if ! carry {
                    break
                }
            }
            let output = numbers[1..].iter().zip(&ops).fold(numbers[0], |acc, (number, op)| match op {
                Op::Add => acc + number,
                Op::Mul => acc * number,
            });
            if result == output {
                sum += output;
                break
            }
        }
    }
    sum
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Op2 { Add, Mul, Cat }

impl Op2 {
    pub fn inc(self) -> (bool, Self) {
        match self {
            Op2::Add => (false, Op2::Mul),
            Op2::Mul => (false, Op2::Cat),
            Op2::Cat => (true, Op2::Add),
        }
    }
}

fn part_2(input: &str) -> i64 {
    let mut sum = 0;
    for (result, numbers) in parse(input) {
        let mut ops = vec![Op2::Add; numbers.len()-1];
        for _ in 0..3usize.pow(ops.len() as u32) {
            for i in 0..ops.len() {
                let (carry, next) = ops[i].inc();
                ops[i] = next;
                if ! carry {
                    break
                }
            }
            let output = numbers[1..].iter().zip(&ops).fold(numbers[0], |acc, (number, op)| match op {
                Op2::Add => acc + number,
                Op2::Mul => acc * number,
                Op2::Cat => acc * (10i64.pow(number.to_string().len() as u32)) + number
            });
            if result == output {
                sum += output;
                break
            }
        }
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
