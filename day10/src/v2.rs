use std::{fs::File, io::{BufRead, BufReader}};

fn increment_cycles(n: i32, cycles: &mut i32, x: i32) {
    //Increments and check cycle count each time
    for _ in 0..n {
        *cycles += 1;
        //Print # if cycle is in sprite's x range
        if x - 1 <= *cycles - 1 && *cycles - 1 <= x + 1 {
            print!("#");
        }
        //Otherwise .
        else {
            print!(".")
        }

        //Newline / reset every 40 cycles
        if *cycles % 40 == 0 {
            println!();
            *cycles = 0;
        }
    }
}

pub fn v2() {
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    //1-indexed
    let mut cycles = 0;
    //0-indexed, center of 3 width sprite
    let mut x = 1;

    for line in lines {
        //noop, 1 cycle, do nothing
        if line == "noop" {
            increment_cycles(1, &mut cycles, x);
        }
        //addx, 2 cycles, add to x
        else {
            increment_cycles(2, &mut cycles, x);
            //Adds to x
            x += line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        }
    }

    //Output will have been printed by increment_cycles
}