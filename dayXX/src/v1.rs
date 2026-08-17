use std::{fs::File, io::{BufRead, BufReader}};

pub fn v1() {
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    for line in lines {
        println!("{line}");
    }

    println!("v1");
}