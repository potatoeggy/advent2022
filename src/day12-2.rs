use std::{collections::VecDeque, io::stdin};

fn main() {
    // input
    let mut buf = String::new();
    let mut grid: Vec<Vec<char>> = vec![];

    while stdin().read_line(&mut buf).unwrap() > 1 {
        grid.push(buf.trim().chars().collect());
        buf.clear();
    }

    // find start pos
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|x| *x == 'E') {
            end = (i, j);
        }
    }

    grid[end.0][end.1] = 'z';

    // bfs baby
    let mut vis: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    vis[end.0][end.1] = true;

    let mut queue = VecDeque::from(vec![(end, 0)]);
    while !queue.is_empty() {
        let ((i, j), steps) = queue.pop_front().unwrap();
        println!("{} {} {}", i, j, steps);
        if grid[i][j] == 'a' {
            println!("{}", steps);
            break;
        }

        for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ni, nj) = (i as i32 + dx, j as i32 + dy);
            if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 {
                // bounds check
                continue;
            }

            let (ni, nj) = (ni as usize, nj as usize);
            if vis[ni][nj] || (grid[ni][nj] as u8) < (grid[i][j] as u8 - 1) {
                // vis and allowed check
                continue;
            }

            vis[ni][nj] = true;
            queue.push_back(((ni, nj), steps + 1));
        }
    }
}
