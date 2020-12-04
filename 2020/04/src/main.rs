use utils;

fn has_fields(input: &str) -> bool {
    input.contains("byr")
        && input.contains("iyr")
        && input.contains("eyr")
        && input.contains("hgt")
        && input.contains("hcl")
        && input.contains("ecl")
        && input.contains("pid")
}

#[derive(Debug)]
enum Measure {
    In,
    Cm,
    Initialized,
}

impl Default for Measure {
    fn default() -> Self {
        Measure::Initialized
    }
}

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
    Initialized,
}

impl Default for EyeColor {
    fn default() -> Self {
        EyeColor::Initialized
    }
}

#[derive(Default, Debug)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Option<(u16, Measure)>,
    hcl: String,
    ecl: Option<EyeColor>,
    pid: String,
}

impl Passport {
    pub fn new(str: &str) -> Passport {
        let list: Vec<&str> = str.trim().split_whitespace().collect();
        let mut passport = Passport::default();
        for field in list {
            let split: Vec<&str> = field.split(":").collect();
            match split[..] {
                ["byr", data] => passport.byr = data.parse::<u16>().unwrap(),
                ["iyr", data] => passport.iyr = data.parse::<u16>().unwrap(),
                ["eyr", data] => passport.eyr = data.parse::<u16>().unwrap(),
                ["hgt", data] => {
                    passport.hgt = if data.ends_with("cm") {
                        let m = data.trim_end_matches("cm");
                        Some((m.parse::<u16>().unwrap(), Measure::Cm))
                    } else if data.ends_with("in") {
                        let m = data.trim_end_matches("in");
                        Some((m.parse::<u16>().unwrap(), Measure::In))
                    } else {
                        None
                    }
                }
                ["hcl", data] => passport.hcl = data.to_string(),
                ["ecl", data] => {
                    passport.ecl = match data {
                        "amb" => Some(EyeColor::Amb),
                        "blu" => Some(EyeColor::Blu),
                        "brn" => Some(EyeColor::Brn),
                        "gry" => Some(EyeColor::Gry),
                        "grn" => Some(EyeColor::Grn),
                        "hzl" => Some(EyeColor::Hzl),
                        "oth" => Some(EyeColor::Oth),
                        _ => None,
                    }
                }
                ["pid", data] => {
                    passport.pid = data.to_string();
                }
                ["cid", _] => (), // purposefully ignore country code
                [field_name, data] => panic!("Unknown field arrangement: {}:{}", field_name, data),
                _ => panic!("field of unknown size"),
            };
        }
        passport
    }

    pub fn validate(&self) -> bool {
        if self.byr < 1920 || self.byr > 2002 {
            return false;
        }
        if self.iyr < 2010 || self.iyr > 2020 {
            return false;
        }
        if self.eyr < 2020 || self.eyr > 2030 {
            return false;
        }
        match &self.hgt {
            None => {
                return false;
            }
            Some((num, m)) => match m {
                Measure::Initialized => {
                    return false;
                }
                Measure::Cm => {
                    if num < &150 || num > &193 {
                        return false;
                    }
                }
                Measure::In => {
                    if num < &59 || num > &76 {
                        return false;
                    }
                }
            },
        };
        if self.hcl.len() < 7 {
            return false;
        }
        if self.hcl.starts_with('#') == false {
            return false;
        }
        if self.hcl.chars().all(|c| match c {
            '#' => true,
            '0'..='9' | 'a'..='f' => true,
            _ => false,
        }) == false
        {
            return false;
        }
        match &self.ecl {
            None => return false,
            Some(_) => (),
        };

        if self.pid.len() != 9 {
            return false;
        }

        if self.pid.chars().all(|c| match c {
            '0'..='9' => return true,
            _ => return false,
        }) == false
        {
            return false;
        }

        true
    }
}

fn main() {
    let input: Vec<String> = utils::read_file(utils::args().remove(0))
        .split("\n\n")
        .map(|i| i.to_owned())
        .collect();

    let contains_required_fields = input.iter().filter(|i| has_fields(i));

    println!("pt 1: {}", contains_required_fields.clone().count());

    let parsed_passports = contains_required_fields.map(|i| Passport::new(i));
    let valid_passports = parsed_passports.filter(|p| p.validate());

    println!("pt 2: {}", valid_passports.clone().count());
}
