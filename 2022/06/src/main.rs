use std::collections::HashSet;

fn main() {
    println!("Welcome to day-06 — Running test suite first!\n");
    let input_data = utils::read_file(utils::args().remove(0), None);

    test_1();
    println!("Puzzle 1: {:?}\n", solve(&input_data, 4));
    test_2();
    println!("Puzzle 2: {:?}\n", solve(&input_data, 14));
}

fn solve(input: &str, base: usize) -> usize {
    let ch = input.split("").collect::<Vec<&str>>();
    let sl = ch.windows(base);

    for (idx, win) in sl.enumerate() {
        let set: HashSet<String> = win
            .iter()
            .filter(|i| i.len() > 0)
            .map(|i| i.clone().to_owned())
            .collect();

        if set.len() == base {
            return idx + (base - 1);
        }
    }

    return 0;
}

fn test_1() {
    vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ]
    .iter()
    .for_each(|(input, expected)| assert_eq!(solve(input, 4), *expected));

    println!("Suite 1 passes");
}

fn test_2() {
    vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ]
    .iter()
    .for_each(|(input, expected)| assert_eq!(solve(input, 14), *expected));
    println!("Suite 2 passes");
}
