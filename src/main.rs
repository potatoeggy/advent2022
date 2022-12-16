use std::io::stdin;

const BOUNDS: (i32, i32) = (0, 10000 + 1);

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

    let mut max = (0, 0);
    let mut min = (0, 0);
    while stdin().read_line(&mut buf).unwrap() > 1 {
        let process = buf
            .trim()
            .split(|f: char| f.is_ascii_punctuation() && !(f == '-'))
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let sen = Sensor::new(process[0], process[1], process[2], process[3]);

        max.0 = std::cmp::max(max.0, sen.pos.0 + sen.beacon_distance);
        max.1 = std::cmp::max(max.1, sen.pos.1 + sen.beacon_distance);
        min.0 = std::cmp::min(min.0, sen.pos.0 - sen.beacon_distance);
        min.1 = std::cmp::min(min.1, sen.pos.1 - sen.beacon_distance);

        sensors.push(sen);
        buf.clear();
    }

    for dest in BOUNDS.0..BOUNDS.1 {
        if dest % 1000 == 0 {
            println!("{} / {}", dest, BOUNDS.1);
        }
        let mut row = vec!['.'; (max.0 - min.0 + 1) as usize];
        for sen in &sensors {
            let x = sen.pos.0 - min.0;
            let y = sen.pos.1 - min.1;
            let d = sen.beacon_distance;
            let dest_rel = dest - min.1;

            if y + d >= dest_rel && y - d <= dest_rel {
                for i in 0..d - (dest_rel - y).abs() + 1 {
                    row[(x + i) as usize] = '#';
                    row[(x - i) as usize] = '#';
                }
            }
        }

        for sen in &sensors {
            let x = sen.pos.0 - min.0;
            let y = sen.pos.1 - min.1;

            let dest_rel = dest - min.1;
            if y == dest_rel {
                row[x as usize] = 'S';
            }

            if sen.beacon.1 == dest {
                row[(sen.beacon.0 - min.0) as usize] = 'B';
            }
        }

        if let Some(i) = row[(BOUNDS.0 - min.0) as usize..(BOUNDS.1 - min.0) as usize]
            .iter()
            .position(|&c| c == '.')
        {
            let freq = i as i32 * 4000000 + dest;
            println!("{}", freq);
        }
    }
}
