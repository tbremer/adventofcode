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

#[derive(Debug)]
struct GPS {
    coords: (usize, usize),
    map: GeologyMap,
}

impl GPS {
    pub fn new(geo: GeologyMap) -> GPS {
        GPS {
            coords: (0, 0),
            map: geo,
        }
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
    let input: Vec<Vec<Geology>> = utils::str_to_vec(utils::read_file(utils::args().remove(0)))
        .iter()
        .map(|r| row_to_geology(r))
        .collect();
    let mut map = GPS::new(input.clone());
    let mut collision_count = 0;

    loop {
        map.move_right(3);
        match map.move_down(1) {
            Err(_) => break,
            Ok(_) => (),
        }

        if map.peek_geo() == &Geology::Tree {
            collision_count += 1
        }
    }

    println!("pt1 collision_count: {}", collision_count);
}
