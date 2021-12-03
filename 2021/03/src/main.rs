use std::collections::HashMap;

fn main() {
    println!("Welcome to day-03 — Running test suite first!");

    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> usize {
    let map = build_map(input);
    let mut gamma = vec!['c'; map.len()];
    let mut epsilon = vec!['c'; map.len()];

    for (idx, counts) in map.iter() {
        let i = *idx;
        if counts.0 > counts.1 {
            gamma[i] = '1';
            epsilon[i] = '0';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';
        }
    }

    let g: String = gamma.into_iter().collect();
    let e: String = epsilon.into_iter().collect();

    let g_num = usize::from_str_radix(&g, 2).unwrap();
    let e_num = usize::from_str_radix(&e, 2).unwrap();

    g_num * e_num
}

fn pt_2(input: &str) -> usize {
    let mut ox_rating: Vec<Option<String>> = input.lines().map(|l| Some(String::from(l))).collect();
    let mut co2_rating = ox_rating.clone().to_owned();
    let len = ox_rating[0].clone().unwrap().len();

    for bit_pos in 0..len {
        let high = get_significant_bits(&ox_rating, bit_pos, true);
        let low = get_significant_bits(&co2_rating, bit_pos, false);

        if get_some_count(&ox_rating) > 1 {
            for idx in 0..ox_rating.len() {
                if get_some_count(&ox_rating) > 1 {
                    let line = ox_rating[idx].as_ref();
                    if let Some(l) = line {
                        let char = char_to_sigbit(l.chars().nth(bit_pos).unwrap());
                        if high != char {
                            ox_rating[idx] = None;
                        }
                    }
                }
            }
        }

        if get_some_count(&co2_rating) > 1 {
            for idx in 0..co2_rating.len() {
                if get_some_count(&co2_rating) > 1 {
                    let line = co2_rating[idx].as_ref();
                    if let Some(l) = line {
                        let char = char_to_sigbit(l.chars().nth(bit_pos).unwrap());
                        if low != char {
                            co2_rating[idx] = None;
                        }
                    }
                }
            }
        }
    }

    let ox: String = ox_rating
        .into_iter()
        .filter(|l| if let None = l { false } else { true })
        .collect::<Vec<Option<String>>>()[0]
        .as_ref()
        .unwrap()
        .to_owned();
    let co2: String = co2_rating
        .into_iter()
        .filter(|l| if let None = l { false } else { true })
        .collect::<Vec<Option<String>>>()[0]
        .as_ref()
        .unwrap()
        .to_owned();

    usize::from_str_radix(&ox, 2).unwrap() * usize::from_str_radix(&co2, 2).unwrap()
}

fn test_1() {
    assert_eq!(
        pt_1(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        ),
        198
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        ),
        230
    );
    println!("Suite 2 passes");
}

fn build_map(input: &str) -> HashMap<usize, (usize, usize)> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let chars = line.chars();

        for (idx, c) in chars.enumerate() {
            if acc.contains_key(&idx) == false {
                acc.insert(idx, (0, 0));
            }

            let mut tuple = acc.get(&idx).unwrap().to_owned();

            match c {
                '0' => tuple.0 += 1,
                '1' => tuple.1 += 1,
                unknown => panic!("Unknown char: {}", unknown),
            }

            acc.insert(idx, tuple);
        }

        acc
    })
}

#[derive(Debug, PartialEq)]
enum SigBit {
    Zero,
    One,
}
fn get_significant_bits(v: &Vec<Option<String>>, pos: usize, highest: bool) -> SigBit {
    let mut counts = (0, 0);
    for i in 0..v.len() {
        if let Some(l) = v[i].as_ref() {
            let c = l.chars().nth(pos).unwrap();

            if c == '0' {
                counts.0 += 1;
            } else {
                counts.1 += 1;
            }
        }
    }

    if highest {
        if counts.0 == counts.1 {
            SigBit::One
        } else if counts.0 > counts.1 {
            SigBit::Zero
        } else {
            SigBit::One
        }
    } else {
        if counts.0 == counts.1 {
            SigBit::Zero
        } else if counts.0 > counts.1 {
            SigBit::One
        } else {
            SigBit::Zero
        }
    }
    // SigBit::Zero
}

fn char_to_sigbit(c: char) -> SigBit {
    match c {
        '0' => SigBit::Zero,
        '1' => SigBit::One,
        v => panic!("Unknown char: {}", v),
    }
}

fn get_some_count(v: &Vec<Option<String>>) -> usize {
    v.iter()
        .filter(|i| match i {
            None => false,
            Some(_) => true,
        })
        .count()
}
