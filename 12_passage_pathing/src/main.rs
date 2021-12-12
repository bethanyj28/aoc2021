use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cave: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(lines) = read_lines("./passage_pathing.txt") {
        for line in lines {
            if let Ok(map_line) = line {
                let parts: Vec<String> = map_line.split("-").map(|p| p.to_string()).collect();
                let from = parts[0].clone();
                let to = parts[1].clone();
                let from_entry = cave.entry(from.clone()).or_insert(Vec::new());
                from_entry.push(to.clone());
                let to_entry = cave.entry(to).or_insert(Vec::new());
                to_entry.push(from);
            }
        }
    }

    let mut to_explore: Vec<Vec<String>> = Vec::new();
    let mut found_paths: Vec<Vec<String>> = Vec::new();
    let connected = cave.get("start").unwrap();
    for c in connected.clone() {
        to_explore.push(vec![String::from("start"), String::from(c)]);
    }
    while !to_explore.is_empty() {
        let p = to_explore.remove(0);
        let last = p.last().unwrap();
        let connected = cave.get(last).unwrap();
        for c in connected.clone() {
            if c == "end" {
                let mut final_path = p.clone();
                final_path.push(String::from(c));
                found_paths.push(final_path);
                continue;
            }
            // part 1 logic
            /*
            if c.to_lowercase() != *c || !p.contains(&c) {
                let mut new_path = p.clone();
                new_path.push(String::from(c));
                to_explore.push(new_path);
            }
            */
            //part 2 logic
            if can_visit(String::from(c.clone()), p.clone()) {
                let mut new_path = p.clone();
                new_path.push(String::from(c));
                to_explore.push(new_path);
            }
        }
    }

    println!("{:?}", cave);
    println!("{}", found_paths.len());
}

fn can_visit(c: String, p: Vec<String>) -> bool {
    if c.to_lowercase() != c { // if large cave, we can visit as much as we want
        return true;
    }

    if c == "start" { // can only visit start and end once, end already accounted for
        return false;
    }

    if !p.contains(&c) { // if we havent' visited this cave, then no problemo
        return true;
    }

    // now the tricky part - we can only revisit one small cave twice
    let mut counts = HashMap::new();
    for i in 0..p.len() {
        let cave = p[i].clone();
        if cave.to_lowercase() != cave {
            continue;
        }
        let cave_count = counts.entry(cave).or_insert(0);
        *cave_count += 1;
    }

    for (_, count) in &counts {
        if *count == 2 {
            return false;
        }
    }

    return true;

    
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
