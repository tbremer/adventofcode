use std::collections::HashSet;

fn main() {
    println!("Welcome to day-08 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> usize {
    input
        .lines()
        .map(|i| i.split(" | ").nth(1).unwrap())
        .fold(0, |acc, cur| {
            let count = cur
                .split_whitespace()
                .filter(|i| {
                    let len = i.len();

                    if len == 2 || len == 3 || len == 4 || len == 7 {
                        true
                    } else {
                        false
                    }
                })
                .count();

            acc + count
        })
}

fn pt_2(input: &str) -> usize {
    let mut count = vec![];
    let lines = input.lines();

    for line in lines {
        let mut inner_count = vec![];
        let sp: Vec<&str> = line.split(" | ").collect();
        let left = sp[0];
        let right = sp[1];

        // populate vector with known shapes
        let mut sets = vec![
            None,
            Some(into_set(find_by_length(left, 2))),
            None,
            None,
            Some(into_set(find_by_length(left, 4))),
            None,
            None,
            Some(into_set(find_by_length(left, 3))),
            Some(into_set(find_by_length(left, 7))),
            None,
        ];

        // l_shape is the difference of 4 & 1
        let l_shape: HashSet<&str> = sets[4]
            .clone()
            .unwrap()
            .difference(&sets[1].clone().unwrap())
            .map(|i| *i)
            .collect();

        // parse into known numbers using the l_shape and length of item
        for l_item in left.split_whitespace() {
            let len = l_item.len();

            if len == 2 || len == 4 || len == 3 || len == 7 {
                continue;
            }

            let as_set = into_set(l_item);

            if len == 6 {
                if l_shape.is_subset(&as_set) {
                    if sets[1].clone().unwrap().is_subset(&as_set) {
                        // 9 holds both L and 1 in it's shape
                        sets[9] = Some(as_set)
                    } else {
                        // 6 only holds L
                        sets[6] = Some(as_set)
                    }
                } else {
                    // 0 does not hold L in it's shape
                    sets[0] = Some(as_set)
                }
            } else if len == 5 {
                if l_shape.is_subset(&as_set) {
                    // 5 holds L in it's shape
                    sets[5] = Some(as_set);
                } else {
                    if sets[1].clone().unwrap().is_subset(&as_set) {
                        // 3 holds 1 in it's shape
                        sets[3] = Some(as_set);
                    } else {
                        // 2 does not hold 1 in it's shape
                        sets[2] = Some(as_set);
                    }
                }
            } else {
                panic!("&str ({}) of unknown length {}", l_item, len)
            }
        }

        for r_item in right.split_whitespace() {
            let as_set = into_set(r_item);
            let idx = sets
                .clone()
                .into_iter()
                .position(|x| x == Some(as_set.clone()));

            if let Some(v) = idx {
                inner_count.push(v.to_string())
            }
        }

        let str: String = inner_count.iter().map(|s| String::from(s)).collect();

        count.push(str.parse::<usize>().unwrap());
    }

    count.iter().fold(0, |acc, cur| acc + cur)
}

fn find_by_length(needle: &str, len: usize) -> &str {
    needle.split_whitespace().find(|s| s.len() == len).unwrap()
}

fn into_set(s: &str) -> HashSet<&str> {
    s.split("").filter(|i| i.len() > 0).collect()
}

fn test_1() {
    assert_eq!(
        pt_1(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ),
        26
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ),
        61229
    );
    println!("Suite 2 passes");
}
