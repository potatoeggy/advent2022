use std::{collections::HashSet, io::stdin};

struct Instruction {
    dir: char,
    len: usize,
}

pub trait Snake {
    fn navigate_tail(&mut self, head: (i32, i32));
}

impl Snake for (i32, i32) {
    fn navigate_tail(&mut self, head: (i32, i32)) {
        let new_pos = navigate_tail(head, *self);
        *self = new_pos;
    }
}

fn navigate_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // return the new tail position
    // if we don't have to move, don't move
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return tail;
    }

    // if we *do* have to move, move optimally
    let newx = if head.0 > tail.0 {
        tail.0 + 1
    } else if head.0 < tail.0 {
        tail.0 - 1
    } else {
        tail.0
    };
    let newy = if head.1 > tail.1 {
        tail.1 + 1
    } else if head.1 < tail.1 {
        tail.1 - 1
    } else {
        tail.1
    };

    (newx, newy)
}

fn main() {
    let mut buf = String::new();

    // two things to handle:
    // the rope can move in any direction with any length,
    // so we can't statically allocate a 2d array
    // we *can* precalculate it though and set the initial position

    let mut ins: Vec<Instruction> = vec![];

    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "" {
            break;
        }

        let dir = buf.chars().next().unwrap();
        let len: usize = buf[1..].trim().parse().unwrap();
        ins.push(Instruction { dir, len });
    }

    // actually, we might not need a grid

    let mut max_pos = (0, 0);
    let mut min_pos = (0, 0);

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    // we're just gonna be *real* *real* lazy
    // and do some copy pasting
    let mut tail_pos2 = (0, 0);
    let mut tail_pos3 = (0, 0);
    let mut tail_pos4 = (0, 0);
    let mut tail_pos5 = (0, 0);
    let mut tail_pos6 = (0, 0);
    let mut tail_pos7 = (0, 0);
    let mut tail_pos8 = (0, 0);
    let mut tail_pos9 = (0, 0);

    let mut tail_history: HashSet<(i32, i32)> = HashSet::new();

    for Instruction { dir, len } in ins {
        for _ in 0..len {
            match dir {
                'U' => head_pos.1 += 1,
                'D' => head_pos.1 -= 1,
                'L' => head_pos.0 -= 1,
                'R' => head_pos.0 += 1,
                _ => unreachable!(),
            }

            tail_pos.navigate_tail(head_pos);
            tail_pos2.navigate_tail(tail_pos);
            tail_pos3.navigate_tail(tail_pos2);
            tail_pos4.navigate_tail(tail_pos3);
            tail_pos5.navigate_tail(tail_pos4);
            tail_pos6.navigate_tail(tail_pos5);
            tail_pos7.navigate_tail(tail_pos6);
            tail_pos8.navigate_tail(tail_pos7);
            tail_pos9.navigate_tail(tail_pos8);

            tail_history.insert(tail_pos9);

            // surely there's a nicer way to do this
            max_pos.0 = max_pos.0.max(tail_pos9.0);
            max_pos.1 = max_pos.1.max(tail_pos9.1);
            min_pos.0 = min_pos.0.min(tail_pos9.0);
            min_pos.1 = min_pos.1.min(tail_pos9.1);
        }
    }
    println!("{}", tail_history.len());
}
