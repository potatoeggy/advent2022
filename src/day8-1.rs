fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut buf = String::new();

    while let Ok(n) = std::io::stdin().read_line(&mut buf) {
        if n == 1 {
            break;
        }
        grid.push(buf.chars().collect());
        buf.clear();
    }

    let mut vis_grid = vec![vec![false; grid[0].len()]; grid.len()];

    for (row, vis_row) in grid.iter().zip(vis_grid.iter_mut()) {
        let mut highest = '.';
        for (tree, vis) in row.iter().zip(vis_row.iter_mut()) {
            // char comparison should work bc it's 0-9 in ascii
            if *tree > highest {
                highest = *tree;
                *vis = true;
                // we could do the summing now without iterating later
                // but it's simpler this way
            }
        }

        highest = '.';
        for (tree, vis) in row.iter().zip(vis_row.iter_mut()).rev() {
            if *tree > highest {
                highest = *tree;
                *vis = true;
            }
        }
    }

    for i in 0..grid[0].len() {
        let mut highest = '.';
        for j in 0..grid.len() {
            if grid[j][i] > highest {
                highest = grid[j][i];
                vis_grid[j][i] = true;
            }
        }

        highest = '.';
        for j in (0..grid.len()).rev() {
            if grid[j][i] > highest {
                highest = grid[j][i];
                vis_grid[j][i] = true;
            }
        }
    }

    let count = vis_grid.iter().flatten().filter(|&&x| x).count();
    println!("{}", count);
}
