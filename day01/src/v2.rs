use std::{cmp::Reverse, collections::BinaryHeap, fs::File, io::{BufRead, BufReader}};

pub fn v2() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        //Adds sentinel empty line
        .chain(std::iter::once(String::new()));

    //State vars
    let mut min_heap = BinaryHeap::new();
    let mut current = 0;
    //Read each line
    for line in lines {
        //Reset on empty
        if line.is_empty() {
            //Push item to heap
            min_heap.push(Reverse(current));
            current = 0;
            //3 Items max on heap
            if min_heap.len() > 3 {
                min_heap.pop();
            }
        }
        //Otherwise increment
        else {
            current += line.parse::<i32>().unwrap();
        }
    }

    //Sum 3 items on min heap
    println!("{}", min_heap.iter().map(|&Reverse(val)| val).sum::<i32>())
}