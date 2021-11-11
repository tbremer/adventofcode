use utils;
#[derive(Debug, PartialEq)]
enum Status {
    Naughty,
    Nice,
}

fn main() {
    println!("Welcome to day-05 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::str_to_vec(&utils::read_file(utils::args().remove(0)), None);

    println!("Puzzle 1: {:?}", run(input_data.clone(), pt_1a));
    println!("Puzzle 2: {:?}", run(input_data, pt_2a));
}

fn run(input_list: Vec<String>, callback: fn(&str) -> Status) -> usize {
    input_list
        .iter()
        .fold(0, |acc, input| match callback(input) {
            Status::Nice => acc + 1,
            Status::Naughty => acc,
        })
}

fn pt_1a(input: &str) -> Status {
    let mut vowel_count = 0;
    let mut duplicate_count = 0;

    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return Status::Naughty;
    }

    for (idx, ch) in input.char_indices() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {} // do nothing if not a vowel.
        }

        match input.get(idx..(idx + 2)) {
            None => {} // do nothing if at end
            Some(p_ch) => {
                if ch == p_ch.chars().last().unwrap() {
                    duplicate_count += 1
                }
            }
        }
    }

    if vowel_count >= 3 && duplicate_count >= 1 {
        Status::Nice
    } else {
        Status::Naughty
    }
}

// fn pt_2(input: &str) -> usize {
//     0
// }

fn pt_2a(input: &str) -> Status {
    /*
     * TULPLE SHAPE
     * 0: It contains a pair of any two letters that appears at least twice in
     * the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but
     * not like aaa (aa, but it overlaps).
     * ------------------------------------------------------------------------
     * 1: It contains at least one letter which repeats with exactly one letter
     * between them, like xyx, abcdefeghi (efe), or even aaa.
     */
    let mut constraints = (false, false);

    for (idx, _ch) in input.char_indices() {
        match input.get(idx..(idx + 3)) {
            None => {}
            Some(triple) => {
                let mut t_ch = triple.chars();
                let first = t_ch.nth(0).unwrap();
                let second = t_ch.nth(0).unwrap();
                let third = t_ch.nth(0).unwrap();
                let f = format!("{}{}", first, second);

                let substr = input.get(idx + 2..).unwrap();

                // rule one
                if substr.contains(&f) {
                    constraints.0 = true;
                }

                // rule 2
                if first == third {
                    constraints.1 = true;
                }
            }
        };

        if constraints.0 == true && constraints.1 == true {
            return Status::Nice;
        }
    }
    Status::Naughty
}

fn test_1() {
    assert_eq!(pt_1a("ugknbfddgicrmopn"), Status::Nice);
    assert_eq!(pt_1a("aaa"), Status::Nice);
    assert_eq!(pt_1a("jchzalrnumimnmhp"), Status::Naughty);
    assert_eq!(pt_1a("haegwjzuvuyypxyu"), Status::Naughty);
    assert_eq!(pt_1a("dvszwmarrgswjxmb"), Status::Naughty);
    println!("Suite 1 passes!")
}

fn test_2() {
    assert_eq!(pt_2a("qjhvhtzxzqqjkmpb"), Status::Nice);
    assert_eq!(pt_2a("xxyxx"), Status::Nice);
    assert_eq!(pt_2a("uurcxstgmygtbstg"), Status::Naughty);
    assert_eq!(pt_2a("ieodomkazucvgmuy"), Status::Naughty);

    println!("Suite 2 passes!")
}
