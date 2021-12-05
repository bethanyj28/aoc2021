use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("part 1");
    /*
    let mut grid: HashMap<String, i32> = HashMap::new();
    if let Ok(lines) = read_lines("./hydrothermal_venture.txt") {
        for line in lines {
            if let Ok(segment) = line {
                let mut points = segment.split(" -> ");
                let point_1: Vec<i32> = points.next().unwrap().split(",").map(|p| p.parse::<i32>().unwrap()).collect();
                let point_2: Vec<i32> = points.next().unwrap().split(",").map(|p| p.parse::<i32>().unwrap()).collect();
                if point_1[1] != point_2[1] && point_1[0] != point_2[0] {
                    continue;
                }
                let mut start = &point_1;
                let mut end = &point_2;
                if point_1[1] == point_2[1] {
                    if point_1[0] > point_2[0] {
                        start = &point_2;
                        end = &point_1
                    }
                } else {
                    if point_1[1] > point_2[1] {
                        start = &point_2;
                        end = &point_1;
                    }
                }
                //println!("segment {:?} -> {:?}", start, end);
                for y in start[1]..=end[1] {
                    for x in start[0]..=end[0] {
                        //println!("point x = {}, y = {}", x, y);
                        let count = grid.entry(format!("{}:{}", x, y)).or_insert(0);
                        *count += 1;
                    }
                }
            }
        }
    }

    //println!("Grid: {:?}", grid);

    let danger_points: Vec<i32> = grid.into_values().filter(|count| count >= &2).collect();
    println!("Danger points count: {}", danger_points.len());
    */
    println!("part 2");
    let mut grid: HashMap<String, i32> = HashMap::new();
    if let Ok(lines) = read_lines("./hydrothermal_venture.txt") {
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

                //println!("segment {:?} -> {:?}", start, end);
                let (x1, y1, x2, y2) = (start[0], start[1], end[0], end[1]);
                let (mut x, mut y) = (x1, y1);
                while x != x2 || y != y2 {
                    //println!("x = {}, y = {}", x, y);
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

    //println!("Grid: {:?}", grid);

    let danger_points: Vec<i32> = grid.into_values().filter(|count| count >= &2).collect();
    println!("Danger points count: {}", danger_points.len());
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
