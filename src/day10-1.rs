use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut strengths: Vec<i32> = vec![];

    while stdin().read_line(&mut buf).unwrap() > 1 {
        match buf.trim().split_once(' ').unwrap_or(("noop", "")) {
            ("noop", "") => strengths.push(0),
            ("addx", num) => strengths.append(&mut vec![0, num.parse::<i32>().unwrap()]),
            _ => unreachable!(),
        };
        buf.clear();
    }

    // optimisation? screw optimisation
    // we get tiny code
    println!(
        "{}",
        [20, 60, 100, 140, 180, 220]
            .map(|i| { i as i32 * (1 + &strengths[..i - 1].iter().sum::<i32>()) })
            .iter()
            .sum::<i32>()
    );
}
