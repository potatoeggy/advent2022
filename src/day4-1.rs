use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut dupes = 0;

    loop {
        stdin().read_line(&mut buf).unwrap();

        let (first, second) = buf.trim().split_once(',').unwrap();
        let (startf, endf) = first
            .split_once('-')
            .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
            .unwrap();
        let (starts, ends) = second
            .split_once('-')
            .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
            .unwrap();

        if startf >= starts && endf <= ends || starts >= startf && ends <= endf {
            dupes += 1;
        }
        println!("Dupes: {}", dupes);
        buf.clear();
    }
}
