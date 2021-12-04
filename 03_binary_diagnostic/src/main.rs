use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("part 1");
    let mut zeroes = HashMap::new();
    let mut ones = HashMap::new();
    if let Ok(lines) = read_lines("./binary_diagnostic.txt") {
        for line in lines {
            if let Ok(diagnostic) = line {
                for (i, bit) in diagnostic.chars().enumerate() {
                    match bit {
                        '1' => {
                            let count = ones.entry(i).or_insert(0);
                            *count += 1;
                        }
                        '0' => {
                            let count = zeroes.entry(i).or_insert(0);
                            *count += 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();
    for n in 0..zeroes.len() {
        let z_count = zeroes.entry(n).or_insert(0);
        let o_count = ones.entry(n).or_insert(0);
        if z_count > o_count {
            gamma_binary += "0";
            epsilon_binary += "1";
        } else {
            gamma_binary += "1";
            epsilon_binary += "0";
        }
    }

    let gamma_int = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon_binary, 2).unwrap();
    println!(
        "gamma: {}\nepsilon: {}\npower consumption: {}",
        gamma_int,
        epsilon_int,
        gamma_int * epsilon_int
    );

    println!("part 2");
    let mut report: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./binary_diagnostic.txt") {
        for line in lines {
            if let Ok(diagnostic) = line {
                report.push(diagnostic);
            }
        }
    }

    let mut o2_opts: Vec<String> = report.clone();
    let mut i = 0;
    while o2_opts.len() > 1 {
        let mut z: Vec<String> = Vec::new();
        let mut o: Vec<String> = Vec::new();

        for diagnostic in o2_opts {
            let bit = diagnostic.chars().nth(i).unwrap();
            match bit {
                '1' => o.push(diagnostic),
                '0' => z.push(diagnostic),
                _ => unreachable!(),
            }
        }

        if z.len() > o.len() {
            o2_opts = z;
        } else {
            o2_opts = o;
        }

        i += 1;
    }
    let o2 = o2_opts.pop().unwrap();
    println!("o2: {}", o2);

    let mut co2_opts: Vec<String> = report.clone();
    i = 0;
    while co2_opts.len() > 1 {
        let mut z: Vec<String> = Vec::new();
        let mut o: Vec<String> = Vec::new();

        for diagnostic in co2_opts {
            let bit = diagnostic.chars().nth(i).unwrap();
            match bit {
                '1' => o.push(diagnostic),
                '0' => z.push(diagnostic),
                _ => unreachable!(),
            }
        }

        if o.len() >= z.len() {
            co2_opts = z;
        } else {
            co2_opts = o;
        }

        i += 1;
    }
    let co2 = co2_opts.pop().unwrap();
    println!("co2: {}", co2);

    let o2_int = isize::from_str_radix(&o2, 2).unwrap();
    let co2_int = isize::from_str_radix(&co2, 2).unwrap();

    println!(
        "o2: {}\nco2: {}\nlife support rating: {}",
        o2_int,
        co2_int,
        o2_int * co2_int, 
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
