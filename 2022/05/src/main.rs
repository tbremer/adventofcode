use std::collections::HashMap;

fn main() {
    println!("Welcome to day-05 — Running test suite first!");
    test_1();
    // test_2();

    println!("Test suite complete, solving puzzle…");

    // let input_data = utils::read_file(utils::args().remove(0));

    // println!("Puzzle 1: {:?}", pt_1(&input_data));
    // println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> String {
    let input = parse_input(input);

    println!("{:?}", input);

    "".to_string()
}
//fn pt_2(_input: &str) -> () {}

fn test_1() {
    assert_eq!(
        pt_1(
            "[D]    
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

impl Moves {
    pub fn new() -> Self {
        Moves {
            count: usize::MAX,
            from: usize::MAX,
            to: usize::MAX,
        }
    }
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
    let temp_diagram = complete_diagram[..complete_diagram.len() - 2].to_vec();

    let m = parse_movements(temp_movements);

    println!("m: {:?}", m);

    Input {
        diagram: parse_diagram(temp_diagram),
        movements: m,
    }
}

fn parse_diagram(input: Vec<&str>) -> HashMap<usize, Vec<String>> {
    let mut diagram: HashMap<usize, Vec<String>> = HashMap::new();

    for (idx, col) in input.iter().enumerate() {
        let col_sp = col.split("] [");
        let mut v = vec![];
        col_sp.for_each(|sp| {
            let clean = sp.trim().replace("[", "").replace("]", "");

            v.push(clean.to_owned().to_string());
        });

        diagram.insert(idx + 1, v);
    }

    diagram
}

fn parse_movements(input: Vec<&str>) -> Vec<Moves> {
    input
        .iter()
        .map(|m| {
            let foo: Vec<&str> = m.split(" ").collect();
            println!("foo {:?}", foo);
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
