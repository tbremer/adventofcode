use md5;

fn main() {
    println!("Welcome to day-4 — Running test suite first!");
    test_1();

    println!("Test suite complete, solving puzzle…");

    let input_data = "bgvyzdsv";

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(base: &str) -> usize {
    let mut i = 0;

    loop {
        let str = format!("{}{}", base, i);
        let digest = md5::compute(str);
        let (b1, b2, b3) = (digest[0] as u16, digest[1] as u16, digest[2] as u16);
        let shifted = b3 >> 4;

        if (b1 + b2 + shifted) == 0 {
            return i;
        }

        i += 1;
    }
}

fn pt_2(base: &str) -> usize {
    let mut i = 0;

    loop {
        let str = format!("{}{}", base, i);
        let digest = md5::compute(str);
        let (b1, b2, b3) = (digest[0] as u16, digest[1] as u16, digest[2] as u16);

        if (b1 + b2 + b3) == 0 {
            return i;
        }

        i += 1;
    }
}

fn test_1() {
    assert_eq!(pt_1("abcdef"), 609043);
    assert_eq!(pt_1("pqrstuv"), 1048970);
    println!("Suite 1 passes");
}
