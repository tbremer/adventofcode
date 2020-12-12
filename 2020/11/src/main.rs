use std::{collections::HashMap, ops::Range};
use utils;

#[derive(Copy, Clone, Debug, PartialEq)]
enum FloorPlan {
    SeatEmpty,
    SeatTaken,
    Floor,
}

fn offset_coords() -> HashMap<String, (isize, isize)> {
    let tuples = vec![
        ("tl".to_string(), (-1, -1)),
        ("t".to_string(), (0, -1)),
        ("tr".to_string(), (1, -1)),
        ("r".to_string(), (1, 0)),
        ("br".to_string(), (1, 1)),
        ("b".to_string(), (0, 1)),
        ("bl".to_string(), (-1, 1)),
        ("l".to_string(), (-1, 0)),
    ];

    tuples
        .into_iter()
        .collect::<HashMap<String, (isize, isize)>>()
}

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let input: Vec<Vec<FloorPlan>> = file.lines().map(|line| line_to_floor_vec(line)).collect();
    let mut pt1_map = input.clone();
    let mut pt2_map = input.clone();

    loop {
        // std::thread::sleep(std::time::Duration::from_millis(42));
        // print!("\x1B[2J\x1B[1;1H");
        // std::thread::sleep(std::time::Duration::from_millis(84));
        // print!("{}", print_map(&pt1_map));
        let (differences, new_map) = cycle_pt1(pt1_map.clone());

        if differences == 0 {
            break;
        }

        pt1_map = new_map;
    }

    loop {
        // if iterations > 3 {
        //     panic!();
        // }
        std::thread::sleep(std::time::Duration::from_millis(42));
        print!("\x1B[2J\x1B[1;1H");
        print!("^[]1337;ClearScrollback^G");
        std::thread::sleep(std::time::Duration::from_millis(16));
        print!("{}", print_map(&pt2_map));

        let (differences, new_map) = cycle_pt2(pt2_map.clone());

        if differences == 0 {
            break;
        }

        pt2_map = new_map;
    }
    // println!("pt 1 (occupied tiles): {}", count_occupied_tiles(&pt1_map));
    println!("pt 2 (occupied tiles): {}", count_occupied_tiles(&pt2_map));
}

fn cycle_pt1(plan: Vec<Vec<FloorPlan>>) -> (usize, Vec<Vec<FloorPlan>>) {
    let mut differences = 0;
    let rl = plan.len();
    let mut new_map = vec![];
    let rows = Range {
        start: 0,
        end: rl.clone(),
    };

    for r in rows {
        let row = plan.get(r).unwrap();
        let ri = r as isize;
        let columns = Range {
            start: 0,
            end: row.len(), // - 1,
        };

        new_map.push(vec![]);

        for c in columns {
            let cur_tile = row.get(c).unwrap();
            let ci = c as isize;

            if cur_tile == &FloorPlan::Floor {
                new_map[r].push(FloorPlan::Floor);
                continue;
            }

            let adjacent_count = count_adjacent_occupied(&plan, (ci, ri));

            let mapped = if cur_tile == &FloorPlan::SeatEmpty && adjacent_count == 0 {
                FloorPlan::SeatTaken
            } else if cur_tile == &FloorPlan::SeatTaken && adjacent_count >= 4 {
                FloorPlan::SeatEmpty
            } else {
                cur_tile.clone()
            };

            if &mapped != cur_tile {
                differences += 1;
            }

            new_map[r].push(mapped);
        }
    }

    (differences, new_map)
}

