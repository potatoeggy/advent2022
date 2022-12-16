use std::io::stdin;

const BOUNDS: (i32, i32) = (0, 4000000 + 1);

struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    beacon_distance: i32,
}

impl Sensor {
    fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Sensor {
        Sensor {
            pos: (sx, sy),
            beacon: (bx, by),
            beacon_distance: (sx - bx).abs() + (sy - by).abs(),
        }
    }
}

fn main() {
    let mut buf = String::new();
    let mut sensors: Vec<Sensor> = vec![];

    while stdin().read_line(&mut buf).unwrap() > 1 {
        let process = buf
            .trim()
            .split(|f: char| f.is_ascii_punctuation() && !(f == '-'))
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let sen = Sensor::new(process[0], process[1], process[2], process[3]);

        sensors.push(sen);
        buf.clear();
    }

    for dest in BOUNDS.0..BOUNDS.1 {
        let mut ranges: Vec<(i32, i32)> = vec![];
        for sen in &sensors {
            let (x, y) = sen.pos;
            let dest_rel = (dest - x).abs();

            if dest_rel <= sen.beacon_distance {
                let remainder = (sen.beacon_distance - dest_rel).abs();
                ranges.push((y - remainder, y + remainder));
            }
        }

        ranges.sort();
        let mut highest = 0;
        if ranges[0].0 > BOUNDS.0 {
            let freq = dest * 4000000 + (ranges[0].0 - 1);
            println!("{}", freq);
            return;
        }

        for &(start, end) in ranges.iter() {
            if start > highest {
                println!("{} {}", start, end);
                let freq: i64 = dest as i64 * 4000000 + (start as i64 - 1);
                println!("{}", freq);
                return;
            }
            highest = std::cmp::max(highest, end + 1);
        }

        if highest < BOUNDS.1 {
            let freq = dest * 4000000 + highest;
            println!("{}", freq);
            return;
        }
    }
}
