use utils;

fn main() {
    println!("Welcome to day-01 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|cur| {
            cur.split("\n")
                .map(|i| i.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
}
fn pt_2(input: &str) -> i32 {
    let mut v: Vec<i32> = input
        .split("\n\n")
        .map(|cur| cur.split("\n").map(|i| i.parse::<i32>().unwrap()).sum())
        .collect();

    v.sort();

    v[v.len() - 3..].iter().sum()
}

fn test_1() {
    assert_eq!(
        pt_1(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        ),
        24_000
    );

    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
        ),
        45_000
    );
}
