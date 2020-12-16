fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let mut sections = file.split("\n\n").collect::<Vec<&str>>();
    let mut walker = Walker::new(sections.remove(0));
    let valid_ranges = walker.extract_ranges();
    let nearby = parse_nearby(sections.remove(1));
    let (valid, invalid) = nearby.iter().fold((vec![], vec![]), |mut acc, ticket| {
        if valid_ranges
            .iter()
            .any(|(min, max)| ticket >= &min && ticket <= &max)
        {
            acc.0.push(ticket);
        } else {
            acc.1.push(ticket);
        }

        acc
    });
    println!(
        "pt 1: {:?}",
        invalid.iter().fold(0usize, |acc, cur| acc + *cur)
    );

    println!("pt 2: {:?}", valid);
}

fn parse_nearby(str: &str) -> Vec<usize> {
    let raw = str.replace("nearby tickets:\n", "").replace("\n", ",");
    raw.split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Walker<'a, 'b> {
    value: &'a str,  // the source
    bytes: &'b [u8], // the source as byte array
    len: usize,
    pos: usize,       // position in source -- equates to char
    next: usize,      // next item, for peeking
    char: Option<u8>, // current character
}

impl<'a, 'b> Walker<'a, 'b> {
    pub fn new(str: &'a str) -> Walker {
        let mut walker = Walker {
            value: str,
            bytes: str.as_bytes(),
            len: str.len(),
            pos: 0,
            next: 0,
            char: None,
        };

        // get things moving
        walker.read_char();

        walker
    }

    fn read_char(&mut self) {
        if self.next >= self.len {
            self.char = None
        } else {
            self.char = Some(self.bytes[self.next]);
        }

        self.pos = self.next;
        self.next += 1;
    }

    // fn peek_char(&self) -> Option<u8> {
    //     if self.next >= self.len {
    //         None
    //     } else {
    //         Some(self.bytes[self.next])
    //     }
    // }

    fn read_int(&mut self) -> usize {
        let start_pos = self.pos;
        loop {
            match self.char {
                Some(b'0'..=b'9') => self.read_char(),
                _ => break,
            }
        }

        let value = &self.value[start_pos..self.pos];

        match value.parse::<usize>() {
            Ok(num) => num,
            Err(e) => panic!(e),
        }
    }

    fn extract_ranges(&mut self) -> Vec<(usize, usize)> {
        let mut ranges = vec![];

        loop {
            match self.char {
                None => break,
                Some(b'0'..=b'9') => {
                    let range_start = self.read_int();
                    #[allow(unused_assignments)]
                    let mut range_end = usize::MIN;

                    match self.char {
                        Some(b'-') => {
                            self.read_char();

                            range_end = self.read_int();
                        }
                        _ => panic!(
                            "Invalid range. Expected `-`, but found `{}`",
                            self.char.unwrap()
                        ),
                    }

                    ranges.push((range_start, range_end));
                }
                _ => self.read_char(),
            }
        }

        ranges
    }
}
