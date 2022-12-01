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
    let mut top_3 = vec![];

    let max1 = max(&v);
    let idx1 = &v.iter().position(|i| i == &max1).unwrap();
    v.remove(*idx1);
    top_3.push(max1);

    let max2 = max(&v);
    let idx2 = &v.iter().position(|i| i == &max2).unwrap();
    v.remove(*idx2);
    top_3.push(max2);

    let max3 = max(&v);
    let idx3 = &v.iter().position(|i| i == &max3).unwrap();
    v.remove(*idx3);
    top_3.push(max3);

    top_3.iter().sum()
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

fn max(v: &Vec<i32>) -> i32 {
    v.iter().max().unwrap().to_owned()
}
