use std::collections::{HashMap, BTreeSet};

type Graph<'a> = HashMap<&'a str, BTreeSet<&'a str>>;

fn parse(input: &str) -> Graph {
    let mut adjs = Graph::new();
    input
        .trim()
        .lines()
        .filter_map(|line| line.trim().split_once('-'))
        .for_each(|(a, b)| {
            adjs.entry(a).or_default().insert(b);
            adjs.entry(b).or_default().insert(a);
        });
    adjs
}

fn part_1(graph: &Graph) -> usize {
    let mut clique3s = BTreeSet::new();
    for (&a, a_adj) in graph.iter() {
        if a.starts_with('t') {
            for &b in a_adj.iter() {
                if let Some(b_adj) = graph.get(b) {
                    for &c in a_adj.intersection(b_adj) {
                        let mut clique3 = [a, b, c];
                        clique3.sort();
                        clique3s.insert(clique3);
                    }
                }
            }
        }
    }
    clique3s.len()
}

fn build_largest_clique<'a>(graph: &Graph, mut clique: Vec<&'a str>, mut adjs: BTreeSet<&'a str>, cur_best_size: usize) -> Vec<&'a str> {
    if clique.len() + adjs.len() <= cur_best_size {
        return vec![]
    }
    if let Some(first) = adjs.pop_first() {
        let without_first = build_largest_clique(graph, clique.clone(), adjs.clone(), cur_best_size);

        if let Some(first_adjs) = graph.get(first) {
            adjs.retain(|&adj| first_adjs.contains(adj));
        } else {
            adjs.clear()
        }

        clique.push(first);

        let with_first = build_largest_clique(graph, clique, adjs, cur_best_size);
        if with_first.len() > without_first.len() {
            with_first
        } else {
            without_first
        }
    } else {
        clique
    }
}

fn part_2(graph: &Graph) -> String {
    let mut best_clique = vec![];

    for (node, adjs) in graph.iter() {
        let clique = build_largest_clique(graph, vec![node], adjs.clone(), best_clique.len());
        if clique.len() > best_clique.len() {
            best_clique = clique;
        }
    }

    best_clique.sort();
    best_clique.join(",")
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let graph = parse(&input);

    println!("part_1 = {}", part_1(&graph));
    println!("part_2 = {}", part_2(&graph));
}
