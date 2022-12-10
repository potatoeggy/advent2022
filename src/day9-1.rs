use std::{collections::HashSet, io::stdin};

fn main() {
    let mut buf = String::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);
    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();

    while stdin().read_line(&mut buf).unwrap() > 1 {
        let dir = buf.chars().next().expect("Invalid direction");
        let len: usize = buf[1..].trim().parse().expect("Invalid length");

        for _ in 0..len {
            match dir {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                _ => unreachable!(),
            }

            if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
                continue;
            }
            tail.0 += head.0.cmp(&tail.0) as i32;
            tail.1 += head.1.cmp(&tail.1) as i32;

            tail_history.insert(tail);
        }
        buf.clear();
    }
    println!("{}", tail_history.len());
}
