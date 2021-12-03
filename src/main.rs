use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // day 1
    sonar_sweep();
    // day 2
    dive();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn dive() {
    println!("Day 2");
    if let Ok(lines) = read_lines("./dive.txt") {
        println!("part 1");
        let mut h = 0;
        let mut d = 0;
        for line in lines {
            if let Ok(command) = line {
                let mut parts = command.split_whitespace();
                let dir: String = parts.next().unwrap().into();
                let units: i64 = parts.next().unwrap().parse::<i64>().unwrap();
                match dir.as_str() {
                    "forward" => h += units,
                    "down" => d += units,
                    "up" => d -= units,
                    _ => unreachable!()
                }
            }
        }
        println!("{} x {} = {}", h, d, h * d)
    }
    if let Ok(lines) = read_lines("./dive.txt") {
        println!("part 2");
        let mut h = 0;
        let mut d = 0;
        let mut a = 0;
        for line in lines {
            if let Ok(command) = line {
                let mut parts = command.split_whitespace();
                let dir: String = parts.next().unwrap().into();
                let units: i64 = parts.next().unwrap().parse::<i64>().unwrap();
                match dir.as_str() {
                    "forward" => {
                        h += units;
                        d += a*units;
                    },
                    "down" => a += units,
                    "up" => a -= units,
                    _ => unreachable!()
                }
            }
        }
        println!("{} x {} = {}", h, d, h * d)
    }
}

fn sonar_sweep() {
    println!("Day 1");
    if let Ok(lines) = read_lines("./sonar_sweep.txt") {
        println!("part 1");
        let mut prev = -1;
        let mut count_incr = 0;
        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i64>().unwrap();
                if prev >= 0 && depth > prev {
                    count_incr += 1;
                }

                prev = depth;
            }
        }

        println!("{}", count_incr);
    }

    if let Ok(lines) = read_lines("./sonar_sweep.txt") {
        println!("part 2");
        let mut window = Vec::new();
        let mut prev = -1;
        let mut count_incr = 0;
        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse::<i64>().unwrap();
                if window.len() == 3 {
                    window.remove(0);
                    window.push(depth);
                    let curr = window.iter().sum();
                    if prev > 0 && curr > prev {
                        count_incr += 1;
                    }
                    prev = curr
                } else {
                    window.push(depth);
                    if window.len() == 3 {
                        prev = window.iter().sum();
                    }
                }
            }
        }
        println!("{}", count_incr);
    }
}
