use std::{fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

pub fn v1() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);


    //State vars
    let mut total = 0;
    //Read each line
    for line in lines {
        //This monstrosityy parse s1-e1,s2-e2
        let ((s1, e1), (s2, e2)) = line
            .split(",")
            .map(
                |range| range
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple().unwrap()
            )
            .collect_tuple().unwrap();

        //Range 1 contains 2 or vice versa
        total += (s1 <= s2 && e1 >= e2 || s1 >= s2 && e1 <= e2) as u32;
    }

    println!("{total}");
}