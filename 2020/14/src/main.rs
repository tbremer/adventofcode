use std::collections::HashMap;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let mut computer = Comp::default();

    for inst in file.lines() {
        match parse_line(inst) {
            Instruction::Mask(v) => computer.mask = v,
            Instruction::Memory((loc, value)) => {
                let temp = apply_mask(&computer.mask, to_padded(value));
                let new_value = to_usize(&temp);

                computer.memory.insert(loc, new_value);
            }
        }
    }

    // println!("computer: {:?}", computer);
    println!("pt 1: {:?}", computer.coalesce_memory());

    computer = Comp::default();

    for inst in file.lines() {
        match parse_line(inst) {
            Instruction::Mask(v) => computer.mask = v,
            Instruction::Memory((loc, value)) => {
                let mem_slots = decode_memory_location(&computer.mask, loc);

                for slot in mem_slots.iter() {
                    computer.memory.insert(to_usize(slot), value);
                }
            }
        }
    }

    println!("pt 2: {}", computer.coalesce_memory());
}

fn decode_memory_location(mask: &str, loc: usize) -> Vec<String> {
    let permutations = mask.chars().fold(vec![], |all, cur| {
        if all.len() == 0 {
            return if cur == 'X' {
                vec![String::from("0"), String::from("1")]
            } else {
                vec![String::from(cur)]
            };
        }

        let mut permutations = vec![];

        for a in all {
            if cur == 'X' {
                permutations.push(format!("{}0", a));
                permutations.push(format!("{}1", a));
            } else {
                permutations.push(format!("{}{}", a, cur));
            }
        }

        permutations
    });

    permutations
        .iter()
        .map(|permutation| apply_floating_mask(&mask, &to_padded(loc), permutation))
        .collect()
}

fn apply_floating_mask(mask: &str, value: &str, floating: &str) -> String {
    let t_value: Vec<char> = value.chars().collect();
    let t_floating: Vec<char> = floating.chars().collect();

    let value = mask
        .char_indices()
        .fold(String::new(), |acc, (c_idx, cur)| match cur {
            '1' => format!("{}{}", acc, '1'),
            '0' => format!("{}{}", acc, t_value[c_idx]),
            'X' => format!("{}{}", acc, t_floating[c_idx]),
            v => panic!("Unknown character: {}", v),
        });

    value
}

#[derive(Debug)]
struct Comp {
    mask: String,
    memory: HashMap<usize, usize>,
}

impl Comp {
    fn coalesce_memory(&self) -> usize {
        self.memory.values().fold(0, |acc, cur| acc + cur)
    }
}

impl Default for Comp {
    fn default() -> Comp {
        Comp {
            mask: String::new(),
            memory: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Memory((usize, usize)),
}

fn parse_line(line: &str) -> Instruction {
    if line.starts_with("mask") {
        return Instruction::Mask(line[7..].to_string());
    }

    if line.starts_with("mem[") {
        let t = line.replace("mem[", "");
        let mut split = t.split("]");
        let loc = split.next().unwrap();
        let rest = split.next().unwrap().replace(" = ", "");

        return Instruction::Memory((
            loc.parse::<usize>().unwrap(),
            rest.parse::<usize>().unwrap(),
        ));
    }

    unreachable!();
}

fn to_padded(num: usize) -> String {
    format!("{:0>36b}", num)
}

fn to_usize(v: &str) -> usize {
    usize::from_str_radix(v, 2).unwrap()
}

fn apply_mask(mask: &str, value: String) -> String {
    let mut new_value = String::with_capacity(36);

    for (idx, v) in mask.char_indices() {
        match v {
            'X' => new_value.push_str(value.get(idx..idx + 1).unwrap()),
            '1' => new_value.push('1'),
            '0' => new_value.push('0'),
            _ => panic!("Encountered unknown char in mask {}", v),
        }
    }

    new_value
}
