use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // day 1
    sonar_sweep();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
