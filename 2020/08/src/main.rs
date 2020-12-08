use std::collections::HashSet;
use utils;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let code_steps: Vec<&str> = file.split("\n").collect();

    // println!("file: {}", file);
    println!("pt 1: {:?}", run_code(&code_steps));
    println!("pt 2: {:?}", run_until_completion(&code_steps));
}

#[derive(Debug)]
struct Execution {
    acc: isize,
    index: usize,
}

fn run_code(code: &Vec<&str>) -> Execution {
    let len = code.len() as isize;
    let mut visited_lines: HashSet<String> = HashSet::new();
    let mut idx: isize = 0;
    let mut count = 0;

    loop {
        if idx == len {
            return Execution {
                acc: count,
                index: idx as usize,
            };
        }

        let line = code[idx as usize];
        let set_key = format!("{}|{}", &idx, &line);

        if visited_lines.contains(&set_key) {
            // println!("idx: {}/line: {}", idx, line);
            return Execution {
                index: idx as usize,
                acc: count,
            };
        }
        visited_lines.insert(set_key);

        match &line[0..3] {
            // noop, increment index and move on.
            "nop" => {
                idx += 1;
                continue;
            }
            // accumulator, increment index,
            "acc" => {
                idx += 1;
                count += line[4..].parse::<isize>().unwrap();
                continue;
            }

            // jump, increment index by argument
            "jmp" => {
                let jump = line[4..].parse::<isize>().unwrap();

                idx = idx + jump;
                continue;
            }
            path => panic!("Unknown code path: `{}`", path),
        }
    }
}

fn run_until_completion(code: &Vec<&str>) -> Execution {
    let mut idx = 0;

    loop {
        let mut code_steps = code.clone();
        let line = match code_steps.get(idx) {
            None => panic!("trying to fetch line that is out of bounds"),
            Some(v) => v,
        };

        let replacement_line = match &line[0..3] {
            "jmp" => format!("{} {}", "nop", &line[4..]),
            "nop" => format!("{} {}", "jmp", &line[4..]),
            _ => {
                idx += 1;
                continue;
            }
        };

        code_steps[idx] = &replacement_line;

        let output = run_code(&code_steps);

        if output.index == code.len() {
            return output;
        }

        idx += 1
    }
}
