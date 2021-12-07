use std::collections::HashMap;

fn main() {
    println!("Welcome to day-06 — Running test suite first!");
    test_1();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", run(&input_data, 80));
    println!("Puzzle 2: {:?}", run(&input_data, 256));
}

fn mem() -> HashMap<u8, usize> {
    let mut m = HashMap::new();

    for i in 0..=8 {
        m.insert(i, 0);
    }

    m
}

fn run(input: &str, days: u16) -> usize {
    let mut lanternfish: HashMap<u8, usize> =
        input
            .split(",")
            .map(|i| i.parse().unwrap())
            .fold(mem(), |mut map, cur| {
                let f = map.get(&cur).unwrap().clone();

                map.insert(cur, f + 1);

                map
            });

    for _ in 0..days {
        let mut swap = mem();
        for key in 0..=8 {
            let val = lanternfish.get(&key).unwrap();
            if key == 0 {
                swap.insert(8, *val);
                swap.insert(6, *val);
                continue;
            }

            if key == 7 {
                let s_val = swap.get(&6).unwrap().clone();
                swap.insert(6, val + s_val);
                continue;
            }

            let next_key = key - 1;
            swap.insert(next_key, *val);
        }

        lanternfish = swap;
    }

    lanternfish
        .values()
        .fold(0, |a, c| if c == &0 { a } else { a + c })
}

fn test_1() {
    assert_eq!(run("3,4,3,1,2", 18), 26);
    assert_eq!(run("3,4,3,1,2", 80), 5934);
    println!("Suite 1 passes");
}
//fn test_2() {
//    println!("Suite 2 passes");
//}
