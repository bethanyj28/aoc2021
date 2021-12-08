use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut positions: Vec<i32> = Vec::new();
    let mut sum = 0;
    if let Ok(lines) = read_lines("./treachery_of_whales.txt") {
        for line in lines {
            if let Ok(crab_positions) = line {
                for pos in crab_positions.split(",") {
                    let pos_num = pos.parse::<i32>().unwrap();
                    positions.push(pos_num);
                    sum += pos_num;
                }
            }
        }
    }

    let avg_float: f32 = sum as f32 / positions.len() as f32;
    let avg = avg_float.round() as i32 - 1;

    positions.sort();
    let mut median: i32 = positions[positions.len() / 2];
    if positions.len() % 2 == 0 {
        let low: i32 = positions[(positions.len() / 2) - 1];
        let median_float: f32 = ((low + median) as f32) / 2.0_f32;
        median = median_float.round() as i32;
    }

    let mut p1_gas = 0;
    let mut p2_gas = 0;
    for pos in positions {
        let med_diff = median - pos;
        p1_gas += med_diff.abs();
        
        let avg_diff = avg - pos;
        for i in 0..avg_diff.abs() {
            p2_gas += i + 1;
        }
    }

    println!("part 1\nGas used: {}", p1_gas);
    println!("part 2\nGas used: {}", p2_gas);
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
