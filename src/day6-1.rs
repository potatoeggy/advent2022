use std::io::stdin;

fn no_dupes(slice: &str) -> bool {
    for (i, c) in slice.chars().enumerate() {
        for (j, c2) in slice.chars().enumerate() {
            if i == j {
                continue;
            }

            if c == c2 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    for i in 0..line.len() {
        if i < 4 {
            continue;
        }
        if no_dupes(&line[i - 4..i]) {
            println!("{}", i);
            break;
        }
    }
}
