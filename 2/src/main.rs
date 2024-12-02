fn is_safe(report: &[i64]) -> bool {
    let mut prev_level = report[0];
    let mut diff = vec![];
    for level in &report[1..] {
        diff.push(prev_level - level);
        prev_level = *level;
    }
    diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (1..=3).contains(&-x))
}

fn is_almost_safe(report: &[i64]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let fixed_report: Vec<i64> = report
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(j, level)| if i == j { None } else { Some(level) })
            .collect();
        if is_safe(&fixed_report) {
            return true;
        }
    }
    false
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reports: Vec<Vec<i64>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let mut safe = 0;
    let mut not_safe = 0;
    let mut almost_safe = 0;

    for report in reports.iter() {
        if is_safe(report) {
            safe += 1;
        } else {
            not_safe += 1;
        }
        if is_almost_safe(report) {
            almost_safe += 1;
        }
    }

    println!("safe = {}", safe);
    println!("unsafe = {}", not_safe);
    println!("almost safe = {}", almost_safe);
}
