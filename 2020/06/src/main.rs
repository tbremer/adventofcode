use std::collections::HashSet;
use std::iter::FromIterator;
use utils;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let input = file.split("\n\n");

    let pt1 = input
        .clone()
        .map(|l| HashSet::<char>::from_iter(l.replace("\n", "").chars()))
        .fold(0, |acc, x| acc + x.len());

    println!("pt1: {}", pt1);

    let pt2 = input
        .clone()
        .map(|line| count_group(line))
        .fold(0, |a, b| a + b);

    println!("pt2: {}", pt2);
}

fn count_group(line: &str) -> usize {
    let all_answ: Vec<HashSet<char>> = line
        .split("\n")
        .map(|l| HashSet::from_iter(l.chars()))
        .collect();

    let all_yes = all_answ
        .iter()
        .fold(HashSet::new(), |acc, set| acc.union(set).cloned().collect());

    all_answ
        .iter()
        .fold(all_yes, |acc, set| acc.intersection(set).cloned().collect())
        .len()
}
