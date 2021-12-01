fn main() {
    println!("Welcome to day-01 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

fn pt_1(input: &str) -> usize {
    let mut lines = input.lines().map(|i| {i.parse::<usize>().unwrap()});
    let mut prev = lines.next().unwrap();
    let mut larger_count = 0;

    for line in lines {
        let l = line;
        if l > prev {
            larger_count+=1
        }

        prev = l;
    }

    larger_count
}

fn pt_2(input: &str) -> usize{
    let lines: Vec<usize> = input.lines().map(|i| {i.parse::<usize>().unwrap()}).collect();
    let mut prev = lines[0] + lines[1] + lines[2];
    let mut larger_count = 0;

    for (idx, v) in lines.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        if lines.len() - 1 < idx + 2 {
            break;
        }

        let next = v + lines[idx+1] + lines[idx+2];

        if prev < next {
            larger_count += 1;
        }

        prev = next;
    }


    larger_count
}

fn test_1() {
    assert_eq!(pt_1("199
200
208
210
200
207
240
269
260
263"), 7);
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(pt_2("199
200
208
210
200
207
240
269
260
263"), 5);
   println!("Suite 2 passes");
}

