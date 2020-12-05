use utils;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Position {
    upper: f32,
    lower: f32,
    instruction: Vec<Instruction>,
}

impl Position {
    pub fn find_path(&mut self) -> u16 {
        let last_step = self.instruction.clone().pop().unwrap();
        for instruction in &self.instruction {
            match instruction {
                Instruction::Down => {
                    let avg = (self.lower + self.upper) / 2.;
                    self.upper = avg.floor();
                }
                Instruction::Up => {
                    let avg = (self.lower + self.upper) / 2.;
                    self.lower = avg.ceil();
                }
            }
        }

        match last_step {
            Instruction::Up => self.upper as u16,
            Instruction::Down => self.lower as u16,
        }
    }
}

#[derive(Debug)]
struct BoardingPass {
    row: Position,
    seat: Position,
    pass: String,
}

impl BoardingPass {
    pub fn new(pass: String) -> BoardingPass {
        let mut row_ins = pass.clone();
        let seat_ins = row_ins.split_off(7);

        BoardingPass {
            pass,
            row: Position {
                upper: 127.,
                lower: 0.,
                instruction: row_ins
                    .chars()
                    .map(|i| match i {
                        'F' => Instruction::Down,
                        'B' => Instruction::Up,
                        unknown => panic!("Unknown row instruction: {}", unknown),
                    })
                    .collect(),
            },
            seat: Position {
                upper: 7.,
                lower: 0.,
                instruction: seat_ins
                    .chars()
                    .map(|i| match i {
                        'R' => Instruction::Up,
                        'L' => Instruction::Down,
                        unknown => panic!("Unknown seat instruction: {}", unknown),
                    })
                    .collect(),
            },
        }
    }
}

fn main() {
    let input: Vec<String> = utils::read_file(utils::args().remove(0))
        .split_whitespace()
        .map(|i| i.to_owned())
        .collect();
    let mut highest_id = 0;
    let mut all_ids = vec![];
    for raw_pass in input {
        let mut pass = BoardingPass::new(raw_pass);
        let id = pass.row.find_path() * 8 + pass.seat.find_path();

        if id > highest_id {
            highest_id = id;
        }
        all_ids.push(id);
    }

    println!("pt1 highest_id: {:?}", highest_id);

    // pt2
    all_ids.sort();
    let mut iter = all_ids.iter();
    let mut cur_seat = iter.next().unwrap();
    loop {
        let next = match iter.next() {
            Some(v) => v,
            None => panic!("We're out of seats!"),
        };

        if cur_seat + 1 != next.to_owned() {
            break;
        }

        cur_seat = next;
    }
    println!("pt2 my seat: {:?}", cur_seat + 1);
}
