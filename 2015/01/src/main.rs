use utils;

fn main() {
    println!("Welcome to day-1, running test suite first!");
    test_01();
    test_02();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Floor: {}", pt_1(&input_data));
    println!("Position when negative: {:?}", pt_2(&input_data))
}

fn pt_1(input: &str) -> i16 {
    let v = input.chars();

    v.fold(0, |acc, i| {
        // println!("i: {} / acc: {}", i, acc);
        match i {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => panic!("Unknown character: {}", i),
        }
    })
}

fn pt_2(input: &str) -> Option<u16> {
    let mut idx = 1;
    let mut floor = 0;
    let v = input.chars().collect::<Vec<_>>();

    for i in v.iter() {
        match i {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unknown character: {}", i),
        };

        if floor < 0 {
            return Some(idx);
        }

        idx += 1;
    }

    None
}

fn test_01() -> () {
    assert_eq!(pt_1("(())"), 0);
    assert_eq!(pt_1("()()"), 0);

    assert_eq!(pt_1("((("), 3);
    assert_eq!(pt_1("(()(()("), 3);
    assert_eq!(pt_1("))((((("), 3);

    assert_eq!(pt_1("())"), -1);
    assert_eq!(pt_1("))("), -1);

    assert_eq!(pt_1(")))"), -3);
    assert_eq!(pt_1(")())())"), -3);
    println!("suite 1 passes passes")
}

fn test_02() {
    assert_eq!(pt_2(")"), Some(1));
    assert_eq!(pt_2("()())"), Some(5));
    println!("suite 2 passes passes")
}
