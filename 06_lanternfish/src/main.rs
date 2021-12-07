use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut school: Vec<i32> = Vec::new();
    let days = 80;
    if let Ok(lines) = read_lines("./lanternfish_simple.txt") {
        for line in lines {
            if let Ok(initial_fish) = line {
                for timer in initial_fish.split(",") {
                    school.push(timer.parse::<i32>().unwrap());
                }
            }
        }
    }

    for _ in 0..days {
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

    println!("part 1\nSchool size: {}", school.len());

    let mut timers: HashMap<i32, u128> = HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)]);
    if let Ok(lines) = read_lines("./lanternfish.txt") {
        for line in lines {
            if let Ok(initial_fish) = line {
                for timer in initial_fish.split(",") {
                    let count = timers.entry(timer.parse::<i32>().unwrap()).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    let total_days: u32 = 256;
    for _ in 0..total_days {
        let baby_count = timers.entry(8).or_insert(0);
        let mut fish_to_move = baby_count.clone();
        *baby_count = 0;
        for i in (0..8).rev() {
            if i == 0 {
                let count = timers.entry(i).or_insert(0);
                let num_to_generate = count.clone();
                *count = fish_to_move;
                let restart_fish = timers.entry(6).or_insert(0);
                *restart_fish += num_to_generate;
                let baby_fish = timers.entry(8).or_insert(0);
                *baby_fish += num_to_generate;
                continue;
            }
            let to_add = fish_to_move;
            let count = timers.entry(i).or_insert(0);
            fish_to_move = count.clone();
            *count = to_add;
        }
    }

    let fish_total: Vec<u128> = timers.into_values().collect();
    let sum: u128 = fish_total.iter().sum();
    println!("part 2\nSchool size: {}", sum);}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
