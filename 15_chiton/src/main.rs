use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = read_lines("./chiton_simple.txt") {
        for line in lines {
            let mut row: Vec<u32> = Vec::new();
            if let Ok(row_str) = line {
                let points: Vec<String> = row_str.split("").map(|p| p.to_string()).collect();
                for point in points {
                    if point == "" {
                        continue;
                    }
                    row.push(point.parse::<u32>().unwrap());
                }
            }
            grid.push(row);
        }
    }

    let mut big_grid: Vec<Vec<u32>> = Vec::new();
    for s_r in 0..5 * grid.len() {
        big_grid.push(Vec::new());
        for s_c in 0..5 * grid[0].len(){
            if s_r < grid.len() && s_c < grid[0].len() {
                big_grid[s_r].push(grid[s_r][s_c]);
                continue;
            }
            let bg = big_grid.clone();
            if s_c >= grid[0].len() {
                big_grid[s_r].push((bg[s_r][s_c - grid[0].len()] + 1) % 10);
                continue;
            }
            big_grid[s_r].push((bg[s_r - grid.len()][s_c] + 1) % 10);
        }
    }

    println!("grid: {:?}", big_grid);
    println!("{}", find_path(0, 0, big_grid))
}

#[derive(Clone)]
struct Cell {
    g: u32,
    h: u32,
    f: u32,
    parent: (usize, usize),
    pos: (usize, usize),
}

fn build_start_cell(pos: (usize, usize)) -> Cell {
    Cell {
        g: 0,
        h: 0,
        f: 0,
        parent: pos,
        pos: pos,
    }
}

fn build_cell(pos: (usize, usize), parent: (usize, usize), g: u32, h: u32) -> Cell {
    Cell {
        g,
        h,
        f: g + h,
        parent,
        pos,
    }
}

fn find_path(init_i: usize, init_j: usize, grid: Vec<Vec<u32>>) -> u32 {
    let mut open_list: HashMap<String, Cell> = HashMap::new();
    let mut closed_list: HashMap<String, Cell> = HashMap::new();
    open_list.insert(
        format!("{}:{}", init_i, init_j),
        build_start_cell((init_i, init_j)),
    );

    while !open_list.is_empty() {
        let mut min_f_pos = String::from("");
        for (pos, c) in open_list.clone() {
            if min_f_pos == "" {
                min_f_pos = pos;
                continue;
            }
            if c.f < open_list.get(&min_f_pos).unwrap().f {
                min_f_pos = pos;
            }
        }

        let q = open_list.remove(&min_f_pos).unwrap();
        let (i, j) = q.pos;

        for (n_i, n_j) in find_neighbors(i, j, grid.len(), grid[0].len()) {
            if n_i == grid.len() - 1 && n_j == grid[0].len() - 1 {
                return q.g + grid[n_i][n_j];
            }

            let n_cell = build_cell((n_i, n_j), (i, j), q.g + grid[n_i][n_j], 0);
            if closed_list.contains_key(&format!("{}:{}", n_i, n_j)) {
                if closed_list.get(&format!("{}:{}", n_i, n_j)).unwrap().f < n_cell.f {
                    continue;
                }
            }

            if open_list.contains_key(&format!("{}:{}", n_i, n_j)) {
                if open_list.get(&format!("{}:{}", n_i, n_j)).unwrap().f < n_cell.f {
                    continue;
                }
            }

            open_list.insert(format!("{}:{}", n_i, n_j), n_cell);
        }
        closed_list.insert(format!("{}:{}", i, j), q);
    }

    return 0;
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
