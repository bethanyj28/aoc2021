use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let translate_hex: HashMap<String, String> = HashMap::from([
        (String::from("0"),String::from("0000")),
        (String::from("1"),String::from("0001")),
        (String::from("2"),String::from("0010")),
        (String::from("3"), String::from("0011")),
        (String::from("4"), String::from("0100")),
        (String::from("5"), String::from("0101")),
        (String::from("6"), String::from("0110")),
        (String::from("7"), String::from("0111")),
        (String::from("8"), String::from("1000")),
        (String::from("9"), String::from("1001")),
        (String::from("A"), String::from("1010")),
        (String::from("B"), String::from("1011")),
        (String::from("C"), String::from("1100")),
        (String::from("D"), String::from("1101")),
        (String::from("E"), String::from("1110")),
        (String::from("F"), String::from("1111")),
    ]);
    let mut binary: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./packet_decoder_simple.txt") {
        for line in lines {
            if let Ok(hex) = line {
                let parts: Vec<String> = hex.split("").map(|p| p.to_string()).collect();
                for part in parts {
                    if part == "" {
                        continue;
                    }
                    let val = translate_hex.get(&part).unwrap();
                    binary.append(val.to_mut_vec()); 
                }
            }
        }
    }
}



/*
fn sum_packet_version(b: String) -> u32 {
    if b.len() <= 10 {
        return 0
    }
    let bits: Vec<String> = b.split("").map(|c| c.to_string()).collect();

    let (prefix, rest) = bits.split_at(6);
    let version_bin = prefix[..3].join("");
    let version_sum = u32::from_str_radix(&version_bin, 2).unwrap();
    let type_id_bin = prefix[3..].join("");
    let type_id = u32::from_str_radix(&type_id_bin, 2).unwrap();
    if type_id == 4 {
        loop {
            let num = rest[:4];
            rest = rest[4:];
            if num[0] == "0" {
                break;
            }
        }
    } else {
        let lt_id = rest.remove(0);
        if lt_id == "0" { // 15 bits, total length of sub packets
            let length = rest[:15];
            rest = rest[15:];
            version_sum += sum_packet_version(rest[:length]);
            rest = rest[length:];
        }else{
        }
    }
    
    return 0;
}

fn sum_packet_version(mut b: String) -> u32 {
    let mut version_count = 0;
    while b.len() >= 10 {
        println!("binary: {}\nversion_count: {}", b, version_count);
        let (prefix, mut rest) = b.split_at(6);
        let (v, t) = prefix.split_at(3);
        version_count += u32::from_str_radix(&v, 2).unwrap();
        let t_id = u32::from_str_radix(&t, 2).unwrap();
        if t_id == 4 {
            loop {
                let (num, temp) = rest.split_at(4);
                rest = temp;
                if u32::from_str_radix(&num, 2).unwrap() < 8 {
                    break;
                }
            }
        } else {
            let lt_id = rest.to_string().remove(0);
            if lt_id == '0' {
                let (_, temp) = rest.split_at(15);
                rest = temp;
            } else {
                let (_, temp) = rest.split_at(11);
                rest = temp;
            }
        }
        b = rest.to_string();
    }

    return version_count
}
*/

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
