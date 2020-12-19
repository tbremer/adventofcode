fn main() {
    let file = utils::read_file(utils::args().remove(0));

    let v = file
        .lines()
        .fold(0, |acc, cur| acc + parse_line(cur.to_string()));

    println!("v: {}", v);

    // for ch in file.chars() {
    //     match ch {
    //         '(' => println!("handle group"),
    //         ')' => println!("rparen, should never see this"),
    //         '0'..='9' => println!("a number!"),
    //         '+' => println!("addition"),
    //         '*' => println!("multiplication"),
    //         ' ' | '\n' | '\r' | '\t' => (),
    //         _ => panic!("Unkown character: {}", ch),
    //     }
    // }
}
#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn parse_line(line: String) -> usize {
    println!("parsing: {}", line);
    let chars: Vec<char> = line.replace(' ', "").chars().collect();
    let len = chars.len();
    let mut idx = 0;
    let mut value: Option<usize> = None;
    let mut operation: Option<Operation> = None;

    while idx < len {
        let item = chars[idx];

        if item == '(' {
            let start = idx + 1;
            let mut end = start;
            let mut paren_count = 1;

            loop {
                if paren_count == 0 {
                    break;
                }
                match chars[end] {
                    '(' => {
                        paren_count += 1;
                        end += 1;
                    }
                    ')' => {
                        paren_count -= 1;
                        end += 1;
                    }
                    _ => end += 1,
                }
            }

            let range = std::ops::Range {
                start,
                end: end - 1,
            };
            let t_string: String = chars[range].to_vec().iter().collect();
            let v = parse_line(t_string);

            match value {
                None => {
                    value = Some(v);
                    idx = end;
                    continue;
                }
                Some(int) => match operation {
                    None => panic!("I am about to exit a paren with no operation"),
                    Some(Operation::Add) => {
                        value = Some(int + v);
                        idx = end;
                        continue;
                    }
                    Some(Operation::Multiply) => {
                        value = Some(int * v);
                        idx = end;
                        continue;
                    }
                },
            }
        }

        if item == '+' {
            operation = Some(Operation::Add);
            idx += 1;
            continue;
        }

        if item == '*' {
            operation = Some(Operation::Multiply);
            idx += 1;
            continue;
        }

        if item.is_digit(10) {
            if value == None {
                value = match item.to_digit(10) {
                    None => panic!("Could not parse {} into digit!", item),
                    Some(v) => Some(v as usize),
                };
                idx += 1;
                continue;
            } else {
                match operation {
                    None => panic!("Need to operate on a value, but operation is `None`"),
                    Some(Operation::Add) => match item.to_digit(10) {
                        None => panic!("Could not parse {} into digit while trying to add!", item),
                        Some(d) => {
                            value = Some(value.unwrap() + (d as usize));
                        }
                    },
                    Some(Operation::Multiply) => match item.to_digit(10) {
                        None => panic!("Could not parse {} into digit while trying to add!", item),
                        Some(d) => {
                            value = Some(value.unwrap() * (d as usize));
                        }
                    },
                };
                idx += 1;
                continue;
            }
        }

        idx += 1;
    }

    match value {
        None => panic!("got to end with no value!"),
        Some(v) => v,
    }
}
