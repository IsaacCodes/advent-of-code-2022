use std::{collections::VecDeque, fs};

pub fn solve(many_starts: bool) {
    //'S' and 'E' respectively
    let mut starts = Vec::new();
    let mut end = (0, 0);

    //Read file
    let text = fs::read_to_string("src/input.txt").unwrap();
    let mut grid = Vec::new();

    //Convert String to 2d Vec of u8
    for (y, line) in text.lines().enumerate() {
        grid.push(Vec::new());
        for (x, char) in line.chars().enumerate() {
            grid[y].push(
                if char == 'S' || many_starts && char == 'a' {
                    starts.push((y, x));
                    0 //'a'
                }
                else if char == 'E' {
                    end = (y, x);
                    25 //'z'
                }
                else {
                    char as u8 - b'a'
                },
            );
        }
    }

    //Distances 2d array + queue + distance (answer)
    let mut dists = vec![vec![u16::MAX; grid[0].len()]; grid.len()];
    for (y, x) in &starts {
        dists[*y][*x] = 0;
    }
    let mut q = VecDeque::from(starts);

    //BFS
    loop {
        let (y, x) = q.pop_front().expect("No Solution (queue empty)");
        if (y, x) == end {
            break;
        }

        q.extend(traverse(&grid, &mut dists, y, x));
    }

    println!("{}", dists[end.0][end.1]);
}

//Search surrounding squares
fn traverse(
    grid: &Vec<Vec<u8>>,
    dists: &mut Vec<Vec<u16>>,
    y: usize,
    x: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    //Search neighbors
    for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        //Check that ny, nx is in range
        let Some(ny) = y.checked_add_signed(dy)
        else {
            continue;
        };
        let Some(nx) = x.checked_add_signed(dx)
        else {
            continue;
        };
        if !(ny < grid.len() && nx < grid[ny].len()) {
            continue;
        }

        //If not visited + <= 1 higher
        if dists[ny][nx] == u16::MAX && grid[ny][nx] <= grid[y][x] + 1 {
            //Add to neighbors + visited
            neighbors.push((ny, nx));
            dists[ny][nx] = dists[y][x] + 1;
        }
    }

    neighbors
}
