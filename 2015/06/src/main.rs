use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Instruction {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq)]
enum LightState {
    On,
    Off,
}

fn main() {
    println!("Welcome to day-06!");

    let input_data = utils::str_to_vec(&utils::read_file(utils::args().remove(0)), None);

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn split_get_n(str: &str, needle: &str, n: usize) -> String {
    let tmp = str.replace(needle, "");
    let mut split = tmp.splitn(n, " ");

    split.nth(0).unwrap().to_string()
}

fn range_to_tuple(range: String) -> (usize, usize) {
    let mut sp = range.split(",");
    let min = sp.nth(0).unwrap();
    let max = sp.nth(0).unwrap();

    (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap())
}

fn parse_instructions(ins: &str) -> (Instruction, (usize, usize), (usize, usize)) {
    let instruction = if ins.starts_with("turn on") {
        Instruction::On
    } else if ins.starts_with("toggle") {
        Instruction::Toggle
    } else if ins.starts_with("turn off") {
        Instruction::Off
    } else {
        panic!("Unknown instruction: {}", ins);
    };
    let min_r = match instruction {
        Instruction::On => split_get_n(ins, "turn on ", 2),
        Instruction::Toggle => split_get_n(ins, "toggle ", 2),
        Instruction::Off => split_get_n(ins, "turn off ", 2),
    };
    let min_range = range_to_tuple(min_r);
    let mr = ins.split("through ").last().unwrap().to_string();
    let max_range = range_to_tuple(mr);

    (instruction, min_range, max_range)
}

fn iterate_over_ranges<F: FnMut((usize, usize)) -> ()>(
    min_range: (usize, usize),
    max_range: (usize, usize),
    mut cb: F,
) {
    for x in min_range.0..=max_range.0 {
        for y in min_range.1..=max_range.1 {
            cb((x, y));
        }
    }
}

fn pt_1(input: &Vec<String>) -> usize {
    let lights: HashMap<(usize, usize), LightState> =
        input.iter().fold(HashMap::new(), |mut acc, cur| {
            let (instruction, min_range, max_range) = parse_instructions(cur);
            iterate_over_ranges(min_range, max_range, |tup| {
                match instruction {
                    Instruction::Toggle => {
                        match acc.get(&tup) {
                            None => acc.insert(tup, LightState::On),
                            Some(v) => match v {
                                LightState::On => acc.insert(tup, LightState::Off),
                                LightState::Off => acc.insert(tup, LightState::On),
                            },
                        };
                    }
                    Instruction::On => {
                        acc.insert(tup, LightState::On);
                    }
                    Instruction::Off => {
                        acc.insert(tup, LightState::Off);
                    }
                };
            });
            acc
        });

    lights.values().fold(0, |count, ls| {
        if ls == &LightState::On {
            count + 1
        } else {
            count
        }
    })
}

fn pt_2(input: &Vec<String>) -> isize {
    let lights: HashMap<(usize, usize), isize> =
        input.iter().fold(HashMap::new(), |mut acc, cur| {
            let (instruction, min_range, max_range) = parse_instructions(cur);
            iterate_over_ranges(min_range, max_range, |tup| {
                let v = acc.get(&tup);
                match instruction {
                    Instruction::Toggle => match v {
                        Some(&d) => acc.insert(tup, d + 2),
                        None => acc.insert(tup, 2),
                    },
                    Instruction::On => match v {
                        Some(&d) => acc.insert(tup, d + 1),
                        None => acc.insert(tup, 1),
                    },
                    Instruction::Off => match v {
                        Some(&d) => acc.insert(tup, cmp::max(0, d - 1)),
                        None => None,
                    },
                };
            });
            acc
        });

    // println!("{:?}", lights);

    lights.values().fold(0, |count, ls| count + ls)
}