fn cycle_pt2(plan: Vec<Vec<FloorPlan>>) -> (usize, Vec<Vec<FloorPlan>>) {
    let mut differences = 0;
    let rl = plan.len();
    let mut new_map = vec![];
    let rows = Range {
        start: 0,
        end: rl.clone(),
    };

    for r in rows {
        let row = plan.get(r).unwrap();
        let ri = r as isize;
        let columns = Range {
            start: 0,
            end: row.len(),
        };

        new_map.push(vec![]);

        for c in columns {
            let cur_tile = row.get(c).unwrap();
            let ci = c as isize;

            if cur_tile == &FloorPlan::Floor {
                new_map[r].push(FloorPlan::Floor);
                continue;
            }

            let occupied = count_occupied_non_adjacent(&plan, (ci, ri));

            let mapped = if cur_tile == &FloorPlan::SeatEmpty && occupied == 0 {
                FloorPlan::SeatTaken
            } else if cur_tile == &FloorPlan::SeatTaken && occupied > 4 {
                FloorPlan::SeatEmpty
            } else {
                cur_tile.clone()
            };

            if &mapped != cur_tile {
                differences += 1;
            }

            new_map[r].push(mapped);
        }
    }

    (differences, new_map)
}

fn count_adjacent_occupied(grid: &Vec<Vec<FloorPlan>>, coords: (isize, isize)) -> usize {
    let mut adjacent_occupied = 0;
    for offset in offset_coords() {
        let (x_off, y_off) = offset.1;
        let x = coords.0 + x_off;
        let y = coords.1 + y_off;

        match grid.get(y as usize) {
            None => continue,
            Some(row) => match row.get(x as usize) {
                None => continue,
                Some(t) => match t {
                    FloorPlan::SeatTaken => adjacent_occupied += 1,
                    _ => continue,
                },
            },
        }
    }

    adjacent_occupied
}

fn count_occupied_non_adjacent(grid: &Vec<Vec<FloorPlan>>, coords: (isize, isize)) -> usize {
    let mut occupied = 0;
    let offsets: Vec<(String, (isize, isize))> = offset_coords().into_iter().collect();
    let mut idx = 0;
    let mut multiplier = 1;

    while idx < offsets.len() {
        let (x_off, y_off) = offsets[idx].1;
        let x = coords.0 + x_off * multiplier;
        let y = coords.1 + y_off * multiplier;

        match grid.get(y as usize) {
            None => {
                idx += 1;
                multiplier = 1;
                continue;
            }
            Some(row) => match row.get(x as usize) {
                None => {
                    idx += 1;
                    multiplier = 1;
                    continue;
                }
                Some(t) => match t {
                    FloorPlan::Floor => {
                        multiplier += 1;
                        continue;
                    }
                    FloorPlan::SeatTaken => {
                        multiplier = 1;
                        occupied += 1;
                        idx += 1;
                        continue;
                    }
                    FloorPlan::SeatEmpty => {
                        multiplier = 1;
                        idx += 1;
                        continue;
                    }
                },
            },
        }
    }

    occupied
}

fn count_occupied_tiles(grid: &Vec<Vec<FloorPlan>>) -> usize {
    let mut count = 0;
    let r_rng = Range {
        start: 0,
        end: grid.len(),
    };

    for r_idx in r_rng {
        let c = &grid[r_idx];
        let c_rng = Range {
            start: 0,
            end: c.len(),
        };
        for c_idx in c_rng {
            match grid[r_idx][c_idx] {
                FloorPlan::SeatTaken => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn line_to_floor_vec(l: &str) -> Vec<FloorPlan> {
    l.chars()
        .map(|c| char_to_floor(c))
        .collect::<Vec<FloorPlan>>()
}

fn char_to_floor(c: char) -> FloorPlan {
    match c {
        'L' => FloorPlan::SeatEmpty,
        '#' => FloorPlan::SeatTaken,
        '.' => FloorPlan::Floor,
        _ => panic!("Unknown floor type: {}", c),
    }
}

fn print_map(map: &Vec<Vec<FloorPlan>>) -> String {
    map.iter().fold(String::new(), |str, cur| {
        format!(
            "{}\n{}",
            str,
            cur.iter().fold(String::new(), |str, fl| {
                match fl {
                    FloorPlan::Floor => format!("{}.", str),
                    FloorPlan::SeatEmpty => format!("{}L", str),
                    FloorPlan::SeatTaken => format!("{}#", str),
                }
            })
        )
    })
}
