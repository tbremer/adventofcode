use std::cmp::{max, min};
use std::ops::Range;
use utils;

fn main() {
    let mut args = utils::args();
    let (filename, offset) = (args.remove(0), args.remove(0));
    let offset = offset.parse::<usize>().unwrap();
    let mut idx = 0;
    let raw_file = utils::read_file(filename);
    let input: Vec<usize> = raw_file
        .split("\n")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    #[allow(unused_assignments)]
    let mut invalid_number: usize = 0;

    loop {
        let start = idx;
        let end = idx + offset;

        if end > input.len() {
            panic!("End ({}) out of bounds", end);
        }

        let range = Range { start, end };
        let preamble = &input[range];
        let item = input.get(end).unwrap();

        let any_match = preamble
            .iter()
            .filter(|cur| {
                let sub = max(item, *cur) - min(item, *cur);
                item != &sub && preamble.contains(&sub)
            })
            .collect::<Vec<&usize>>();

        if any_match.len() == 0 {
            println!("pt 1: {}", item);
            invalid_number = item.clone();
            break;
        }

        idx += 1;
    }

    let mut base_idx = 0;
    #[allow(unused_assignments)]
    let mut low_high = (0, 0);

    'outer: loop {
        let mut sum = 0;
        let mut min_max_vec = vec![];
        for num in &input[base_idx..] {
            sum += num;

            if sum == invalid_number {
                low_high = (find_min(num, &min_max_vec), find_max(num, &min_max_vec));
                break 'outer;
            }
            if sum > invalid_number {
                break;
            }
            min_max_vec.push(num);
        }

        base_idx += 1;
    }

    println!("pt 2: {:?}", low_high.0 + low_high.1);
}

fn find_min(base: &usize, nums: &Vec<&usize>) -> usize {
    nums.iter().fold(*base, |acc, cur| min(acc, **cur))
}

fn find_max(base: &usize, nums: &Vec<&usize>) -> usize {
    nums.iter().fold(*base, |acc, cur| max(acc, **cur))
}
