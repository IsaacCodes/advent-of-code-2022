use std::{cmp::max, fs::File, io::{BufRead, BufReader}};

pub fn v1() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        //Adds sentinel empty line
        .chain(std::iter::once(String::new()));

    //State vars
    let mut highest = 0;
    let mut current = 0;
    //Read each line
    for line in lines {
        //Reset on empty
        if line.is_empty() {
            highest = max(highest, current);
            current = 0;
        }
        //Otherwise increment
        else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("{highest}")
}