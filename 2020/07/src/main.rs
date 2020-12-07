use std::collections::HashSet;
use utils;

fn main() {
    let input = utils::read_file(utils::args().remove(0));
    let raw_rules: Vec<Rule> = input.split("\n").map(|r| parse_rule(r)).collect();
    let set = find_all_containers("shiny gold", &raw_rules);

    println!("pt1: {}", set.len());
}

type CountRule = (usize, String);
type Rule = (String, Option<Vec<CountRule>>);

fn parse_rule(rule: &str) -> Rule {
    let mut group = rule.split("bags contain").into_iter();
    let base = group.next().unwrap().trim();

    let mut subrules = vec![];

    for sr in group {
        let rule = sr.trim();
        if rule == "no other bags." {
            break;
        }

        let sub = rule.split(",");

        for s in sub {
            let monkey = s
                .trim()
                .replace("bags", "")
                .replace("bag", "")
                .replace(",", "")
                .replace(".", "");
            let num_idx = monkey.find(" ").unwrap();
            let num = monkey.get(0..num_idx).unwrap();
            let rule = monkey.get(num_idx..).unwrap().trim();

            subrules.push((num.parse::<usize>().unwrap(), rule.to_string()));
        }
    }

    (
        base.to_string(),
        if subrules.len() == 0 {
            None
        } else {
            Some(subrules)
        },
    )
}

fn find_all_containers(color: &str, rule_set: &Vec<Rule>) -> HashSet<(usize, String)> {
    let mut can_hold = HashSet::new();

    for (name, sub_rules) in rule_set.iter() {
        match &sub_rules {
            None => continue,
            Some(rules) => {
                can_hold = rules.iter().fold(can_hold, |mut set, cur| {
                    if &cur.1 == color {
                        set.insert((cur.0, name.clone()));

                        &set | &find_all_containers(name, rule_set)
                    } else {
                        set
                    }
                })
            }
        }
    }

    can_hold
}
