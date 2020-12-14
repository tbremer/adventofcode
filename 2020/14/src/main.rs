use std::collections::HashMap;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let mut computer = Comp::default();

    for inst in file.lines() {
        match parse_line(inst) {
            Instruction::Mask(v) => computer.mask = v,
            Instruction::Memory((loc, value)) => {
                println!("adding `{}` to slot `{}`.", value, loc);
                let temp = apply_mask(&computer.mask, to_padded(value));
                let new_value = to_usize(&temp);


                computer.memory.insert(loc, new_value);
            }
        }
    }

    println!("computer: {:?}", computer);
    println!("pt 1: {:?}", computer.coalesce_memory());
}

#[derive(Debug)]
struct Comp {
    mask: String,
    memory: HashMap<String, usize>,
}

impl Comp {
    fn coalesce_memory(&self) -> usize {
        self.memory.values().fold(0, |acc, cur| acc + cur)
    }
}

impl Default for Comp {
    fn default()  -> Comp {
        Comp {
            mask: String::new(),
            memory: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Memory((String, usize)),
}

fn parse_line(line: &str) -> Instruction {
    if line.starts_with("mask") {
        return Instruction::Mask(line[7..].to_string())
    }

    if line.starts_with("mem[") {
        let t = line.replace("mem[","");
        let mut split = t.split("]");
        let loc = split.next().unwrap();
        let rest = split.next().unwrap().replace(" = ", "");
        
        return Instruction::Memory((String::from(loc), rest.parse::<usize>().unwrap()))
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
            'X' => new_value.push_str(value.get(idx..idx+1).unwrap()),
            '1' => new_value.push('1'),
            '0' => new_value.push('0'),
            _ => panic!("Encountered unknown char in mask {}", v),
        }
    }

    new_value
}