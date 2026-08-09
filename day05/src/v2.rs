use std::{fs::File, io::{BufRead, BufReader}};

struct Crates {
    core: Vec<Vec<char>>,
}

impl Crates {
    fn from_diagram(diagram: Vec<Vec<char>>) -> Self {
        //Gets character at [-1][-2] and converts it to an int
        let core_len = (diagram[diagram.len() - 1][diagram[0].len() - 2] as usize) - ('0' as usize);
        let mut core: Vec<Vec<char>> = vec![Vec::new(); core_len];
        
        //Loops through diagram
        for x in 0..core_len {
            for y in (0..diagram.len()).rev() {
                //Formula for x to access correct part of diagram
                let chr = diagram[y][1 + 4*x];
                //If there is a crate, add it
                if chr.is_alphabetic() {
                    core[x].push(chr);
                }
            }
        }

        Self { core }
    }

    fn move_crates(&mut self, count: usize, src: usize, dest: usize) {
        let src_len = self.core[src-1].len();
        //Splits off top count crates from src
        let to_move = self.core[src-1].split_off(src_len - count);
        //Pushes them onto dest
        self.core[dest-1].extend(to_move);
    }

    fn get_message(&self) -> String {
        let mut res = String::new();

        for stack in &self.core {
            res.push(*stack.last().unwrap());
        };

        res
    }
}

pub fn v2() {
    //Read file into line reading buffer
    let file = File::open("src/input.txt").unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);    

    //Gets diagram
    let mut crate_diagram: Vec<Vec<char>> = vec![];
    //Up to empty line
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        crate_diagram.push(line.chars().collect());
    }

    //Creates crates object
    let mut crates = Crates::from_diagram(crate_diagram);

    //Prints instructions line by line
    for line in lines {
        //Split by spaces
        let words: Vec<&str> = line.split(" ").collect();

        //Parse into ints, passes to crates
        crates.move_crates(
            words[1].parse().unwrap(),
            words[3].parse().unwrap(),
            words[5].parse().unwrap()
        )
    }
    
    println!("{}", crates.get_message());
}