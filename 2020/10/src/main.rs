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
    println!(
        "pt 2 (recursive): {:?}",
        count_paths_recursive(0, &all, &mut cache)
    );
    println!("pt 2 (josiah): {:?}", count_paths_josiah(&all, max));
}

fn count_paths_josiah(list: &Vec<usize>, target: usize) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    let range = std::ops::Range {
        start: 0,
        end: list.len() - 1,
    };

    cache.insert(0, 1);

    for idx in range {
        let item = list[idx];
        let lookup = cache.clone();

        if item == 0 {
            continue;
        }

        let a = if item > 0 {
            match lookup.get(&(item - 1)) {
                None => &0,
                Some(v) => v,
            }
        } else {
            &0
        };

        let b = if item > 1 {
            match lookup.get(&(item - 2)) {
                None => &0,
                Some(v) => v,
            }
        } else {
            &0
        };

        let c = if item > 2 {
            match lookup.get(&(item - 3)) {
                None => &0,
                Some(v) => v,
            }
        } else {
            &0
        };

        cache.insert(item, a + b + c);
    }

    std::cmp::max(0, *cache.get(&target).unwrap())
}

fn count_paths_recursive(i: usize, list: &Vec<usize>, cache: &mut HashMap<usize, usize>) -> usize {
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
            count += count_paths_recursive(idx, list, cache);
        }
    }
    cache.insert(item, count);
    count
}
