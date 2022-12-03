use std::collections::HashSet;

fn is_lower(c: &str) -> bool {
    c.chars().next().unwrap().is_lowercase()
}

pub fn str_val(str: &str) -> u32 {
    match str {
        "a" | "A" => {
            if is_lower(str) {
                1
            } else {
                27
            }
        }
        "b" | "B" => {
            if is_lower(str) {
                2
            } else {
                28
            }
        }
        "c" | "C" => {
            if is_lower(str) {
                3
            } else {
                29
            }
        }
        "d" | "D" => {
            if is_lower(str) {
                4
            } else {
                30
            }
        }
        "e" | "E" => {
            if is_lower(str) {
                5
            } else {
                31
            }
        }
        "f" | "F" => {
            if is_lower(str) {
                6
            } else {
                32
            }
        }
        "g" | "G" => {
            if is_lower(str) {
                7
            } else {
                33
            }
        }
        "h" | "H" => {
            if is_lower(str) {
                8
            } else {
                34
            }
        }
        "i" | "I" => {
            if is_lower(str) {
                9
            } else {
                35
            }
        }
        "j" | "J" => {
            if is_lower(str) {
                10
            } else {
                36
            }
        }
        "k" | "K" => {
            if is_lower(str) {
                11
            } else {
                37
            }
        }
        "l" | "L" => {
            if is_lower(str) {
                12
            } else {
                38
            }
        }
        "m" | "M" => {
            if is_lower(str) {
                13
            } else {
                39
            }
        }
        "n" | "N" => {
            if is_lower(str) {
                14
            } else {
                40
            }
        }
        "o" | "O" => {
            if is_lower(str) {
                15
            } else {
                41
            }
        }
        "p" | "P" => {
            if is_lower(str) {
                16
            } else {
                42
            }
        }
        "q" | "Q" => {
            if is_lower(str) {
                17
            } else {
                43
            }
        }
        "r" | "R" => {
            if is_lower(str) {
                18
            } else {
                44
            }
        }
        "s" | "S" => {
            if is_lower(str) {
                19
            } else {
                45
            }
        }
        "t" | "T" => {
            if is_lower(str) {
                20
            } else {
                46
            }
        }
        "u" | "U" => {
            if is_lower(str) {
                21
            } else {
                47
            }
        }
        "v" | "V" => {
            if is_lower(str) {
                22
            } else {
                48
            }
        }
        "w" | "W" => {
            if is_lower(str) {
                23
            } else {
                49
            }
        }
        "x" | "X" => {
            if is_lower(str) {
                24
            } else {
                50
            }
        }
        "y" | "Y" => {
            if is_lower(str) {
                25
            } else {
                51
            }
        }
        "z" | "Z" => {
            if is_lower(str) {
                26
            } else {
                52
            }
        }
        _ => panic!("Unknown str: {}", str),
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
