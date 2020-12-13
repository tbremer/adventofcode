use utils;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let input: Vec<&str> = file.lines().collect();

    let mut boat = BoatyMcBoatFace::default();

    for line in input.clone() {
        let instruction = &line[0..1];
        let arg = &line[1..].parse::<isize>().unwrap();

        match instruction {
            "F" => match &boat.facing {
                &Cardinal::North => boat.north += arg,
                &Cardinal::South => boat.north -= arg,
                &Cardinal::East => boat.east += arg,
                &Cardinal::West => boat.east -= arg,
            },
            "N" => boat.north += arg,
            "S" => boat.north -= arg,
            "E" => boat.east += arg,
            "W" => boat.east -= arg,
            "R" => boat.facing = Cardinal::derive(&(Cardinal::resolve(boat.facing) + arg)),
            "L" => boat.facing = Cardinal::derive(&(Cardinal::resolve(boat.facing) - arg)),
            v => panic!("Unknown rule: `{}`", v),
        }
    }

    println!("pt 1: {}", boat.north.abs() + boat.east.abs());
    boat = BoatyMcBoatFace::default();

    for line in input {
        let instruction = &line[0..1];
        let arg = &line[1..].parse::<isize>().unwrap();

        match instruction {
            // move boat to waypoint arg times. Waypoint keeps it's distance
            "F" => {
                boat.east += arg * boat.waypoint.0;
                boat.north += arg * boat.waypoint.1;
            }
            "N" => boat.waypoint.1 += arg,
            "S" => boat.waypoint.1 -= arg,
            "E" => boat.waypoint.0 += arg,
            "W" => boat.waypoint.0 -= arg,
            // rotation around boat
            // https://www.youtube.com/watch?v=M7zJCMpPLKA
            "R" | "L" => {
                let normalized = if instruction == "L" {
                    if arg == &90 {
                        &270isize
                    } else if arg == &180 {
                        &180isize
                    } else if arg == &270 {
                        &90isize
                    } else {
                        panic!("Unknown rotation `{}`", arg);
                    }
                } else {
                    arg
                };
                match normalized {
                    90 => {
                        let (x, y) = rotate_90_cw((boat.waypoint.0, boat.waypoint.1));

                        boat.waypoint.0 = x;
                        boat.waypoint.1 = y;
                    }
                    180 => {
                        boat.waypoint.0 *= -1;
                        boat.waypoint.1 *= -1;
                    }
                    270 => {
                        let (x, y) = rotate_90_ccw((boat.waypoint.0, boat.waypoint.1));

                        boat.waypoint.0 = x;
                        boat.waypoint.1 = y;
                    }
                    _ => panic!("Unknown R rotation: {}", arg),
                }
            }
            v => panic!("Unknown rule: `{}`", v),
        }
    }

    println!("pt 2: {}", boat.north.abs() + boat.east.abs());
}

fn rotate_90_cw(coords: (isize, isize)) -> (isize, isize) {
    let new_east = 0 * coords.0 + 1 * coords.1;
    let new_north = -1 * coords.0 + 0 * coords.1;

    (new_east, new_north)
}

fn rotate_90_ccw(coords: (isize, isize)) -> (isize, isize) {
    let new_east = 0 * coords.0 + -1 * coords.1;
    let new_north = 1 * coords.0 + 0 * coords.1;

    (new_east, new_north)
}

#[derive(Copy, Clone, Debug)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn derive(degrees: &isize) -> Cardinal {
        match degrees % 360 {
            0 => Cardinal::North,
            -270 | 90 => Cardinal::East,
            -180 | 180 => Cardinal::South,
            -90 | 270 => Cardinal::West,
            v => panic!("Unkown Cardinal degrees: `{}`", v),
        }
    }

    #[allow(dead_code)]
    pub fn resolve(dir: Cardinal) -> isize {
        match dir {
            Cardinal::North => 0,
            Cardinal::East => 90,
            Cardinal::South => 180,
            Cardinal::West => 270,
        }
    }
}

#[derive(Debug)]
struct BoatyMcBoatFace {
    north: isize,
    east: isize,
    facing: Cardinal,
    waypoint: (isize, isize),
}

impl Default for BoatyMcBoatFace {
    fn default() -> BoatyMcBoatFace {
        BoatyMcBoatFace {
            north: 0isize,
            east: 0isize,
            facing: Cardinal::East,
            waypoint: (10, 1),
        }
    }
}
