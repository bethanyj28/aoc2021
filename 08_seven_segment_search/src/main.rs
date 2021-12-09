use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Entry {
    signal_pattern: Vec<String>,
    output: Vec<String>,
}

fn chars_in(a: String, b: String) -> i32 {
    let mut count = 0;
    for c in a.chars() {
        if b.contains(c) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let mut entries: Vec<Entry> = Vec::new();
    if let Ok(lines) = read_lines("./seven_segment_search.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let mut parts = entry.split("|");
                let signal_pattern: Vec<String> = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                let output: Vec<String> = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                entries.push(Entry {
                    signal_pattern: signal_pattern,
                    output: output,
                })
            }
        }
    }

    let mut sum_subset = 0;
    let mut sum_vals = 0;
    for entry in entries {
        let mut code = HashMap::new();
        let mut reference = HashMap::new();
        let mut sorted_patterns: Vec<String> = entry.signal_pattern.clone().into_iter().map(|p| {
            let mut chars: Vec<char> = p.chars().collect();
            chars.sort_unstable();
            return chars.into_iter().collect::<String>();
        }).collect();
        sorted_patterns.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()));
        for pattern in sorted_patterns.clone() {
            let mut val = 0;
            match pattern.chars().count() {
                2 => val = 1,
                4 => val = 4,
                3 => val = 7,
                7 => val = 8,
                _ => val = 0,
            };
            code.insert(pattern.clone(), val);
            if val > 0 {
                reference.insert(val, pattern.clone());
            }
        }

        for pattern in sorted_patterns.clone() {
            match pattern.chars().count() {
                6 => {
                    let similar_chars = chars_in(pattern.clone(), reference.get(&1).unwrap().clone());
                    if  similar_chars == 2 {
                        let similar_chars_four = chars_in(pattern.clone(), reference.get(&4).unwrap().clone());
                        if similar_chars_four == 4 {
                            code.insert(pattern, 9);
                        } else {
                            code.insert(pattern, 0);
                        }
                    } else {
                        code.insert(pattern.clone(), 6);
                        reference.insert(6, pattern.clone());
                    }
                },
                5 => {
                    let similar_chars = chars_in(pattern.clone(), reference.get(&6).unwrap().clone());
                    if similar_chars == 5 {
                        code.insert(pattern, 5);
                    } else {
                        let similar_chars_seven = chars_in(pattern.clone(), reference.get(&7).unwrap().clone());
                        if similar_chars_seven == 3 {
                            code.insert(pattern, 3);
                        } else{
                            code.insert(pattern, 2);
                        } 
                    }
                },
                _ => continue,
            }
        }

        let mut num = String::new();
        for digit in entry.output.clone() {
            if digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7 {
                sum_subset += 1;
            }

            let mut chars: Vec<char> = digit.chars().collect();
            chars.sort_unstable();
            let sorted_digit = chars.into_iter().collect::<String>();
            num = format!("{}{}", num, code.get(&sorted_digit).unwrap());
        }

        sum_vals += num.parse::<i32>().unwrap()
    }

    println!("part 1\nSum: {}", sum_subset);
    println!("part 2\nSum: {}", sum_vals);
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
