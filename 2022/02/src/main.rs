fn main() {
    println!("Welcome to day-02 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

#[derive(Clone, Debug, PartialEq)]
enum Throws {
    Rock,
    Paper,
    Scissors,
}

fn pt_1(input: &str) -> i32 {
    input.split("\n").fold(0, |acc, round| {
        let mut total = 0;
        let mut c = round.chars();

        loop {
            let them = match c.nth(0) {
                Some(t) => parse_throw(t),
                None => break,
            };
            let me = match c.nth(1) {
                Some(m) => parse_throw(m),
                None => panic!("Broke, but got them: {:?}", them),
            };

            c.next();

            total += play_score(&me);

            if them == me {
                total += 3
            }
            // i win
            else if me == Throws::Rock && them == Throws::Scissors
                || me == Throws::Scissors && them == Throws::Paper
                || me == Throws::Paper && them == Throws::Rock
            {
                total += 6;
            }
        }

        acc + total
    })
}

fn pt_2(input: &str) -> i32 {
    input.split("\n").fold(0, |acc, round| {
        let mut total = 0;
        let mut c = round.chars();

        loop {
            let them = match c.nth(0) {
                Some(t) => parse_throw(t),
                None => break,
            };
            let me = match c.nth(1) {
                Some(m) => parse_known_throw(&them, m), // get_throw(m),
                None => panic!("Broke, but got them: {:?}", them),
            };

            c.next();

            total += play_score(&me);

            if them == me {
                total += 3
            }
            // i win
            else if me == Throws::Rock && them == Throws::Scissors
                || me == Throws::Scissors && them == Throws::Paper
                || me == Throws::Paper && them == Throws::Rock
            {
                total += 6;
            }
        }

        acc + total
    })
}

fn test_1() {
    assert_eq!(
        pt_1(
            "A Y
B X
C Z"
        ),
        15
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "A Y
B X
C Z"
        ),
        12
    );
    println!("Suite 2 passes");
}

fn parse_throw(t: char) -> Throws {
    match t {
        'A' | 'X' => Throws::Rock,
        'B' | 'Y' => Throws::Paper,
        'C' | 'Z' => Throws::Scissors,
        th => panic!("Unknown throw: {}", th),
    }
}

fn parse_known_throw(them: &Throws, m: char) -> Throws {
    match m {
        // Draw
        'Y' => them.clone(),
        // Lose
        'X' => match them {
            Throws::Rock => Throws::Scissors,
            Throws::Paper => Throws::Rock,
            Throws::Scissors => Throws::Paper,
        },
        // win
        'Z' => match them {
            Throws::Rock => Throws::Paper,
            Throws::Paper => Throws::Scissors,
            Throws::Scissors => Throws::Rock,
        },
        c => panic!("Unknown char: {}", c),
    }
}

fn play_score(throw: &Throws) -> i32 {
    match throw {
        Throws::Rock => 1,
        Throws::Paper => 2,
        Throws::Scissors => 3,
    }
}
