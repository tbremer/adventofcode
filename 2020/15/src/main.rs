use std::collections::HashMap;

fn main() {
    let mut args = utils::args();
    let file = utils::read_file(args.remove(0));
    let num_of_rounds = args.remove(0).parse::<usize>().unwrap() + 1;
    let input: Vec<usize> = file
        .clone()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let twenty_twenty_number = find_number_by_count(num_of_rounds, &input);

    println!("twenty_twenty_number: {}", twenty_twenty_number);
}

fn find_number_by_count(num_of_rounds: usize, initializer: &Vec<usize>) -> usize {
    let mut round_number = 1;
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut prev_number_spoken = usize::MAX;

    for item in initializer {
        match map.get(item) {
            None => {
                map.insert(item.clone(), (usize::MAX, round_number));
            }
            Some(tuple) => {
                map.insert(item.clone(), (tuple.1, round_number));
            }
        }

        prev_number_spoken = item.clone();
        round_number += 1;
    }

    while round_number < num_of_rounds {
        let vec = match map.get(&prev_number_spoken) {
            None => panic!("{} never inserted", prev_number_spoken),
            Some(v) => v.clone(),
        };

        if vec.0 == usize::MAX {
            let zero_tuple = map.get(&0).unwrap().clone();

            map.insert(0, (zero_tuple.1, round_number));

            prev_number_spoken = 0;
        } else {
            let new_num = vec.1 - vec.0;

            let v = match map.get(&new_num) {
                Some(v) => v.clone(),
                None => (usize::MAX, usize::MAX),
            };

            map.insert(new_num, (v.1, round_number));

            prev_number_spoken = new_num;
        }

        // println!("speaking: {}\n", prev_number_spoken);
        round_number += 1;
    }

    prev_number_spoken
}
