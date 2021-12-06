use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut school: Vec<i32> = Vec::new();
    let days = 256;
    if let Ok(lines) = read_lines("./lanternfish_simple.txt") {
        for line in lines {
            if let Ok(initial_fish) = line {
                for timer in initial_fish.split(",") {
                    school.push(timer.parse::<i32>().unwrap());
                }
            }
        }
    }

    for day in 0..days {
        println!("day {}", day);
        let mut new_fish = 0;
        for i in 0..school.len() {
            if school[i] == 0 {
                new_fish += 1;
                school[i] = 6;
                continue;
            }
            school[i] -= 1;
        }

        for _ in 0..new_fish {
            school.push(8);
        }
    }

    println!("School size: {}", school.len());
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
