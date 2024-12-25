use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    And, Or, Xor,
}

impl std::str::FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err(())
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Gate<'a> {
    op: Op,
    srca: &'a str,
    srcb: &'a str,
    dst: &'a str,
}

fn parse<'a>(input: &'a str) -> (HashMap<&'a str, bool>, Vec<Gate<'a>>) {
    let (initial, circut) = input.trim().split_once("\n\n").unwrap();
    let initial = initial.trim().lines().map(|line| {
        let (dst, value) = line.split_once(':').unwrap();
        (dst.trim(), value.trim() == "1")
    }).collect();
    let circut = circut.trim().lines().map(|line| {
        let line = line.trim().split(' ').collect::<Vec<_>>();
        Gate {
            op: line[1].parse().unwrap(),
            srca: line[0].trim(),
            srcb: line[2].trim(),
            dst: line[4].trim()
        }
    }).collect();

    (initial, circut)
}

fn part_1(input: &str) -> usize {
    let (mut gate_values, circut) = parse(input);
    loop {
        let mut changed = false;
        for gate in circut.iter() {
            let srca = gate_values.get(gate.srca);
            let srcb = gate_values.get(gate.srcb);
            if let (Some(srca), Some(srcb)) = (srca, srcb) {
                let dst_value = match gate.op {
                    Op::And => srca & srcb,
                    Op::Or => srca | srcb,
                    Op::Xor => srca ^ srcb,
                };
                changed = changed || gate_values.insert(gate.dst, dst_value).is_none()
            }
        }
        if ! changed {
            break;
        }
    }
    let z = (0..100)
        .filter_map(|i| gate_values.get(format!("z{:02}", i).as_str()).copied())
        .enumerate()
        .map(|(i, bit)| (bit as usize) << i)
        .sum();

    z
}

type Circut<'a> = HashMap<&'a str, Gate<'a>>;

#[derive(Debug, Clone)]
struct LoopDetected;

fn eval_circut<'b, 'a: 'b>(circut: &Circut<'a>, memo: &mut HashMap<&'b str, bool>, wire: &'b str, path: &mut HashSet<&'b str>) -> Result<Option<bool>, LoopDetected> {
    if let Some(&wire_value) = memo.get(wire) {
        return Ok(Some(wire_value));
    }

    if ! path.insert(wire) {
        return Err(LoopDetected);
    }

    let dst_value = if let Some(gate) = circut.get(wire) {
        if let Some(srca) = eval_circut(circut, memo, gate.srca, path)? {
            if let Some(srcb) = eval_circut(circut, memo, gate.srcb, path)? {
                let dst_value = match gate.op {
                    Op::And => srca & srcb,
                    Op::Or => srca | srcb,
                    Op::Xor => srca ^ srcb,
                };

                memo.insert(wire, dst_value);
                Some(dst_value)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    path.remove(wire);
    Ok(dst_value)
}

fn run_circut(circut: &Circut, x: u64, y: u64) -> Result<u64, LoopDetected> {
    let mut memo = HashMap::new();
    for i in 0..64 {
        memo.insert(format!("x{:02}", i), ((x >> i) & 1) == 1);
        memo.insert(format!("y{:02}", i), ((y >> i) & 1) == 1);
    }

    let z = (0..64).map(|n| format!("z{:02}", n)).collect::<Vec<_>>();

    let mut memo = memo.iter().map(|(name, &bit)| (name.as_str(), bit)).collect::<HashMap<_, _>>();

    Ok(z.iter()
        .map(|zn| eval_circut(circut, &mut memo, zn.as_str(), &mut HashSet::new()))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .enumerate()
        .filter_map(|(i, bit)| bit.map(|bit| (bit as u64) << i))
        .sum::<u64>())
}

fn get_deps<'a>(gates: &Circut<'a>, gate_name: &str) -> HashSet<&'a str> {
    let mut deps = HashSet::new();
    gates.get(gate_name).iter().for_each(|gate| {
        deps.extend([gate.dst]);
        deps.extend(get_deps(gates, &gate.srca));
        deps.extend(get_deps(gates, &gate.srcb));
    });
    deps
}

fn make_error_mask(circut: &Circut) -> Result<u64, LoopDetected> {
    let mut mask = 0;
    for n in 0..44 {
        for i in 0..4 {
            for j in 0..4 {
                let x = i << n;
                let y = j << n;
                let z_actual = run_circut(circut, x, y)?;
                let z_expect = (x + y) & ((1 << 46) - 1);
                mask |= z_actual ^ z_expect;
            }
        }
    }
    Ok(mask)
}

fn score_circut(circut: &Circut) -> Result<u64, LoopDetected> {
    Ok(make_error_mask(circut)?.count_ones() as u64)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScoredCircut<'a>(u64, Circut<'a>, Vec<&'a str>);

impl<'a> std::cmp::Ord for ScoredCircut<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<'a> std::cmp::PartialOrd for ScoredCircut<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn part_2(input: &str) -> String {
    let (_, circut) = parse(input);
    let circut = circut.iter().map(|gate| (gate.dst, gate.clone())).collect::<HashMap<&str, Gate>>();

    let score = score_circut(&circut).unwrap();
    let mut queue = BinaryHeap::from([ScoredCircut(score, circut, vec![])]);

    while let Some(mut cur) = queue.pop() {
        if cur.2.len() > 8 {
            continue
        }
        if cur.0 == 0 && cur.2.len() == 8 {
            cur.2.sort();
            return cur.2.join(",");
        }

        let all_gates = cur.1.keys().cloned().collect::<HashSet<_>>();
        let error_mask = make_error_mask(&cur.1).unwrap();
        let first_error = error_mask.trailing_zeros() as i32;
        let bad_deps = get_deps(&cur.1, &format!("z{:02}", first_error));
        let good_deps = get_deps(&cur.1, &format!("z{:02}", first_error-1));

        for &gate_a_name in bad_deps.difference(&good_deps) {
            for &gate_b_name in all_gates.difference(&good_deps) {
                if cur.2.contains(&gate_b_name) || cur.2.contains(&gate_a_name) {
                    continue
                }

                let mut gate_a = cur.1.get(gate_a_name).unwrap().clone();
                let mut gate_b = cur.1.get(gate_b_name).unwrap().clone();

                (gate_b.dst, gate_a.dst) = (gate_a.dst, gate_b.dst);

                let mut new_circut = cur.1.clone();
                new_circut.insert(gate_a.dst, gate_a);
                new_circut.insert(gate_b.dst, gate_b);

                if let Ok(score) = score_circut(&new_circut) {
                    let mut path = cur.2.clone();
                    path.extend([gate_a_name, gate_b_name]);
                    queue.push(ScoredCircut(score, new_circut, path));
                }
            }
        }
    }

    panic!("could not solve")
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_1 = {}", part_1(&input));
    println!("part_2 = {}", part_2(&input));
}
