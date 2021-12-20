use std::collections::HashMap;
use std::cmp;
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
    if let Ok(lines) = read_lines("./packet_decoder.txt") {
        for line in lines {
            if let Ok(hex) = line {
                let parts: Vec<String> = hex.split("").map(|p| p.to_string()).collect();
                for part in parts {
                    if part == "" {
                        continue;
                    }
                    let mut val: Vec<String> = translate_hex.get(&part).unwrap().chars().map(|b| b.to_string()).collect();
                    binary.append(&mut val);
                }
            }
        }
    }

    //println!("version sum: {}", sum_packet_version(binary.clone()));
    let s = order(binary, Vec::new());
    println!("order: {:?}", s);
    //println!("evaluated: {}", evaluate(s));
}

fn evaluate(mut s: Vec<String>) -> u32 {
    let mut buffer: Vec<u32> = Vec::new();
    while !s.is_empty() {
        println!("s: {:?}", s);
        println!("b: {:?}", buffer);
        let op = s.pop().unwrap();
        match &*op {
            "+" => {
                let mut val = 0;
                while !buffer.is_empty() {
                    val += buffer.pop().unwrap();
                }
                buffer.push(val);
            },
            "*" => {
                let mut val = 1;
                while !buffer.is_empty() {
                    val *= buffer.pop().unwrap();
                }
                buffer.push(val);
            },
            "min" => {
                let mut val = u32::MAX;
                while !buffer.is_empty() {
                    let num = buffer.pop().unwrap();
                    if num < val {
                        val = num;
                    }
                }
                buffer.push(val);
            },
            "max" => {
                let mut val = 0;
                while !buffer.is_empty() {
                    let num = buffer.pop().unwrap();
                    if num < val {
                        val = num;
                    }
                }
                buffer.push(val);
            },
            "gt" => {
                if buffer.pop().unwrap() < buffer.pop().unwrap() {
                    buffer.push(1);
                } else {
                    buffer.push(0);
                }
            },
            "lt" => {
                if buffer.pop().unwrap() > buffer.pop().unwrap() {
                    buffer.push(1);
                } else {
                    buffer.push(0);
                }
            },
            "eq" => {
                if buffer.pop().unwrap() == buffer.pop().unwrap() {
                    buffer.push(1);
                } else {
                    buffer.push(0);
                }
            },
            _ => {
                let num = u32::from_str_radix(&op, 2).unwrap();
                buffer.push(num);
            },
        }
    }

    return buffer.pop().unwrap();
}

fn order(b: Vec<String>, mut s: Vec<String>) -> Vec<String> {
     if b.len() <= 10 {
        return s;
    }
    let (prefix, mut rest) = b.split_at(6);
    // second 3 = type_id
    let type_id_bin = prefix[3..].join("");
    let type_id = u32::from_str_radix(&type_id_bin, 2).unwrap();

    // type = 4 -> number, no other segments
    if type_id == 4 {
        println!("num packet");
        let mut num_bin = String::from("");
        loop {
            let (num_bits, temp) = rest.split_at(5);
            rest = temp;
            num_bin.push_str(&*num_bits[1..].join(""));
            if num_bits[0] == "0" {
                break;
            }
        }

        let num = u32::from_str_radix(&num_bin, 2).unwrap();
        println!("num: {}", num);
        s.push(format!("{}", num));
        if rest.len() <= 10 {
            return s;
        }

        return order(rest.into_iter().cloned().collect(), s);
    }
    println!("operator packet");
    let mut exp_op: String = String::from("");
    match type_id {
        0 => exp_op = String::from("+"),
        1 => exp_op = String::from("*"),
        2 => exp_op = String::from("min"),
        3 => exp_op = String::from("max"),
        5 => exp_op = String::from("gt"),
        6 => exp_op = String::from("lt"),
        7 => exp_op = String::from("eq"),
        _ => unreachable!(),
    }
    s.push(exp_op);

    // length type id
    let mut rest_vec: Vec<String> = rest.into_iter().cloned().collect();
    let lt_id = rest_vec.remove(0);
    if lt_id == "0" { // 15 bits -> total length of subpackets
        let (_, temp) = rest_vec.split_at(15);
        rest_vec = temp.into_iter().cloned().collect();
    } else {
        let (_, temp) = rest_vec.split_at(11);
        rest_vec = temp.into_iter().cloned().collect();
    }

    return order(rest_vec, s);
}

