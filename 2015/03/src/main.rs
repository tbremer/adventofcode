use std::collections::HashMap;

type Coords = (i8, i8);

fn main() {
    println!("Welcome to day-3 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(path: &str) -> usize {
    // (x, y)
    let mut coords: Coords = (0, 0);
    let mut dict: HashMap<Coords, usize> = HashMap::new();
    dict.insert(coords, 1);

    for p in path.chars() {
        match p {
            '>' => coords.0 += 1,
            '<' => coords.0 -= 1,
            '^' => coords.1 += 1,
            'v' => coords.1 -= 1,
            _ => panic!("Unknown char: {}", p),
        }

        if dict.contains_key(&coords) {
            let v = &dict.get(&coords).unwrap().to_owned();
            dict.insert(coords, v + 1);
        } else {
            dict.insert(coords, 1);
        }
    }

    dict.len()
}

fn is_even(int: usize) -> bool {
    int % 2 == 0
}

fn pt_2(path: &str) -> usize {
    let p = path.chars().collect::<Vec<_>>();
    let mut s_coords = (0, 0);
    let mut rs_coords = (0, 0);
    let mut dict: HashMap<Coords, usize> = HashMap::new();
    dict.insert(s_coords, 2);

    for idx in 0..path.len() {
        let mut coords = if is_even(idx) {
            &mut s_coords
        } else {
            &mut rs_coords
        };

        match p[idx] {
            '>' => coords.0 += 1,
            '<' => coords.0 -= 1,
            '^' => coords.1 += 1,
            'v' => coords.1 -= 1,
            i => panic!("Unknown char: {}", i),
        }

        if dict.contains_key(&coords) {
            let v = &dict.get(&coords).unwrap().to_owned();
            dict.insert(coords.clone(), v + 1);
        } else {
            dict.insert(coords.clone(), 1);
        }
    }
    dict.len()
}

fn test_1() {
    assert_eq!(pt_1(">"), 2);
    assert_eq!(pt_1("^>v<"), 4);
    assert_eq!(pt_1("^v^v^v^v^v"), 2);
    println!("Suite 1 passes");
}
fn test_2() {
    assert_eq!(pt_2("^v"), 3);
    assert_eq!(pt_2("^>v<"), 3);
    assert_eq!(pt_2("^v^v^v^v^v"), 11);
    println!("Suite 2 passes")
}
