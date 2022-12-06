use std::ops::Range;

fn main() {
    println!("Welcome to day-04 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0), None);

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> u32 {
    parse_input(input).iter().fold(0, |acc, cur| {
        if contains_all(&cur.0, &cur.1) || contains_all(&cur.1, &cur.0) {
            acc + 1
        } else {
            acc
        }
    })
}

fn pt_2(input: &str) -> u32 {
    parse_input(input).iter().fold(0, |acc, cur| {
        if overlaps(&cur.0, &cur.1) {
            acc + 1
        } else {
            acc
        }
    })
}

fn test_1() {
    assert_eq!(
        pt_1(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        ),
        2
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
        ),
        4
    );
    println!("Suite 2 passes");
}

fn parse_input(input: &str) -> Vec<(Range<u32>, Range<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut rngs = line.split(",");
            let a = strs_to_rng(rngs.next().unwrap());
            let b = strs_to_rng(rngs.last().unwrap());

            (a, b)
        })
        .collect()
}

fn strs_to_rng(i: &str) -> Range<u32> {
    let mut nums = i.split("-");
    let s: u32 = nums.next().unwrap().parse().unwrap();
    let e: u32 = nums.last().unwrap().parse().unwrap();

    Range { start: s, end: e }
}

fn contains_all(a: &Range<u32>, b: &Range<u32>) -> bool {
    if a.start > b.start || a.end < b.end {
        false
    } else {
        true
    }
}

fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    if a.start == b.start || a.end == b.end {
        true
    } else if a.contains(&b.start) {
        true
    } else if a.contains(&b.end) {
        true
    } else if b.contains(&a.start) {
        true
    } else if b.contains(&a.end) {
        true
    } else {
        false
    }
}
