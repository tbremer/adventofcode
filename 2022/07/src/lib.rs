use core::panic;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Directory,
    File,
}
#[derive(Debug)]
pub struct File {
    pub is: FileType,
    pub idx: usize,
    pub name: String,
    pub path: String,
    size: usize,
    parent: Option<usize>,
    files: Vec<usize>,
}

pub fn parse(input: &str) -> Vec<File> {
    let mut lines = input.lines();
    let mut cur_parent: usize = 0;
    let mut dir_hash: HashMap<String, usize> = HashMap::new();
    let mut fs: Vec<File> = vec![File {
        is: FileType::Directory,
        idx: 0,
        name: "/".to_string(),
        size: 0,
        parent: None,
        files: vec![],
        path: "/".to_string(),
    }];

    dir_hash.insert("/".to_string(), 0);

    // skip root, we know it's there
    lines.next();

    for line in lines {
        if line == "$ ls" {
            continue;
        }
        if line.starts_with("$ cd") {
            let dir_name = last_word(line);

            if dir_name == ".." {
                let cur_parent_dir = fs.get(cur_parent).unwrap();
                cur_parent = cur_parent_dir.parent.unwrap();
            } else {
                let parent = fs.get(cur_parent).unwrap();
                let mut p = parent.path.clone();
                p.push_str(&dir_name);

                let dir_idx = match dir_hash.get(&p) {
                    Some(idx) => idx.to_owned(),
                    None => panic!("Could not find {:?}, in {:?}", dir_name, dir_hash),
                };

                cur_parent = dir_idx;
            }
        } else if line.starts_with("dir") {
            let dir_name = last_word(line);
            let idx = fs.len();
            let parent = fs.get(cur_parent).unwrap();
            let mut path = parent.path.clone();
            path.push_str(&dir_name);

            let hash_key = path.clone();

            fs.push(File {
                idx,
                path,
                is: FileType::Directory,
                name: dir_name,
                size: 0,
                parent: Some(cur_parent),
                files: vec![],
            });

            let p = fs.get_mut(cur_parent).unwrap();
            p.files.push(idx);

            dir_hash.insert(hash_key, idx);
        } else if starts_numeric(line) {
            let (size, name) = f_stat(line);
            let idx = fs.len();
            let parent = fs.get(cur_parent).unwrap();
            let mut path = parent.path.clone();

            path.push_str(&name);

            fs.push(File {
                idx,
                name,
                size,
                path,
                is: FileType::File,
                parent: Some(cur_parent),
                files: vec![],
            });

            let p = fs.get_mut(cur_parent).unwrap();
            p.files.push(idx);
        } else {
            panic!("cannot parse: {:?}", line);
        }
    }

    fs
}

pub fn calc_dir(idx: usize, fs: &Vec<File>) -> usize {
    let f = fs.get(idx).unwrap();
    let it_files = f.files.clone();

    it_files.iter().fold(0, |acc, f_id| {
        let file = fs.get(*f_id).unwrap();

        if file.is == FileType::Directory {
            acc + calc_dir(file.idx, fs)
        } else {
            acc + file.size
        }
    })
}

fn last_word(i: &str) -> String {
    i.split(" ").last().unwrap().to_string()
}

fn starts_numeric(i: &str) -> bool {
    i[0..1].parse::<u8>().is_ok()
}

fn f_stat(f: &str) -> (usize, String) {
    let mut sp = f.split(" ");
    let size = sp.next().unwrap();
    let name = sp.next().unwrap();

    (size.parse::<usize>().unwrap(), name.to_string())
}
