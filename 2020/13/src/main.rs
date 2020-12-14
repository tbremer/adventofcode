use utils;

fn main() {
    let path_name = utils::args().remove(0);
    let mut file = utils::read_file(path_name.clone()).replace("x,", "");
    let mut input = file.lines();

    let target = input.next().unwrap().parse::<i32>().unwrap();
    let values = input
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<i32>().unwrap());

    println!("target: {}, values: {:?}", target, values);

    let pt1 = values.fold((i32::MAX, i32::MAX, i32::MAX), |acc, cur| {
        if target / cur == cur {
            return (target, 0, cur);
        }

        let monkey = (target / cur | 0) + 1;

        if monkey * cur < acc.0 {
            return (monkey * cur, monkey * cur - target, cur);
        }

        acc
    });

    println!("pt 1 tuple: {:?}", pt1);
    println!("pt 1 maths: {:?}", pt1.1 * pt1.2);

    // pt 2 was completely stolen from here:
    // https://dev.to/qviper/advent-of-code-2020-python-solution-day-13-24k4
    // I don't know math. lol

    file = utils::read_file(path_name);
    let mut lines = file.lines();
    let numbers_raw = lines.nth(1).unwrap();
    let nums = numbers_raw.split(",").map(|i| match i {
        "x" => isize::MAX,
        v => v.parse::<isize>().unwrap(),
    });
    let mut map = std::collections::HashMap::new();

    for (idx, bus_id) in nums.enumerate() {
        if bus_id == isize::MAX {
            continue;
        }
        let i = idx as isize;

        map.insert(bus_id, (-i).rem_euclid(bus_id));
    }

    let mut vals = vec![];

    for v in map.iter() {
        vals.push(v.0);
    }

    vals.sort();
    vals.reverse();

    let mut val = map.get(vals[0]).unwrap().clone();
    let mut r = vals[0].clone();
    let mut monkey = vals.iter();

    monkey.next();

    for bus_id in monkey {
        while val.rem_euclid(**bus_id) != *map.get(bus_id).unwrap() {
            val += r;
        }

        r *= *bus_id
    }

    println!("pt 2: {}", val);
}
