use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("./smoke_basin.txt") {
        for line in lines {
            let mut row: Vec<i32> = Vec::new();
            if let Ok(row_str) = line {
                let points: Vec<String> = row_str.split("").map(|p| p.to_string()).collect();
                for point in points {
                    if point == "" {
                        continue;
                    }
                    row.push(point.parse::<i32>().unwrap());
                }
            }
            grid.push(row);
        }
    }

    let mut low_points = 0;
    let mut basin_sizes: Vec<i32> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut points_higher = 0;
            let mut max_points = 0;
            let mut neighbors: Vec<i32> = Vec::new();
            let curr_point = grid[i][j];
            if i != 0 {
                neighbors.push(grid[i - 1][j]);
            }
            if i != grid.len() - 1 {
                neighbors.push(grid[i + 1][j]);
            }
            if j != 0 {
                neighbors.push(grid[i][j - 1]);
            }
            if j != grid[i].len() - 1 {
                neighbors.push(grid[i][j + 1]);
            }
            for neighbor in neighbors {
                if neighbor > -1 {
                    max_points += 1;
                }
                if neighbor > curr_point {
                    points_higher += 1;
                }
            }

            if points_higher == max_points {
                low_points += 1 + curr_point;
                basin_sizes.push(crawl_basin(i, j, grid.clone()));
            }
        }
    }

    basin_sizes.sort();
    let mut product = 1;
    for _ in 0..3 {
        product *= basin_sizes.pop().unwrap();
    }
    println!("{}", low_points);
    println!("{}", product);
}

fn crawl_basin(init_i: usize, init_j: usize, grid: Vec<Vec<i32>>) -> i32 { // essentially bfs
    let mut to_explore: Vec<(usize, usize, i32)> = Vec::new();
    let mut explored = HashMap::new();
    explored.insert(format!("{}:{}", init_i, init_j), grid[init_i][init_j]);
    to_explore.push((init_i, init_j, grid[init_i][init_j]));
    while !to_explore.is_empty() {
        let p = to_explore.remove(0);
        let (i, j, val) = p;
        for (n_i, n_j) in find_neighbors(i, j, grid.len(), grid[i].len()) {
            let n_val = grid[n_i][n_j];
            if n_val > val && n_val != 9 {
                if !explored.contains_key(&format!("{}:{}", n_i, n_j)) {
                    explored.insert(format!("{}:{}", n_i, n_j), n_val);
                    to_explore.push((n_i, n_j, n_val));
                }
            }
        }
    }
    return explored.len() as i32;
}

fn find_neighbors(i: usize, j: usize, i_max: usize, j_max: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if i < i_max - 1 {
        neighbors.push((i + 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if j < j_max - 1 {
        neighbors.push((i, j + 1));
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
