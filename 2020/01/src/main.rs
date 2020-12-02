use utils;

#[derive(Clone, Debug)]
struct Permutation {
    cur_idx: (usize, usize),
    items: Vec<u16>,
}

impl Permutation {
    fn new(vec: Vec<u16>) -> Permutation {
        Permutation {
            cur_idx: (0, 1),
            items: vec,
        }
    }
}

impl Iterator for Permutation {
    type Item = (u16, u16);
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.items.len();
        if self.cur_idx.1 == len {
            self.cur_idx.0 = self.cur_idx.0 + 1;
            self.cur_idx.1 = self.cur_idx.0 + 1;
        }

        if self.cur_idx.0 >= len {
            return None;
        }

        let idx0 = self.cur_idx.0;
        let idx1 = self.cur_idx.1;

        self.cur_idx.1 = self.cur_idx.1 + 1;

        Some((self.items[idx0], self.items[idx1]))
    }
}

fn main() {
    let input_data = utils::iter_to_int(
        utils::read_file(utils::args().remove(0))
            .split("\n")
            .map(|i| i.to_owned())
            .collect(),
    );
    // let monkey = (input_data);
    let foo = Permutation::new(input_data.clone());

    for (a, b) in foo.clone() {
        if a + b == 2020 {
            let maths: usize = usize::from(a) * usize::from(b);
            println!("pair: {}", maths);
            break;
        }
    }

    for (a, b) in foo.clone() {
        let remainder = match 2020u16.checked_sub(a) {
            Some(v) => match v.checked_sub(b) {
                None => continue,
                Some(v) => v,
            },
            None => continue,
        };

        if input_data.contains(&remainder) {
            let maths: usize = usize::from(a) * usize::from(b) * usize::from(remainder);
            println!("triples: {}", maths);
            break;
        }
    }
}
