use std::io::stdin;

fn main() {
    let mut total_priority = 0;
    let mut s = String::new();
    loop {
        stdin().read_line(&mut s).unwrap();
        let (first, second) = s.split_at(s.len() / 2);

        for c in first.chars() {
            if second.contains(c) {
                total_priority += match c {
                    c if c.is_lowercase() => c as u32 - 96, // a starts at 97
                    c if c.is_uppercase() => c as u32 - 64 + 26,
                    _ => panic!(),
                };
                break;
            }
        }
        println!("Prio: {}", total_priority);
        s.clear();
    }
}
