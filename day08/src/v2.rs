use std::{fs::File, io::{BufRead, BufReader}};

pub fn v2() {
    let file = File::open("src/input.txt").unwrap();
    //Read into 2D Vec of 0-9 u8's
    let grid: Vec<Vec<u8>> = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(
            |line| line
            .chars()
            .map(|char| char as u8 - '0' as u8)
            .collect()
        )
        .collect();

    //Track tree scores
    let (w, h) = (grid[0].len(), grid.len());
    let mut scores = vec![vec![0_u32; w]; h];

    //Visit each tree
    for y in 0..h {
        for x in 0..w {            
            //Loop left side from right to left
            let mut left_score = 0;
            for other_x in (0..x).rev() {
                left_score += 1;
                //Break on taller tree
                if grid[y][other_x] >= grid[y][x] {
                    break;
                }
            }

            //Loop right side from left to right
            let mut right_score = 0;
            for other_x in x+1..w {
                right_score += 1;
                //Break on taller tree
                if grid[y][other_x] >= grid[y][x] {
                    break;
                }
            }

            //Loop top side from bottom to top
            let mut top_score = 0;
            for other_y in (0..y).rev() {
                top_score += 1;
                //Break on taller tree
                if grid[other_y][x] >= grid[y][x] {
                    break;
                }
            }

            //Loop bottom side from top to bottom
            let mut bottom_score = 0;
            for other_y in y+1..h {
                bottom_score += 1;
                //Break on taller tree
                if grid[other_y][x] >= grid[y][x] {
                    break;
                }
            }

            //Set combined score
            scores[y][x] = left_score * right_score * top_score * bottom_score;
        }
    }

    //Sums for # of seen trees
    let res: u32 = scores.into_iter()
        .map(
            |row| row.into_iter()
                .max().unwrap()
        )
        .max().unwrap();

    println!("{res}");
}