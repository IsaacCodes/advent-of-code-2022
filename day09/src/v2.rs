use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

struct Rope {
    knots: Vec<(i32, i32)>,
    hist: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        Rope {knots: vec![(0, 0); 10], hist: HashSet::from([(0, 0)])}
    }

    fn move_head(&mut self, dx: i32, dy: i32) {
        //Move each knot (excluing first, like its a tail), dx/dy times
        for _ in 0..dx.abs() {
            self.knots[0].0 += dx.signum();
            self.move_tail(1);
        }
        for _ in 0..dy.abs() {
            self.knots[0].1 += dy.signum();
            self.move_tail(1);
        }
    }

    fn move_tail(&mut self, knot_i: usize) {
        //If distance between head and tail is <= 1 for both x and y, don't need to move tail
        if (self.knots[knot_i-1].0 - self.knots[knot_i].0).abs() <= 1 && (self.knots[knot_i-1].1 - self.knots[knot_i].1).abs() <= 1 {
            return;
        }

        //Move in the direction of the head / last tail
        self.knots[knot_i].0 += (self.knots[knot_i-1].0 - self.knots[knot_i].0).signum();
        self.knots[knot_i].1 += (self.knots[knot_i-1].1 - self.knots[knot_i].1).signum();
        
        //Adds true tail (9) to history
        if knot_i == 9 {
            self.hist.insert(self.knots[knot_i]);
        }
        //Otherwise recurse to next knot
        else {
            self.move_tail(knot_i+1)
        }
    }
}


pub fn v2() {
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