use std::collections::HashSet;
use utils;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let raw_rules: Vec<Rule> = file.split("\n").map(|line| parse_rule(line)).collect();

    println!("pt 1: {:?}", count("shiny gold", &raw_rules).len());
    println!("pt 2: {}", count_rec("shiny gold", &raw_rules) - 1);
}

type SubRule = (String, usize);
type Rule = (String, Vec<SubRule>);

fn parse_rule(rule: &str) -> Rule {
    let rule_tmp = split_at(&rule, "contain");

    let name = &rule_tmp[0];
    let mut sub_rules = vec![];
    let rule_set = split_at(&rule_tmp[1], ", ");

    for rule in rule_set {
        if rule.starts_with("no other") {
            break;
        }

        let mut rule: Vec<&str> = rule.splitn(2, " ").collect();
        let qty = rule.remove(0);
        let name = rule.remove(0);

        sub_rules.push((
            remove_rule_trailing_text(name),
            qty.parse::<usize>().unwrap(),
        ));
    }

    (name.replace(" bags", ""), sub_rules)
}

fn split_at(str: &str, split: &str) -> Vec<String> {
    str.split(split).map(|s| s.trim().to_owned()).collect()
}

fn remove_rule_trailing_text(str: &str) -> String {
    str.replace("bags", "")
        .replace("bag", "")
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

fn count(name: &str, rules: &Vec<Rule>) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();

    for r in rules {
        for sr in &r.1 {
            if &sr.0 == name {
                set.insert(r.0.to_string());
                set = &set | &count(&r.0, rules);
            }
        }
    }

    set
}

fn count_rec(needle: &str, haystack: &Vec<Rule>) -> usize {
    for straw in haystack {
        if straw.0 == needle {
            let c = straw
                .1
                .iter()
                .fold(1, |a, c| a + c.1 * count_rec(&c.0, haystack));
            return c;
        }
    }
    unreachable!()
}
