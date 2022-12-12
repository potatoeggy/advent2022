use std::collections::VecDeque;

const WORRY_DEFLATE: i32 = 3;

// this is by far the shittest code i've ever written in rust
#[derive(Clone)]
struct Monkey<'a> {
    items: VecDeque<i32>,
    op: &'a dyn Fn(i32) -> i32,
    test: &'a dyn Fn(i32) -> bool,
    test_dest: (usize, usize),
    count: i32,
}

impl Monkey<'_> {
    pub fn operate_all(&mut self) {
        for i in self.items.iter_mut() {
            *i = (self.op)(*i) / WORRY_DEFLATE;
        }
    }

    pub fn test_and_move_all(&self, dest: &mut Vec<Monkey>) {
        for &item in self.items.iter() {
            if (self.test)(item) {
                dest[self.test_dest.0].items.push_back(item);
            } else {
                dest[self.test_dest.1].items.push_back(item);
            }
        }
    }

    pub fn clear_items(&mut self) {
        // always run after test_and_move_all
        // so that we follow rust's borrow guarantees
        // feels like a hack
        self.count += self.items.len() as i32;
        self.items.clear();
    }
}

fn main() {
    // we do a lil bit of hardcoding
    let mut monkeys = vec![
        Monkey {
            items: VecDeque::from(vec![98, 89, 52]),
            op: &|x| x * 2,
            test: &|x| x % 5 == 0,
            test_dest: (6, 1),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![57, 95, 80, 92, 57, 78]),
            op: &|x| x * 13,
            test: &|x| x % 2 == 0,
            test_dest: (2, 6),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![82, 74, 97, 75, 51, 92, 83]),
            op: &|x| x + 5,
            test: &|x| x % 19 == 0,
            test_dest: (7, 5),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![97, 88, 51, 68, 76]),
            op: &|x| x + 6,
            test: &|x| x % 7 == 0,
            test_dest: (0, 4),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![63]),
            op: &|x| x + 1,
            test: &|x| x % 17 == 0,
            test_dest: (0, 1),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![94, 91, 51, 63]),
            op: &|x| x + 4,
            test: &|x| x % 13 == 0,
            test_dest: (4, 3),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![61, 54, 94, 71, 74, 68, 98, 83]),
            op: &|x| x + 2,
            test: &|x| x % 3 == 0,
            test_dest: (2, 7),
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![90, 56]),
            op: &|x| x * x,
            test: &|x| x % 11 == 0,
            test_dest: (3, 5),
            count: 0,
        },
    ];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].operate_all();

            let monkey = monkeys[i].clone();
            monkey.test_and_move_all(&mut monkeys);
            monkeys[i].clear_items();
        }
    }
    monkeys.sort_by_key(|x| -x.count);
    println!("{}", monkeys[0].count * monkeys[1].count);
}
