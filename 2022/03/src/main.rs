use day_03::{parse_input, parse_pt_1, parse_pt_2, str_val, string_to_hashset};
use std::collections::HashSet;

fn main() {
    println!("Welcome to day-03 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));
    let input = parse_input(&input_data);

    println!("Puzzle 1: {:?}", pt_1(input.pt1));
    println!("Puzzle 2: {:?}", pt_2(input.pt2));
}

fn pt_1(input: Vec<(String, String)>) -> u32 {
    input.iter().fold(0, |acc, cur| {
        let a: HashSet<String> = cur.0.chars().map(|c| c.to_string()).collect();
        let b: HashSet<String> = cur.1.chars().map(|c| c.to_string()).collect();
        let int = a.intersection(&b).next().unwrap();

        acc + str_val(int.as_str())
    })
}
fn pt_2(input: Vec<(String, String, String)>) -> u32 {
    input.iter().fold(0, |acc, cur| {
        let a = string_to_hashset(&cur.0);
        let b = string_to_hashset(&cur.1);
        let c = string_to_hashset(&cur.2);

        let ab: HashSet<&String> = a.intersection(&b).collect();
        let bc: HashSet<&String> = b.intersection(&c).collect();
        let int = ab.intersection(&bc).nth(0).unwrap();

        acc + str_val(int.as_str())
    })
}

fn test_1() {
    assert_eq!(
        pt_1(parse_pt_1(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        )),
        157
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(parse_pt_2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        )),
        70
    );
    println!("Suite 2 passes");
}
