use std::{collections::HashMap, io::stdin};

fn main() {
    let mut s = String::new();
    let map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let mut total = 0;

    loop {
        stdin().read_line(&mut s).unwrap();
        let (opp, you) = s.trim().split_once(" ").unwrap();

        total += map.get(you).unwrap();

        total += match opp {
            "A" => match you {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => panic!(),
            },
            "B" => match you {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!(),
            },
            "C" => match you {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => panic!(),
            },
            _ => panic!(),
        };

        s.clear();
        println!("{}", total);
    }
}
