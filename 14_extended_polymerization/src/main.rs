use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pair_insertion = HashMap::new();
    let mut polymer = String::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(template_line) = line {
                if template_line == "" {
                    continue;
                }

                if template_line.contains("->") {
                    // store pair insertion rule
                    let parts: Vec<String> = template_line.split(" -> ").map(|p| p.to_string()).collect();
                    let pair = parts[0].clone();
                    let insert = parts[1].clone();
                    pair_insertion.insert(pair, insert);

                    continue;
                }

                // store polymer string
                polymer = template_line;
            }
        }
    }

    let mut polymer_count: HashMap<String, u128> = HashMap::new();
    for c in polymer.chars() {
        let count = polymer_count.entry(c.to_string()).or_insert(0);
        *count += 1;
    }

    /* part 1
    let steps = 10;
    for step in 0..steps {
        println!("{}", step);
        let mut prev = 0;
        let mut curr = 1;
        let polymer_arr: Vec<String> = polymer.chars().map(|p| p.to_string()).collect();
        let mut new_polymer = polymer_arr[0].clone();
        while curr != polymer.len() {
            if !pair_insertion.contains_key(&format!("{}{}", polymer_arr[prev], polymer_arr[curr])) {
                prev += 1;
                curr += 1;
                continue;
            }

            let insert = pair_insertion.get(&format!("{}{}", polymer_arr[prev], polymer_arr[curr])).unwrap();
            new_polymer.push_str(&format!("{}{}", insert, polymer_arr[curr]));
            prev = curr;
            curr += 1;
            let count = polymer_count.entry(insert.clone()).or_insert(0);
            *count += 1;
        }

        polymer = new_polymer;
    }

    let mut counts: Vec<i32> = polymer_count.into_values().collect();
    counts.sort();
    let min_count = counts[0];
    let max_count = counts[counts.len() - 1];
    */

    let mut pair_counts: HashMap<String, u128> = HashMap::new();
    let template: Vec<String> = polymer.chars().map(|p| p.to_string()).collect();
    let mut prev = 0;
    for curr in 1..template.len() {
        let count = pair_counts.entry(format!("{}{}", template[prev], template[curr])).or_insert(0);
        *count += 1;
        prev = curr;
    }

    let steps = 40;
    for _ in 0..steps {
        //println!("step {} pairs: {:?}", step+1, pair_counts);
        let mut new_pair_counts = HashMap::new();
        for (pair, count) in pair_counts.clone() {
            let polymers: Vec<String> = pair.chars().map(|p| p.to_string()).collect();
            let a = polymers[0].clone();
            let b = polymers[1].clone();
            if !pair_insertion.contains_key(&format!("{}{}", a, b)) {
                let pair_count = new_pair_counts.entry(pair).or_insert(0);
                *pair_count += count;
                continue;
            }

            let insert = pair_insertion.get(&format!("{}{}", a, b)).unwrap();
            let a_pair = new_pair_counts.entry(format!("{}{}", a, insert)).or_insert(0);
            *a_pair += count;
            let b_pair = new_pair_counts.entry(format!("{}{}", insert, b)).or_insert(0);
            *b_pair += count;
            let count_element = polymer_count.entry(insert.clone()).or_insert(0);
            *count_element += count;
        }
        pair_counts = new_pair_counts;
    }

    println!("counts: {:?}", polymer_count);

    let mut counts: Vec<u128> = polymer_count.into_values().collect();
    counts.sort();
    let min_count = counts[0];
    let max_count = counts[counts.len() - 1];
    println!("answer: {} - {} = {}", max_count, min_count, max_count - min_count);
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
