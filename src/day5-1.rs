use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut buf = String::new();
    let mut grid: Vec<VecDeque<char>> = vec![];

    stdin().read_line(&mut buf).unwrap();

    while buf.trim() != "" {
        let changed = buf // i coulda just split at every 4th char and fixed calced it from there
            .trim_end_matches('\n')
            .replace("    ", "0")
            .replace('[', "")
            .replace(']', "")
            .replace(' ', "");

        if changed.parse::<u64>().is_ok() {
            // if input is done, break
            break;
        }
        let char_array: Vec<char> = changed.chars().collect();

        if grid.len() == 0 {
            // if first run
            for _ in 0..char_array.len() {
                grid.push(VecDeque::new());
            }
        }

        for (i, c) in char_array.iter().enumerate() {
            if *c != '0' {
                grid[i].push_back(*c);
            }
        }

        buf.clear();
        stdin().read_line(&mut buf).unwrap();
    }

    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let changed = buf.replace(char::is_alphabetic, "").replace("  ", " ");
        if changed.trim() == "" {
            continue;
        }

        let inputs = changed
            .trim()
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let nums_to_move = inputs.get(0).unwrap();
        let from = inputs.get(1).unwrap() - 1;
        let to = inputs.get(2).unwrap() - 1;

        for _ in 0..*nums_to_move {
            let c = grid[from as usize].pop_front().unwrap();
            grid[to as usize].push_front(c);
        }

        let tops = grid
            .iter()
            .map(|v| v.front().unwrap_or(&' '))
            .collect::<Vec<&char>>();
        println!("{}", tops.iter().cloned().collect::<String>());
    }
}
