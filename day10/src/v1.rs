use std::{fs::File, io::{BufRead, BufReader}};

fn increment_cycles(n: i32, total_strength: &mut i32, cycles: &mut i32, x: i32) {
    //Increments and check cycle count each time
    for _ in 0..n {
        *cycles += 1;
        if (*cycles + 20) % 40 == 0 {
            *total_strength += (*cycles) * x;
        }
    }
}

pub fn v1() {
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    //State vars
    let mut total_strength = 0;
    let mut cycles = 0;
    let mut x = 1;

    for line in lines {
        //noop, 1 cycle, do nothing
        if line == "noop" {
            increment_cycles(1, &mut total_strength, &mut cycles, x);
        }
        //addx, 2 cycles, add to x
        else {
            increment_cycles(2, &mut total_strength, &mut cycles, x);
            //Adds to x
            x += line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        }
    }

    println!("{total_strength}");
}