use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Welcome to day-07 — Running test suite first!");
    test_1();
    // test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data1 = utils::read_file(utils::args().remove(0));
    let input_data2 = utils::read_file(utils::args().remove(1));

    println!("Puzzle 1: {:?}", pt_1(&input_data1).get("a"));
    println!("Puzzle 2: {:?}", pt_1(&input_data2).get("a"));
}

fn pt_1(input: &str) -> HashMap<String, u16> {
    // regex definitiions
    let re_initializer_int = Regex::new(r"^(?P<int>\d{1,}) -> (?P<t>\w{1,})").unwrap();
    let re_initializer_wire = Regex::new(r"^(?P<wire>[a-z]{1,}) -> (?P<t>[a-z]{1,})").unwrap();
    let re_and_int =
        Regex::new(r"^(?P<l>\d{1,}) AND (?P<r>[a-z]{1,}) -> (?P<t>[a-z]{1,})").unwrap();
    let re_and = Regex::new(r"^(?P<l>[a-z]{1,}) AND (?P<r>[a-z]{1,}) -> (?P<t>[a-z]{1,})").unwrap();
    let re_or = Regex::new(r"^(?P<l>.+) OR (?P<r>.+) -> (?P<t>[a-z]{1,})").unwrap();
    let re_not = Regex::new(r"^NOT (?P<r>.+) -> (?P<t>[a-z]{1,})").unwrap();
    let re_lshift = Regex::new(r"^(?P<l>.+) LSHIFT (?P<r>.+) -> (?P<t>[a-z]{1,})").unwrap();
    let re_rshift = Regex::new(r"^(?P<l>.+) RSHIFT (?P<r>.+) -> (?P<t>[a-z]{1,})").unwrap();
    // end regex definitiions

    let mut map = HashMap::new();
    let mut idx: usize = 0;
    let mut v: Vec<&str> = input.split("\n").collect();

    loop {
        if v.len() == 0 {
            break;
        }

        if idx >= v.len() {
            idx = 0
        }
        let item = v[idx];

        if re_initializer_int.is_match(item) {
            let cap = re_initializer_int.captures(item).unwrap();
            let key = String::from(&cap["t"]);
            let value = &cap["int"].parse::<u16>().unwrap();
            map.insert(key.clone(), value.clone());
            v.remove(idx);
            continue;
        } else if re_initializer_wire.is_match(item) {
            let cap = re_initializer_wire.captures(item).unwrap();
            let key = String::from(&cap["t"]);
            let wire = String::from(&cap["wire"]);

            if let Some(&val) = map.get(&wire) {
                map.insert(key, val.clone());
                v.remove(idx);
            } else {
                idx += 1;
            }
            continue;
        } else if re_and.is_match(item) {
            let cap = re_and.captures(item).unwrap();
            let l = String::from(&cap["l"]);
            let r = String::from(&cap["r"]);
            let t = String::from(&cap["t"]);

            if let Some(&m_l) = map.get(&l) {
                if let Some(&m_r) = map.get(&r) {
                    map.insert(t, m_l & m_r);
                    v.remove(idx);
                    continue;
                }
            }
        } else if re_and_int.is_match(item) {
            let cap = re_and_int.captures(item).unwrap();
            let l = &cap["l"].parse::<u16>().unwrap();
            let r = String::from(&cap["r"]);
            let t = String::from(&cap["t"]);

            if let Some(&m_r) = map.get(&r) {
                map.insert(t, l & m_r);
                v.remove(idx);
                continue;
            }
        } else if re_or.is_match(item) {
            let cap = re_or.captures(item).unwrap();
            let l = String::from(&cap["l"]);
            let r = String::from(&cap["r"]);
            let t = String::from(&cap["t"]);

            if let Some(&m_l) = map.get(&l) {
                if let Some(&m_r) = map.get(&r) {
                    map.insert(t, m_l | m_r);
                    v.remove(idx);
                    continue;
                }
            }
        } else if re_not.is_match(item) {
            let cap = re_not.captures(item).unwrap();
            let r = String::from(&cap["r"]);
            let t = String::from(&cap["t"]);

            if let Some(&m_r) = map.get(&r) {
                map.insert(t, !m_r);
                v.remove(idx);
                continue;
            }
        } else if re_lshift.is_match(item) {
            let cap = re_lshift.captures(item).unwrap();
            let l = String::from(&cap["l"]);
            let r = &cap["r"].parse::<u16>().unwrap();
            let t = String::from(&cap["t"]);

            if let Some(&m_l) = map.get(&l) {
                map.insert(t, m_l << r);
                v.remove(idx);
                continue;
            }
        } else if re_rshift.is_match(item) {
            let cap = re_rshift.captures(item).unwrap();
            let l = String::from(&cap["l"]);
            let r = &cap["r"].parse::<u16>().unwrap();
            let t = String::from(&cap["t"]);

            if let Some(&m_l) = map.get(&l) {
                map.insert(t, m_l >> r);
                v.remove(idx);
                continue;
            }
        } else {
            println!("Unknown item: {}", item);
        }

        idx += 1;
    }

    map
}

//fn pt_2(_input: &str) -> () {}

fn test_1() {
    assert_eq!(
        pt_1(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"
        ),
        vec![
            (String::from("d"), 72),
            (String::from("e"), 507),
            (String::from("f"), 492),
            (String::from("g"), 114),
            (String::from("h"), 65412),
            (String::from("i"), 65079),
            (String::from("x"), 123),
            (String::from("y"), 456),
        ]
        .into_iter()
        .collect::<HashMap<String, u16>>()
    );
    println!("Suite 1 passes");
}
//fn test_2() {
//    println!("Suite 2 passes");
//}
