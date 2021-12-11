use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut garden: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = read_lines("./dumbo_octopus.txt") {
        for line in lines {
            if let Ok(row) = line {
                let octopi: Vec<u32> = row.chars().map(|o| o.to_digit(10).unwrap()).collect();
                garden.push(octopi);
            }
        }
    }

    let mut s = 0;
    //let mut flashes = 0;
    let mut all_flashed = false;
    while !all_flashed {
        let (updated_garden, _, all_flash) = step(garden.clone());
        garden = updated_garden;
        //flashes += flash;
        all_flashed = all_flash;
        s += 1;
    }
    println!("all flashed at step: {}", s);

    //println!("flashes: {}", flashes);
}

fn step(mut garden: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, u64, bool) {
    // increment by 1
    let mut garden_map: HashMap<String, u32> = HashMap::new();
    for (x, row) in garden.clone().iter().enumerate() {
        for (y, octopus) in row.clone().iter().enumerate() {
            garden_map.insert(format!("{}:{}", x, y), *octopus);
        }
    }

    // find the flashes and increment neighbor
    for (x, row) in garden.clone().iter().enumerate() {
        for (y, _) in row.clone().iter().enumerate() {
            if *garden_map.get(&format!("{}:{}", x, y)).unwrap() < 10 {
                let octopus = garden_map.entry(format!("{}:{}", x, y)).or_insert(0);
                *octopus += 1;
                if *octopus == 10 {
                    let mut flashed: Vec<(usize, usize)> = Vec::new();
                    flashed.push((x, y));
                    while !flashed.is_empty() {
                        let (f_x, f_y) = flashed.pop().unwrap();
                        for n in find_neighbors(f_x, f_y, garden.clone().len()) {
                            let (n_x, n_y) = n;
                            let n_energy = garden_map.entry(format!("{}:{}", n_x, n_y)).or_insert(0);
                            if *n_energy < 10 {
                                *n_energy += 1;
                                if *n_energy == 10 {
                                    flashed.push((n_x, n_y))
                                }
                            }
                        } 
                    }
                }
            }
        }
    }

    // normalize and count flashes
    let mut flashes: u64 = 0;
    for (x, row) in garden.clone().iter().enumerate() {
        for (y, _) in row.clone().iter().enumerate() {
            let energy = garden_map.get(&format!("{}:{}", x, y)).unwrap();
            if *energy > 9 {
                flashes += 1;
            }
            garden[x][y] = energy % 10;
        }
    }

    let all_flashed = flashes == garden_map.len() as u64;
    return (garden, flashes, all_flashed);
}

fn find_neighbors(i: usize, j: usize, max: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    // bad naming - booleans for if i and j can go lower and higher
    let i_low = i > 0;
    let j_low = j > 0;
    let i_high = i < max - 1;
    let j_high = j < max - 1;

    if i_low {
        // down
        neighbors.push((i - 1, j));
    }
    if i_high {
        // up
        neighbors.push((i + 1, j));
    }
    if j_low {
        // left
        neighbors.push((i, j - 1));
    }
    if j_high {
        // right
        neighbors.push((i, j + 1));
    }
    if i_low && j_low {
        // down-left
        neighbors.push((i - 1, j - 1));
    }
    if i_low && j_high {
        // down-right
        neighbors.push((i - 1, j + 1));
    }
    if i_high && j_high {
        // up-right
        neighbors.push((i + 1, j + 1));
    }
    if i_high && j_low {
        // up-left
        neighbors.push((i + 1, j - 1));
    }

    return neighbors;
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
