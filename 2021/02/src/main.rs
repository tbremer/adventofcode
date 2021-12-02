fn main() {
    println!("Welcome to day-02 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

enum Direction {
    Up(usize),
    Down(usize),
    Forward(usize),
}

fn str_to_directions(str: &str) -> Vec<Direction> {
    str.lines()
        .map(|line| {
            let mut sp = line.split_whitespace();
            let dir = sp.nth(0).unwrap();
            let num: usize = sp.nth(0).unwrap().parse().unwrap();

            if dir == "up" {
                Direction::Up(num)
            } else if dir == "down" {
                Direction::Down(num)
            } else if dir == "forward" {
                Direction::Forward(num)
            } else {
                panic!("Unknown direction: {}", dir);
            }
        })
        .collect()
}

fn pt_1(input: &str) -> usize {
    let (h, v) = str_to_directions(input)
        .iter()
        .fold((0, 0), |acc, cur| match cur {
            Direction::Up(d) => (acc.0, acc.1 - d),
            Direction::Down(d) => (acc.0, acc.1 + d),
            Direction::Forward(d) => (acc.0 + d, acc.1),
        });
    h * v
}

fn pt_2(input: &str) -> usize {
    let (hor, depth, _) = str_to_directions(input)
        .iter()
        .fold((0, 0, 0), |acc, cur| match cur {
            Direction::Up(d) => (acc.0, acc.1, acc.2 - d),
            Direction::Down(d) => (acc.0, acc.1, acc.2 + d),
            Direction::Forward(d) => (acc.0 + d, acc.1 + d * acc.2, acc.2),
        });

    hor * depth
}

fn test_1() {
    assert_eq!(
        pt_1(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        ),
        150
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        ),
        900
    );
    println!("Suite 2 passes");
}
