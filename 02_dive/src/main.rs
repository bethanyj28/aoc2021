use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    dive();
}

fn dive() {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
