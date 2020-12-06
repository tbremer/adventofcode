use utils;

#[derive(Debug)]
struct Group {
    count: usize,
    first: Vec<char>,
    rest: Vec<Vec<char>>,
}

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let input = file.split("\n\n");

    let pt1 = input
        .clone()
        .map(|i| {
            let mut input: Vec<char> = i.to_owned().replace("\n", "").chars().collect();

            input.sort();
            input.dedup();

            input.len()
        })
        .fold(0, |acc, x| acc + x);

    println!("pt1: {}", pt1);

    let pt2: Vec<Group> = input
        .clone()
        .map(|i| {
            i.split("\n")
                .map(|i| i.to_owned().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .map(|mut g| {
            let first = g.remove(0);

            Group {
                first,
                rest: g.to_owned(),
                count: 0,
            }
        })
        .collect();

    let count = pt2.iter().fold(0, |mut acc, cur| {
        if cur.rest.len() == 0 {
            return acc + cur.first.len();
        }

        for char in &cur.first {
            let mut rest = cur.rest.iter();
            if rest.all(|v| v.contains(char)) {
                acc += 1;
            }
        }

        acc
    });
    println!("count: {:?}", count);
}
