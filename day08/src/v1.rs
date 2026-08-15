use std::{fs::File, io::{BufRead, BufReader}};

fn mark_visible<I>(grid: &Vec<Vec<i8>>, seen: &mut Vec<Vec<bool>>, positions: I)
where
    I: IntoIterator<Item = (usize, usize)>,
{
    let mut tallest = -1;
    for (x, y) in positions {
        let height = grid[y][x];
        if height > tallest {
            seen[y][x] = true;
            tallest = height;
        }
    }
}

pub fn v1() {
    let file = File::open("src/input.txt").unwrap();
    //Read into 2D Vec of 0-9 i8's
    let grid: Vec<Vec<i8>> = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(
            |line| line
            .chars()
            .map(|char| char as i8 - '0' as i8)
            .collect()
        )
        .collect();

    //Track seen trees (false for unseen)
    let (w, h) = (grid[0].len(), grid.len());
    let mut seen = vec![vec![false; w]; h];

    for y in 0..h {
        let iter = (0..w).map(|x| (x, y));
        mark_visible(&grid, &mut seen, iter.clone());
        mark_visible(&grid, &mut seen, iter.rev());
    }

    for x in 0..w {
        let iter = (0..h).map(|y| (x, y));
        mark_visible(&grid, &mut seen, iter.clone());
        mark_visible(&grid, &mut seen, iter.rev());
    }

    //Sums for # of seen trees
    let res: u32 = seen.into_iter()
        .map(
            |row| row.into_iter()
                .map(|x| x as u32).sum::<u32>()
        )
        .sum();

    println!("{res}");
}