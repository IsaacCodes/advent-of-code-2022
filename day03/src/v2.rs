use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

//Alphabetic lowercase 1-26, uppercase 27-52
fn get_value(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        1 + (c as u32) - ('a' as u32)
    }
    else {
        27 + (c as u32) - ('A' as u32)
    }
}

//Searchs sack for overlap and returns char
fn search_sack(sacks: &[String]) -> char {
    //Convert first to sacks to hashes
    let sack1_hash: HashSet<char> = sacks[0].chars().collect();
    let sack2_hash: HashSet<char> = sacks[1].chars().collect();
    let sack3 = &sacks[2];

    //Checks comp2 against comp1's hash
    for char in sack3.chars() {
        if sack1_hash.contains(&char) && sack2_hash.contains(&char) {
            return char;
        }
    }

    //Should never happen
    panic!("No overlap!");
}

pub fn v2() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    //Split into chunks with itertools
    let mut total = 0;
    for chunk in &lines.chunks(3) {
        //Process each chunk of 3 sacks
        let sacks: Vec<String> = chunk.collect();
        total += get_value(search_sack(&sacks));
    }
    
    println!("{total}")
}