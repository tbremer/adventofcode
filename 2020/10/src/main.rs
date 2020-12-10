use std::collections::HashMap;
use utils;

fn main() {
    let file = utils::read_file(utils::args().remove(0));
    let mut input: Vec<usize> = file
        .split("\n")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let mut diff_count = (0, 0);

    input.sort();

    let mut input_iter = input.iter();
    let mut item = &0;

    loop {
        let tmp = input_iter.next();

        match tmp {
            None => {
                diff_count.1 += 1;
                break;
            }
            Some(v) => {
                let diff = v - item;

                match diff {
                    1 => diff_count.0 += 1,
                    3 => diff_count.1 += 1,
                    _ => panic!("Unknown diff: {}", diff),
                }

                item = v;
            }
        }
    }

    println!("pt 1 maths: {:?}", diff_count.0 * diff_count.1);
    let max = input
        .clone()
        .iter()
        .fold(0, |acc, cur| std::cmp::max(acc, *cur));
    let mut all = vec![0, max + 3];

    all.extend(input.iter());

    all.sort();

    let mut cache = HashMap::new();
    println!("pt 2: {:?}", count_paths(0, &all, &mut cache));
}

fn count_paths(i: usize, list: &Vec<usize>, cache: &mut HashMap<usize, usize>) -> usize {
    if i == list.len() - 1 {
        return 1;
    }
    let mut count = 0;
    let item = list[i];

    if cache.contains_key(&item) {
        return *cache.get(&item).unwrap();
    }

    let mut max = list.len();
    if i + 4 < max {
        max = i + 4;
    }

    let range = std::ops::Range {
        start: i + 1,
        end: max,
    };

    for idx in range {
        if list[idx] - item <= 3 {
            count += count_paths(idx, list, cache);
        }
    }
    cache.insert(item, count);
    count
}
