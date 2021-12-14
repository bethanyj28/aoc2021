use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut paper: HashMap<(u32, u32), bool> = HashMap::new();
    let mut fold_dir: Vec<(String, u32)> = Vec::new();
    if let Ok(lines) = read_lines("./transparent_origami.txt") {
        for line in lines {
            if let Ok(direction) = line {
                if direction == "" {
                    continue;
                }

                if direction.contains("fold along") {
                    let mut fold = direction.trim_start_matches("fold along ").split("=");
                    fold_dir.push((String::from(fold.next().unwrap()), fold.next().unwrap().parse::<u32>().unwrap()));
                    continue;
                }

                //point
                let coord: Vec<u32> = direction.split(",").map(|p| p.parse::<u32>().unwrap()).collect();
                paper.insert((coord[0], coord[1]), true);
            }
        }
    }

    //let num_folds = 1;
    //let mut i = 0;
    for (axis, line) in fold_dir.clone() {
        //i += 1;
        for (point, _) in paper.clone() {
            let (x, y) = point;
            if axis == "x" && x > line {
                paper.remove(&(x, y));
                let x_new = line - (x - line);
                paper.insert((x_new, y), true);
                continue
            }
            if axis == "y" && y > line {
                paper.remove(&(x, y));
                let y_new = line - (y - line);
                paper.insert((x, y_new), true);
            }
        }
        /*
        if i == num_folds {
            break;
        }
        */
    }

    //println!("{}", paper.len());
    println!("{:?}", paper.into_keys());
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
