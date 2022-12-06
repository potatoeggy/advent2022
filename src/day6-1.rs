use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    for i in 4..line.len() {
        if HashSet::<char>::from_iter(line[i - 4..i]).len() == 4 {
            println!("{}", i);
            break;
        }
    }
}
