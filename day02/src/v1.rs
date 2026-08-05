use std::{fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Action {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Action::Rock),
            'B' | 'Y' => Ok(Action::Paper),
            'C' | 'Z' => Ok(Action::Scissors),
            _ => Err("Invalid direction character"),
        }
    }
}

impl Action {
    fn shape_score(&self) -> i32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn outcome_score(&self, opponent: &Action) -> i32 {
        match (self, opponent) {
            //win
            (Action::Rock, Action::Scissors)
            | (Action::Paper, Action::Rock)
            | (Action::Scissors, Action::Paper) => 6,
            //tie
            (a, b) if a == b => 3,
            //loss
            _ => 0,
        }
    }
}

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
        //Extracts each player's action
        let opp_player: Action = line
            .chars().nth(0).expect("Missing char")
            .try_into().unwrap();
        let your_player: Action = line
            .chars().nth(2).expect("Missing char")
            .try_into().unwrap();

        total += your_player.shape_score() + your_player.outcome_score(&opp_player);
    }

    println!("{total}");
}