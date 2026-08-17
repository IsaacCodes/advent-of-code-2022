use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    hist: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        Rope {head: (0, 0), tail: (0, 0), hist: HashSet::from([(0, 0)])}
    }

    fn move_head(&mut self, dx: i32, dy: i32) {
        //Loop to do single moves
        for _ in 0..dx.abs() {
            self.move_head_single(dx.signum(), 0);
        }
        for _ in 0..dy.abs() {
            self.move_head_single(0, dy.signum());
        }
    }

    fn move_head_single(&mut self, dx: i32, dy: i32) {
        //Move head
        self.head.0 += dx;
        self.head.1 += dy;

        //If distance between head and tail is <= 1 for both x and y, don't need to move tail
        if (self.head.0 - self.tail.0).abs() <= 1 && (self.head.1 - self.tail.1).abs() <= 1 {
            return;
        }

        //Moving left/right, same row
        if dx != 0 && self.head.1 == self.tail.1 {
            self.tail.0 += dx;
        }
        //Moving up/down, same col
        else if dy != 0 && self.head.0 == self.tail.0 {
            self.tail.1 += dy;
        }
        //Move left/right, tail goes diagonal
        else if dx != 0 {
            //Set tail y to head y and increment tail x
            self.tail.1 = self.head.1;
            self.tail.0 += dx;
        }
        //Move up/down, tail goes diagonal
        else if dy != 0 {
            //Set tail x to head x and increment tail y
            self.tail.0 = self.head.0;
            self.tail.1 += dy;
        }

        //Adds tail to history
        self.hist.insert(self.tail);
    }
}

pub fn v1() {
    let file = File::open("src/input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok);

    //Keep track of rope, starts overlapped at 0, 0
    let mut rope = Rope::new();

    for line in lines {
        //Format is `{dir}: {dist}`
        let dir = &line[0..=0];
        let dist: i32 = line[2..].parse().unwrap();

        //Note: using upwards and rightwars as positive
        match dir {
            "L" => rope.move_head(-dist, 0),
            "R" => rope.move_head(dist, 0),
            "U" => rope.move_head(0, dist),
            "D" => rope.move_head(0, -dist),
            _ => panic!("Bad direction!"),
        }
    }

    println!("{}", rope.hist.iter().count());
}