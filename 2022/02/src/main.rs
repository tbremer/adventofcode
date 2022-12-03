use std::vec::IntoIter;
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

impl Throws {
    pub fn from_string(input: &str) -> Self {
        match input {
            "A" | "X" => Throws::Rock,
            "B" | "Y" => Throws::Paper,
            "C" | "Z" => Throws::Scissors,
            th => panic!("Unknown input throw: {}", th),
        }
    }

    fn play_score(&self) -> i32 {
        match self {
            Throws::Rock => 1,
            Throws::Paper => 2,
            Throws::Scissors => 3,
        }
    }

    fn oppose(&self, opponent: &Self) -> i32 {
        if self == opponent {
            3
        } else if self == &Throws::Rock && opponent == &Throws::Scissors
            || self == &Throws::Scissors && opponent == &Throws::Paper
            || self == &Throws::Paper && opponent == &Throws::Rock
        {
            6
        } else {
            0
        }
    }

    fn predict(&self, hand: &str) -> Self {
        match hand {
            // Draw
            "Y" => self.clone(),
            // Lose
            "X" => match self {
                Throws::Rock => Throws::Scissors,
                Throws::Paper => Throws::Rock,
                Throws::Scissors => Throws::Paper,
            },
            // win
            "Z" => match self {
                Throws::Rock => Throws::Paper,
                Throws::Paper => Throws::Scissors,
                Throws::Scissors => Throws::Rock,
            },
            c => panic!("Unknown char: {}", c),
        }
    }
}

fn pt_1(input: &str) -> i32 {
    let pairs = parse_input(input.clone());

    pairs.fold(0, |acc, cur| {
        let t = Throws::from_string(cur.get(0).unwrap());
        let m = Throws::from_string(cur.get(1).unwrap());

        acc + m.play_score() + m.oppose(&t)
    })
}

fn pt_2(input: &str) -> i32 {
    let pairs = parse_input(input.clone());

    pairs.fold(0, |acc, cur| {
        let t = Throws::from_string(cur.get(0).unwrap());
        let m = t.predict(cur.get(1).unwrap());

        acc + m.play_score() + m.oppose(&t)
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

fn parse_input(i: &str) -> IntoIter<Vec<String>> {
    i.lines()
        .flat_map(|line| {
            let v: Vec<String> = line.split(" ").map(|i| i.to_string()).collect();
            let chunks = v.chunks(2).map(|i| i.to_owned());

            chunks.collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>()
        .into_iter()
}
