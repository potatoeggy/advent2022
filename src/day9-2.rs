use std::{collections::HashSet, io::stdin};

pub trait Snake {
    fn navigate_tail(&mut self, head: (i32, i32));
}

impl Snake for (i32, i32) {
    fn navigate_tail(&mut self, head: (i32, i32)) {
        if (head.0 - self.0).abs() <= 1 && (head.1 - self.1).abs() <= 1 {
            return;
        }
        self.0 += head.0.cmp(&self.0) as i32;
        self.1 += head.1.cmp(&self.1) as i32;
    }
}

fn main() {
    let mut buf = String::new();
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];
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

            for i in 0..tails.len() {
                let old_tail = *tails.get((i as i32 - 1) as usize).unwrap_or(&head);
                tails[i].navigate_tail(old_tail);
            }
            tail_history.insert(*tails.last().expect("Empty tail vector"));
        }
        buf.clear();
    }
    println!("{}", tail_history.len());
}
