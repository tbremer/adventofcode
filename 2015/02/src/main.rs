#[derive(Debug)]
struct Present {
    w: u32,
    h: u32,
    l: u32,
}

impl Present {
    fn two_smallest_sides(&self) -> (u32, u32) {
        let mut a = vec![self.w, self.h, self.l];
        a.sort();

        (a[0], a[1])
        // (0, 0)
    }

    pub fn from_str(str: &str) -> Present {
        let parsed_dimensions: Vec<u32> =
            utils::iter_to_int::<u32>(utils::str_to_vec(str, Some("x")));
        let l = parsed_dimensions[0];
        let w = parsed_dimensions[1];
        let h = parsed_dimensions[2];
        Present { w: w, h: h, l: l }
    }

    // formula: 2*l*w + 2*w*h + 2*h*l.
    pub fn surface_area(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    pub fn smallest_area(&self) -> u32 {
        let (a, b) = self.two_smallest_sides();

        a * b
    }

    pub fn perimeter(&self) -> u32 {
        let (a, b) = self.two_smallest_sides();

        a + a + b + b
    }

    pub fn volume(&self) -> u32 {
        self.w * self.h * self.l
    }
}

fn main() {
    println!("Welcome to day-1, running test suite first!");
    test_1();
    test_2();

    println!("Test suite complete, solving puzzle…");

    let input_data = utils::str_to_vec(&utils::read_file(utils::args().remove(0)), None);

    println!("Wrapping Paper: {:?}", pt_1(&input_data));
    println!("Ribbon: {:?}", pt_2(&input_data));
    // println!("Position when negative: {:?}", pt_2(&input_data))
}

fn pt_1a(dimensions: &str) -> u32 {
    let box_to_wrap = Present::from_str(dimensions);

    box_to_wrap.surface_area() + box_to_wrap.smallest_area()
}

fn pt_1(input: &Vec<String>) -> u32 {
    input.iter().fold(0, |acc, cur| acc + pt_1a(cur))
}

fn pt_2a(dimensions: &str) -> u32 {
    let box_to_wrap = Present::from_str(dimensions);

    box_to_wrap.perimeter() + box_to_wrap.volume()
}

fn pt_2(input: &Vec<String>) -> u32 {
    input.iter().fold(0, |acc, cur| acc + pt_2a(cur))
}

fn test_1() {
    assert_eq!(pt_1a("2x3x4"), 58);
    assert_eq!(pt_1a("1x1x10"), 43);
    println!("suite 1 passes passes")
}

fn test_2() {
    assert_eq!(pt_2a("2x3x4"), 34);
    assert_eq!(pt_2a("1x1x10"), 14);
    println!("suite 2 passes passes")
}
