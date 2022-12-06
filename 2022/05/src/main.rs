use std::collections::HashMap;

fn main() {
    println!("Welcome to day-05 — Running test suite first!");
    test_1();
    // test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0), Some(false));
    println!("{:?}", input_data);

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    // println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> String {
    let mut iter = 0;
    let mut input = parse_input(input);
    let mut output: HashMap<usize, String> = HashMap::new();

    input.movements.iter().for_each(|mvmnt| {
        let mut from_stack = input.diagram.get(&mvmnt.from).unwrap().clone();
        let mut to_stack = input.diagram.get(&mvmnt.to).unwrap().clone();

        for _ in 0..mvmnt.count {
            let item = from_stack.remove(0);

            to_stack.insert(0, item);
        }

        input.diagram.remove(&mvmnt.to);
        input.diagram.insert(mvmnt.to, to_stack);
        input.diagram.remove(&mvmnt.from);
        input.diagram.insert(mvmnt.from, from_stack);

        iter += 1;
    });

    for key in input.diagram.keys() {
        let stack = input.diagram.get(&key).unwrap();

        match stack.first() {
            None => (),
            Some(l) => {
                output.insert(*key, l.clone());
            }
        }
    }

    let mut k = output.keys().collect::<Vec<&usize>>();

    k.sort();

    k.iter().fold(String::new(), |acc, cur| {
        let letter = output.get(cur).unwrap();

        acc + letter
    })

    // "".to_string()
}
//fn pt_2(_input: &str) -> () {}

fn test_1() {
    assert_eq!(
        pt_1(
            "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        ),
        String::from("CMZ")
    );
    println!("Suite 1 passes");
}
//fn test_2() {
//    println!("Suite 2 passes");
//}

#[derive(Debug)]
struct Moves {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    diagram: HashMap<usize, Vec<String>>,
    movements: Vec<Moves>,
}

fn parse_input(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let complete_diagram: Vec<&str> = sections.next().unwrap().lines().collect();
    let temp_movements: Vec<&str> = sections.next().unwrap().lines().collect();
    let temp_diagram = complete_diagram[..complete_diagram.len() - 1].to_vec();

    Input {
        diagram: parse_diagram(temp_diagram),
        movements: parse_movements(temp_movements),
    }
}

fn parse_diagram(input: Vec<&str>) -> HashMap<usize, Vec<String>> {
    let mut diagram: HashMap<usize, Vec<String>> = HashMap::new();

    for col in input.iter() {
        let mut col_idx = 0;
        let formatted_col = format_col(col);
        let col_sp = formatted_col.split("][");

        col_sp.for_each(|sp| {
            col_idx += 1;
            let clean = sp.trim().replace("[", "").replace("]", "");
            if clean == "-" {
                return;
            }

            match diagram.get(&col_idx) {
                None => diagram.insert(col_idx, vec![clean]),
                Some(v) => {
                    let mut nv = v.clone();
                    nv.push(clean);
                    diagram.insert(col_idx, nv)
                }
            };
        });
    }

    diagram
}

fn format_col(col: &&str) -> String {
    col.replace("    ", "[-]")
        .replace("] [", "][")
        .trim()
        .to_string()
}

fn parse_movements(input: Vec<&str>) -> Vec<Moves> {
    input
        .iter()
        .map(|m| {
            let mut t_m = m
                .split(" ")
                .filter(|ins| ins != &"move" && ins != &"from" && ins != &"to")
                .map(|monkey| monkey.parse::<usize>().unwrap());

            Moves {
                count: t_m.next().unwrap(),
                from: t_m.next().unwrap(),
                to: t_m.next().unwrap(),
            }
        })
        .collect()
}
