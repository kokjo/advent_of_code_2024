use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct UnorderedPair<'a>(&'a str, &'a str);

impl<'a> UnorderedPair<'a> {
    fn new(a: &'a str, b: &'a str) -> Self {
        Self(a.min(b), a.max(b))
    }
}

fn parse(input: &str) -> HashSet<UnorderedPair> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            line.trim()
                .split_once('-')
                .map(|(a, b)| UnorderedPair::new(a, b))
        })
        .collect()
}

fn part_1(pairs: &HashSet<UnorderedPair>) -> usize {
    let nodes: HashSet<&str> = pairs.iter().flat_map(|pair| [pair.0, pair.1]).collect();

    let mut sets_of_3 = HashSet::<[&str; 3]>::new();
    for &a in nodes.iter() {
        if ! a.starts_with('t') {
            continue;
        }
        for &b in nodes.iter() {
            if a == b {
                continue;
            }
            for &c in nodes.iter() {
                if c == a || c == b {
                    continue;
                }
                if pairs.contains(&UnorderedPair::new(a, b)) && pairs.contains(&UnorderedPair::new(b, c)) && pairs.contains(&UnorderedPair::new(c, a)) {
                    let mut triple = [a, b, c];
                    triple.sort();
                    sets_of_3.insert(triple);
                }
            }
        }
    }

    sets_of_3.len()
}

fn is_clique(pairs: &HashSet<UnorderedPair>, clique: &[&str]) -> bool {
    if let Some((&first, rest)) = clique.split_first() {
        rest.iter().all(|&other| pairs.contains(&UnorderedPair::new(first, other)))
            && is_clique(pairs, rest)
    } else {
        true
    }
}

fn build_largest_clique<'a>(pairs: &HashSet<UnorderedPair<'a>>, mut clique: Vec<&'a str>, adjs: &[&'a str]) -> Vec<&'a str> {
    if let Some((&first, rest)) = adjs.split_first() {
        let without_first = build_largest_clique(pairs, clique.clone(), rest);

        clique.push(first);
        if is_clique(pairs, &clique) {
            let with_first = build_largest_clique(pairs, clique, rest);

            if with_first.len() > without_first.len() {
                with_first
            } else {
                without_first
            }
        } else {
            without_first
        }
    } else {
        clique
    }
}

fn part_2(pairs: &HashSet<UnorderedPair>) -> String {
    let nodes: HashSet<&str> = pairs.iter().flat_map(|pair| [pair.0, pair.1]).collect();

    let mut best_clique = vec![];

    for &node in nodes.iter() {
        let adjs: Vec<&str> = pairs.iter().filter_map(|pair| (pair.0 == node).then(|| pair.1).or((pair.1 == node).then(|| pair.0))).collect();
        let clique = build_largest_clique(pairs, vec![node], &adjs);
        if clique.len() > best_clique.len() {
            best_clique = clique;
        }
    }

    best_clique.sort();
    best_clique.join(",")
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let pairs = parse(&input);

    println!("part_1 = {}", part_1(&pairs));
    println!("part_2 = {}", part_2(&pairs));
}
