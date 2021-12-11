use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let brackets: HashMap<String, String> = HashMap::from([
        (String::from("("), String::from(")")),
        (String::from("["), String::from("]")),
        (String::from("{"), String::from("}")),
        (String::from("<"), String::from(">")),
    ]);
    let mut error_score = 0;
    let error_score_guide: HashMap<String, i32> = HashMap::from([
        (String::from(")"), 3),
        (String::from("]"), 57),
        (String::from("}"), 1197),
        (String::from(">"), 25137),
    ]);
    let mut autocomplete_scores: Vec<i64> = Vec::new();
    let autocomplete_score_guide: HashMap<String, i32> = HashMap::from([
        (String::from("("), 1),
        (String::from("["), 2),
        (String::from("{"), 3),
        (String::from("<"), 4),
    ]);

    if let Ok(lines) = read_lines("./syntax_scoring.txt") {
        for line in lines {
            let mut stack: Vec<String> = Vec::new();
            if let Ok(chunk) = line {
                let mut invalid = false;
                let characters: Vec<String> = chunk.chars().map(|c| c.to_string()).collect();
                for c in characters {
                    if brackets.contains_key(&c) {
                        stack.push(c);
                        continue;
                    }
                    let matching_char = stack.pop().unwrap();
                    if brackets.contains_key(&matching_char) {
                        if *brackets.get(&matching_char).unwrap() == c {
                            // valid
                            continue;
                        }
                    }
                    // invalid
                    error_score += *error_score_guide.get(&c).unwrap();
                    invalid = true;
                    break;
                }
                if invalid {
                    continue;
                }
                let mut autocomplete_score = 0;
                while stack.len() > 0 {
                    let c = stack.pop().unwrap();
                    autocomplete_score *= 5;
                    autocomplete_score += *autocomplete_score_guide.get(&c).unwrap() as i64;
                }
                if autocomplete_score > 0 {
                    autocomplete_scores.push(autocomplete_score);
                }
            }
        }
    }

    autocomplete_scores.sort();

    println!("error score: {}", error_score);
    println!(
        "autocomplete score: {}",
        autocomplete_scores[autocomplete_scores.len() / 2]
    );
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
