use std::collections::HashMap;
fn main() {
    println!("Welcome to day-04 — Running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::read_file(utils::args().remove(0));

    println!("Puzzle 1: {:?}", pt_1(&input_data));
    println!("Puzzle 2: {:?}", pt_2(&input_data));
}

type Board = Vec<Vec<Mark>>;
type Boards = HashMap<usize, Board>;

#[derive(Debug, Clone)]
enum Mark {
    Open(usize),
    Closed(usize),
}

fn pt_1(input: &str) -> usize {
    let mut monkey = input.split("\n\n").into_iter();
    let draws = monkey
        .nth(0)
        .unwrap()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut map = build_boards(monkey.collect::<Vec<&str>>());

    for draw in draws {
        for m_idx in 0..map.len() {
            let mut b = map.remove(&m_idx).unwrap();
            let b_len = b.len();

            for y in 0..b_len {
                let row = b.get(y).unwrap().clone();

                for x in 0..row.len() {
                    let item = row.get(x).unwrap();
                    if let Mark::Open(v) = item {
                        if v == &draw {
                            b[y][x] = Mark::Closed(*v);
                        }
                    }
                }
            }

            if check_board(&b) {
                return solve(draw, b);
            }

            map.insert(m_idx, b);
        }
    }

    0
}
fn pt_2(input: &str) -> usize {
    let mut monkey = input.split("\n\n").into_iter();
    let draws = monkey
        .nth(0)
        .unwrap()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut map = build_boards(monkey.collect::<Vec<&str>>());

    for draw in draws {
        let c = map.clone();
        let keys = c.keys();
        for m_idx in keys {
            let map_len = map.len();
            let mut b = map.remove(&m_idx).unwrap();
            let b_len = b.len();

            for y in 0..b_len {
                let row = b.get(y).unwrap().clone();

                for x in 0..row.len() {
                    let item = row.get(x).unwrap();
                    if let Mark::Open(v) = item {
                        if v == &draw {
                            b[y][x] = Mark::Closed(*v);
                        }
                    }
                }
            }

            if check_board(&b) {
                if map_len == 1 {
                    return solve(draw, b);
                } else {
                    map.remove(m_idx);
                }
            } else {
                map.insert(*m_idx, b);
            }
        }
    }

    0
}

fn test_1() {
    assert_eq!(
        pt_1(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        4512
    );
    println!("Suite 1 passes");
}
fn test_2() {
    assert_eq!(
        pt_2(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        1924
    );
    println!("Suite 2 passes");
}

fn build_boards(boards: Vec<&str>) -> Boards {
    let mut idx = 0;
    boards.iter().fold(HashMap::new(), |mut acc, cur| {
        let b = cur
            .lines()
            .map(|l| {
                l.trim()
                    .split_whitespace()
                    .map(|num| Mark::Open(num.parse::<usize>().unwrap()))
                    .collect()
            })
            .collect();

        acc.insert(idx, b);

        idx += 1;

        acc
    })
}

fn check_board(b: &Board) -> bool {
    let y_len = b.len();

    for y in 0..y_len {
        let r = &b[y];
        let len = r.len();
        // row marked closed
        if r.iter().all(|space| match space {
            Mark::Closed(_) => true,
            Mark::Open(_) => false,
        }) {
            return true;
        }

        for x in 0..len {
            let mut v = vec![];
            // column marked true
            for y in 0..y_len {
                v.push(b[y][x].clone())
            }

            if v.iter().all(|space| match space {
                Mark::Closed(_) => true,
                Mark::Open(_) => false,
            }) {
                return true;
            }
        }
    }
    false
}

fn solve(draw: usize, board: Board) -> usize {
    draw * board.iter().flatten().fold(0, |mut acc, cur| {
        if let Mark::Open(x) = cur {
            acc += x
        }

        acc
    })
}
