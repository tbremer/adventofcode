use utils;

#[derive(Clone, Debug, PartialEq)]
enum Geology {
    Open,
    Tree,
}

enum GPSError {
    OutOfBounds,
}

type GeologyMap = Vec<Vec<Geology>>;
type Coords = (usize, usize);

#[derive(Debug)]
struct GPS {
    coords: Coords,
    map: GeologyMap,
}

impl GPS {
    pub fn new(geo: GeologyMap) -> GPS {
        GPS {
            coords: (0, 0),
            map: geo,
        }
    }

    pub fn reset_location(&mut self) {
        self.coords = (0, 0);
    }

    pub fn peek_geo(&self) -> &Geology {
        &self.map[self.coords.1][self.coords.0]
    }

    pub fn move_down(&mut self, steps: usize) -> Result<(), GPSError> {
        let len = self.map.len();
        let new_idx = self.coords.1 + steps;

        if new_idx >= len {
            return Err(GPSError::OutOfBounds);
        }

        self.coords.1 = new_idx;

        Ok(())
    }

    pub fn move_right(&mut self, steps: usize) {
        let cur_row = &self.map[self.coords.1];
        let len = cur_row.len();
        let mut new_idx = self.coords.0 + steps;

        if new_idx >= len {
            let temp = new_idx - len;

            new_idx = temp;
        }

        self.coords.0 = new_idx;
    }

    pub fn move_until_espcaped(&mut self, steps: Coords) -> usize {
        let mut collision_count = 0;

        loop {
            self.move_right(steps.0);
            match self.move_down(steps.1) {
                Err(_) => break,
                Ok(_) => (),
            }
            if self.peek_geo() == &Geology::Tree {
                collision_count += 1
            }
        }

        collision_count
    }
}

fn row_to_geology(row: &str) -> Vec<Geology> {
    row.chars()
        .map(|g| match g {
            '.' => Geology::Open,
            '#' => Geology::Tree,
            unknown => panic!("Unknown Geology: {}", unknown),
        })
        .collect()
}

fn main() {
    let input: GeologyMap = utils::str_to_vec(utils::read_file(utils::args().remove(0)))
        .iter()
        .map(|r| row_to_geology(r))
        .collect();
    let mut map = GPS::new(input.clone());

    // pt 1
    let r1 = map.move_until_espcaped((3, 1));
    map.reset_location();
    println!("pt1 collision_count: {}", r1);

    // pt 2
    let r2 = map.move_until_espcaped((1, 1)); //Right 1, down 1.
    map.reset_location(); // Right 3, down 1. (This is the slope you already checked.)
    let r3 = map.move_until_espcaped((5, 1)); // Right 5, down 1.
    map.reset_location();
    let r4 = map.move_until_espcaped((7, 1)); // Right 7, down 1.
    map.reset_location();
    let r5 = map.move_until_espcaped((1, 2)); // Right 1, down 2.
    map.reset_location();

    println!("pt2 collisions: {},{},{},{},{}", r1, r2, r3, r4, r5);
    println!("pt 2 maths: {}", r1 * r2 * r3 * r4 * r5);
}
