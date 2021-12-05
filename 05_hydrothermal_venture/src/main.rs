use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = "./hydrothermal_venture.txt";
    println!("part 1: {}", find_danger_points(false, file_name));
    println!("part 2: {}", find_danger_points(true, file_name));
}

fn find_danger_points(track_diagonals: bool, file_name: &str) -> usize {
    let mut grid: HashMap<String, i32> = HashMap::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(segment) = line {
                let mut points = segment.split(" -> ");
                let start: Vec<i32> = points
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect();
                let end: Vec<i32> = points
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect();

                let (x1, y1, x2, y2) = (start[0], start[1], end[0], end[1]);
                let (mut x, mut y) = (x1, y1);
                if !track_diagonals && (x1 != x2 && y1 != y2) {
                    continue;
                }
                while x != x2 || y != y2 {
                    let count = grid.entry(format!("{}:{}", x, y)).or_insert(0);
                    *count += 1;

                    if x != x2 {
                        if x < x2 {
                            x += 1;
                        } else {
                            x -= 1;
                        }
                    }

                    if y != y2 {
                        if y < y2 {
                            y += 1;
                        } else {
                            y -= 1;
                        }
                    }
                }

                // inclusive of last numbers
                let count = grid.entry(format!("{}:{}", x, y)).or_insert(0);
                *count += 1;
            }
        }
    }
    let danger_points: Vec<i32> = grid.into_values().filter(|count| count >= &2).collect();
    return danger_points.len();
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
