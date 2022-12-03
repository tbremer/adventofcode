use std::collections::HashSet;

pub fn str_val(str: &str) -> u32 {
    let b = str.as_bytes()[0];

    if b > 96 {
        (b - 96).into()
    } else {
        (b - 38).into()
    }
}

pub fn string_to_hashset(s: &String) -> HashSet<String> {
    s.chars().map(|c| c.to_string()).collect()
}

pub struct Input {
    pub pt1: Vec<(String, String)>,
    pub pt2: Vec<(String, String, String)>,
}

pub fn parse_input(input: &str) -> Input {
    Input {
        pt1: parse_pt_1(input),
        pt2: parse_pt_2(input),
    }
}

pub fn parse_pt_1(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            let line = l
                .split("")
                .map(|s| s.to_owned().to_string())
                .collect::<Vec<String>>();
            let len = line.len();
            let tuple = line.split_at(len / 2).clone();

            (tuple.0.join(""), tuple.1.join(""))
        })
        .collect()
}

pub fn parse_pt_2(input: &str) -> Vec<(String, String, String)> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .chunks(3)
        .map(|c| match c[..] {
            [x, y, z] => (x.to_string(), y.to_string(), z.to_string()),
            _ => panic!("Nope"),
        })
        .collect()
}
