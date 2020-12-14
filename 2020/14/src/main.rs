use std::collections::HashMap;

fn main() {
    let file = utils::read_file(utils::args().remove(0));

    println!("{}", file);
    println!("{:?}", parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    println!("{:?}", parse_line("mem[8] = 11"));
}

struct Comp {
    mask: String,
    memory: HashMap<String, String>,
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Memory((u32, u32)),
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


        return Instruction::Memory((loc.parse::<u32>().unwrap(), rest.parse::<u32>().unwrap()))

    }

    unreachable!();
}

fn to_padded(num: u32) -> String {
    format!("{:0>36b}", num)
}