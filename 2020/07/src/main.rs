use std::collections::HashSet;
use utils;

fn main() {
    let input = utils::read_file(utils::args().remove(0));
    let raw_rules: Vec<Rule> = input.split("\n").map(|r| parse_rule(r)).collect();
    let set = find_all_containers("shiny gold", &raw_rules);

    println!("pt1: {}", set.len());

    let graph = directed_graph("shiny gold", &raw_rules);

    println!("pt2 graph: {:#?}", graph);
    println!("pt2 count: {}", count_graph(*graph.child));
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

#[derive(Debug, Clone)]
struct LinkedListIsh {
    node: String,
    count: usize,
    child: Box<Option<LinkedListIsh>>,
    sibling: Box<Option<LinkedListIsh>>,
}

impl Default for LinkedListIsh {
    fn default() -> Self {
        LinkedListIsh {
            node: String::new(),
            count: 1,
            child: Box::new(None),
            sibling: Box::new(None),
        }
    }
}

fn directed_graph(color: &str, rule_set: &Vec<Rule>) -> LinkedListIsh {
    let mut link = LinkedListIsh::default();
    for (name, children) in rule_set.iter() {
        if name != color {
            continue;
        }

        link.node = name.to_string();

        link.child = match children {
            None => Box::new(None),
            Some(kid_rules) => {
                let mut kids = kid_rules.iter();
                let (count, name) = kids.next().unwrap();
                let mut graph = directed_graph(name, rule_set);

                graph.count = count.clone();
                graph.sibling = match kids.next() {
                    None => Box::new(None),
                    Some((count, name)) => {
                        let mut sib_graph = directed_graph(name, rule_set);
                        sib_graph.count = count.clone();

                        Box::new(Some(sib_graph))
                    }
                };

                Box::new(Some(graph))
            }
        };
    }

    link
}

fn count_graph(graph: Option<LinkedListIsh>) -> usize {
    match graph {
        None => 0,
        Some(graph) => {
            println!(
                "{}: {}",
                graph.node,
                graph.count
                    + graph.count * count_graph(*graph.child.clone())
                    + count_graph(*graph.sibling.clone())
            );
            graph.count + graph.count * count_graph(*graph.child) + count_graph(*graph.sibling)
        }
    }
}
