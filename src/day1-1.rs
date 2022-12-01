use std::io::stdin;

fn main() {
    let mut biggest_elf = 0;
    let mut s = String::new();

    loop {
        let mut total = 0;

        stdin().read_line(&mut s).unwrap();
        while s.trim() != "" {
            total += s.trim().parse::<i32>().unwrap();
            s.clear();
            stdin().read_line(&mut s).unwrap();
        }
        biggest_elf = std::cmp::max(biggest_elf, total);
        println!("Biggest: {}", biggest_elf);
    }
}
