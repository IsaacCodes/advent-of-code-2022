use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

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
fn search_sack(sack: String) -> char {
    //Split up into 2 compartments
    let len = sack.chars().count();
    let mid = sack.char_indices().nth(len / 2).unwrap().0;
    let (comp1, comp2) = sack.split_at(mid);

    //Convert compartment 1 to a hash
    let comp1_hash: HashSet<char> = comp1.chars().collect();

    //Checks comp2 against comp1's hash
    for char in comp2.chars() {
        if comp1_hash.contains(&char) {
            return char;
        }
    }

    //Should never happen
    panic!("No overlap!");
}

pub fn v1() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    let total: u32 = lines.map(|sack| get_value(search_sack(sack))).sum();
    println!("{total}")
}