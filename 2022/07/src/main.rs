use day_07::{calc_dir, parse, FileType};

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("Welcome to day-07 — Running test suite first!\n");

    let input_data = utils::read_file(utils::args().remove(0), None);

    test_1();
    println!("Puzzle 1: {:?}\n", pt_1(&input_data));

    test_2();
    println!("Puzzle 2: {:?}\n", pt_2(&input_data));
}

fn pt_1(input: &str) -> usize {
    let fs = parse(input);
    let it_fs = fs.iter();
    let mut total_size = 0;

    for file in it_fs {
        if file.is == FileType::Directory {
            let size = calc_dir(file.idx, &fs);

            if (size > 100000) == false {
                total_size += size
            }
        }
    }

    total_size
}

fn pt_2(input: &str) -> usize {
    let total_space = 70_000_000;
    let req_free = 30_000_000;
    let fs = parse(input);
    let fs_it = fs.iter();
    let root_size = calc_dir(0, &fs);
    let cur_space = total_space - root_size;
    let needed = req_free - cur_space;

    let mut dirs: Vec<usize> = fs_it
        .filter(|file| file.is == FileType::Directory)
        .map(|dir| calc_dir(dir.idx, &fs))
        .collect();

    dirs.sort();

    for dir in dirs.iter() {
        if dir > &needed {
            return *dir;
        }
    }

    // println!("{:?}", (cur_space, needed));

    0
}

fn test_1() {
    assert_eq!(
        pt_1(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        ),
        95437
    );
    println!("Suite 1 passes");
}

fn test_2() {
    assert_eq!(
        pt_2(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        ),
        24933642
    );
    println!("Suite 2 passes");
}
