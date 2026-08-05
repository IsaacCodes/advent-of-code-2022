use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Copy, Clone, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Action {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Action::Rock),
            'B' => Ok(Action::Paper),
            'C' => Ok(Action::Scissors),
            _ => Err("Invalid direction character"),
        }
    }
}

impl Action {
    fn wins_against(&self) -> Action {
        match self {
            Action::Rock => Action::Scissors,
            Action::Paper => Action::Rock,
            Action::Scissors => Action::Paper,
        }
    }

    fn loses_against(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissors,
            Action::Scissors => Action::Rock,
        }
    }

    fn rigs_with(&self, value: char) -> Result<Self, &'static str> {
        match value {
            'X' => Ok(self.wins_against()),
            'Y' => Ok(*self),
            'Z' => Ok(self.loses_against()),
            _ => Err("Invalid direction character"),
        }
    }

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
            (a, b) if a.wins_against() == *b => 6,
            //tie
            (a, b) if a == b => 3,
            //loss
            _ => 0,
        }
    }
}

pub fn v2() {
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
        let your_player: Action = opp_player
            .rigs_with(
                line.chars().nth(2).expect("Missing char")
            ).unwrap();

        total += your_player.shape_score() + your_player.outcome_score(&opp_player);
    }

    println!("{total}");
}