/*
fn evaluate(b: Vec<String>, op: String) -> u32 {
    if b.len() <= 10 {
        return 0;
    }
    let (prefix, mut rest) = b.split_at(6);
    // second 3 = type_id
    let type_id_bin = prefix[3..].join("");
    let type_id = u32::from_str_radix(&type_id_bin, 2).unwrap();

    println!("b: {:?}\nop: {}\nt_id: {}", b, op, type_id);
    // type = 4 -> number, no other segments
    let mut exp_op: String = String::from("");
    if type_id == 4 {
        println!("num packet");
        let mut num_bin = String::from("");
        loop {
            let (num_bits, temp) = rest.split_at(5);
            rest = temp;
            num_bin.push_str(&*num_bits[1..].join(""));
            if num_bits[0] == "0" {
                break;
            }
        }
        let num = u32::from_str_radix(&num_bin, 2).unwrap();
        if rest.len() <= 10 {
            return num;
        }
        match &*op {
            "+" => {
                return num + evaluate(rest.into_iter().cloned().collect(), op);
            },
            "*" => {
                return num * evaluate(rest.into_iter().cloned().collect(), op);
            },
            "min" => {
                return cmp::min(num, evaluate(rest.into_iter().cloned().collect(), op));
            },
            "max" => {
                return cmp::max(num, evaluate(rest.into_iter().cloned().collect(), op));
            },
            "gt" => {
                if num > evaluate(rest.into_iter().cloned().collect(), exp_op){
                    return 1;
                }
                return 0;
            },
            "lt" => {
                if num < evaluate(rest.into_iter().cloned().collect(), exp_op){
                    return 1;
                }
                return 0;
            },
            "eq" => {
                if num == evaluate(rest.into_iter().cloned().collect(), exp_op){
                    return 1;
                }
                return 0;
            },
            _ => {
                return num;
            },
        }
    }
    println!("operator packet");
    match type_id {
        0 => exp_op = String::from("+"),
        1 => exp_op = String::from("*"),
        2 => exp_op = String::from("min"),
        3 => exp_op = String::from("max"),
        5 => exp_op = String::from("gt"),
        6 => exp_op = String::from("lt"),
        7 => exp_op = String::from("eq"),
        _ => unreachable!(),
    }

    // length type id
    let mut rest_vec: Vec<String> = rest.into_iter().cloned().collect();
    let lt_id = rest_vec.remove(0);
    if lt_id == "0" { // 15 bits -> total length of subpackets
        let (_, temp) = rest_vec.split_at(15);
        rest_vec = temp.into_iter().cloned().collect();
    } else {
        let (_, temp) = rest_vec.split_at(11);
        rest_vec = temp.into_iter().cloned().collect();
    }

    return evaluate(rest_vec, exp_op);
}
*/

fn sum_packet_version(b: Vec<String>) -> u32 {
    if b.len() <= 10 {
        return 0;
    }
    let (prefix, mut rest) = b.split_at(6);
    // first 3 = version
    let version_bin = prefix[..3].join("");
    let version = u32::from_str_radix(&version_bin, 2).unwrap();
    // second 3 = type_id
    let type_id_bin = prefix[3..].join("");
    let type_id = u32::from_str_radix(&type_id_bin, 2).unwrap();

    // type = 4 -> number, no other segments
    if type_id == 4 {
        println!("num packet");
        loop {
            let (num, temp) = rest.split_at(5);
            rest = temp;
            if num[0] == "0" {
                break;
            }
        }
        if rest.len() <= 10 {
            return version;
        }

        return version + sum_packet_version(rest.into_iter().cloned().collect());
    }
    println!("operator packet");

    // length type id
    let mut rest_vec: Vec<String> = rest.into_iter().cloned().collect();
    let lt_id = rest_vec.remove(0);
    if lt_id == "0" { // 15 bits -> total length of subpackets
        let (_, temp) = rest_vec.split_at(15);
        rest_vec = temp.into_iter().cloned().collect();
    } else {
        let (_, temp) = rest_vec.split_at(11);
        rest_vec = temp.into_iter().cloned().collect();
    }

    return version + sum_packet_version(rest_vec);
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
