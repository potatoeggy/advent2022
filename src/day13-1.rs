use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut ins: Vec<Vec<(usize, usize)>> = vec![];

    let mut maxx: usize = 0;
    let mut maxy: usize = 0;
    let mut minx: usize = 0;
    let mut miny: usize = 0;
    while stdin().read_line(&mut buf).unwrap() > 1 {
        let line: Vec<&str> = buf.trim().split("->").collect();
        let path: Vec<(usize, usize)> = line
            .iter()
            .map(|s| {
                let mut iter = s.trim().split(",");
                let x = iter.next().unwrap().parse::<usize>().unwrap();
                let y = iter.next().unwrap().parse::<usize>().unwrap();

                maxx = std::cmp::max(maxx, x);
                maxy = std::cmp::max(maxy, y);
                minx = std::cmp::min(minx, x);
                miny = std::cmp::min(miny, y);

                (x as usize, y as usize)
            })
            .collect();
        ins.push(path);
        buf.clear();
    }

    let mut grid = vec![vec![false; maxy + 1]; maxx + 1];

    for path in ins {
        let mut x = path[0].0;
        let mut y = path[0].1;
        for (nx, ny) in path {
            grid[nx][ny] = true;
            while x != nx {
                grid[x][y] = true;
                if x < nx {
                    x += 1;
                } else {
                    x -= 1;
                }
            }

            while y != ny {
                grid[x][y] = true;
                if y < ny {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    let mut count = 0;
    loop {
        let mut landed = false;

        let mut pos = (500, 0);

        while pos.1 <= maxy {
            if grid[pos.0][pos.1] {
                if pos.0 - 1 >= minx && !grid[pos.0 - 1][pos.1] {
                    pos.0 -= 1;
                    continue;
                }

                if pos.0 + 1 <= maxx && !grid[pos.0 + 1][pos.1] {
                    pos.0 += 1;
                    continue;
                }

                grid[pos.0][pos.1 - 1] = true;
                landed = true;
                count += 1;
                break;
            }
            pos.1 += 1;
        }

        if !landed {
            break;
        }
    }
    println!("{}", count);
}
