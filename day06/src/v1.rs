use std::{collections::VecDeque, fs};

fn has_duplicates(container: &VecDeque<char>) -> bool {
    for i in 0..container.len() {
        for j in i+1..container.len() {
            if container[i] == container[j] {
                return true;
            }
        }
    }

    false
}

pub fn sol(n: u32) {
    //Read file into string
    let raw_data = fs::read_to_string("src/input.txt").unwrap();
    let data = raw_data.trim_end().chars();

    //Proccess until it hits requirement
    let mut counter = 0;
    let mut marker = VecDeque::new();
    for chr in data {
        //Push new char
        marker.push_back(chr);
        counter += 1;

        //Start checking for dups + popping after 4 to always maintain last 4 chars
        if counter >= n {
            if !has_duplicates(&marker) {
                break;
            }
            marker.pop_front();
        }
    }

    println!("{counter}");
}

pub fn v1() {
    sol(4);
}