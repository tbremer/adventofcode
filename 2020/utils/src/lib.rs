use core::panic;
use std::env;
use std::fs::File;
use std::io::prelude::Read;

pub fn args() -> Vec<String> {
    let mut arguments: Vec<String> = env::args_os()
        .map(|i| match i.to_str() {
            Some(v) => v.to_owned(),
            None => panic!("No string found"),
        })
        .collect();

    arguments.split_off(1)
}

pub fn read_file(path: String) -> String {
    println!("path: {}", path);
    let f = File::open(path);
    let mut contents = String::new();

    match f {
        Ok(mut file) => match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => panic!(&e.to_string()),
        },
        Err(e) => panic!(&e.to_string()),
    }

    contents.trim().to_owned()
}

pub fn iter_to_int(items: Vec<String>) -> Vec<u16> {
    let mut new_list = vec![];

    for item in items {
        match item.parse::<u16>() {
            Ok(v) => new_list.push(v),
            Err(e) => panic!(&e.to_string()),
        };
    }

    new_list
}
