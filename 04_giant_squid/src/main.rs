use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut to_call: Vec<i32> = Vec::new();
    let mut boards: Vec<HashMap<i32, (i32, i32)>> = Vec::new();
    if let Ok(lines) = read_lines("./giant_squid.txt") {
        let mut board = HashMap::new();
        let mut i = 0;
        let mut j = 0;
        for line in lines {
            if to_call.len() == 0 {
                if let Ok(called_numbers) = line {
                    for num in called_numbers.split(",") {
                        to_call.push(num.parse::<i32>().unwrap());
                    }
                }
                continue;
            }
            if let Ok(board_row) = line {
                if board_row == "" && board.len() != 0 {
                    boards.push(board);
                    board = HashMap::new();
                    i = 0;
                    j = 0;
                    continue;
                }

                for num in board_row.split_whitespace() {
                    board.insert(num.parse::<i32>().unwrap(), (i, j));
                    i += 1;
                }
                i = 0;
            }
            j += 1;
        }
        boards.push(board);
    }

    println!("part 1");
    let mut selections = HashMap::new();
    let mut winning_board_nums: Vec<i32> = Vec::new();
    let mut winning_call = 0;
    let mut called: Vec<i32> = Vec::new();
    for num in &to_call {
        called.push(*num);
        for (n, board) in boards.iter().enumerate() {
            match board.get(&num) {
                Some(coord) => {
                    let (i, j) = coord;
                    let row_count = selections.entry(format!("{}{}{}", n, "r", i)).or_insert(0);
                    *row_count += 1;
                    if *row_count == 5 {
                        winning_board_nums = board.clone().into_keys().collect();
                        winning_call = *num;
                        break;
                    }
                    let col_count = selections.entry(format!("{}{}{}", n, "c", j)).or_insert(0);
                    *col_count += 1;
                    if *col_count == 5 {
                        winning_board_nums = board.clone().into_keys().collect();
                        winning_call = *num;
                        break;
                    }
                },
                None => continue,
            }
        }
        if winning_call != 0 {
            break;
        }
    }

    let mut sum = 0;
    for num in winning_board_nums {
        if called.contains(&num) {
            continue;
        }
        sum += num;
    }

    println!("Sum of all uncalled numbers: {}\nLast number called: {}\nProduct: {}", sum, winning_call, sum*winning_call);
    println!("part 2");
    /*
    selections.clear();
    called.clear();
    let mut winning_boards: Vec<usize> = Vec::new();
    let mut last_board_nums: Vec<i32> = Vec::new();
    let mut last_win_call = 0;
    for num in &to_call {
        called.push(*num);
        println!("called {}", *num);
        for (n, board) in boards.iter().enumerate() {
            if winning_boards.contains(&n) {
                continue;
            }
            match board.get(&num) {
                Some(coord) => {
                    let (i, j) = coord;
                    let row_count = selections.entry(format!("{}{}{}", n, "r", i)).or_insert(0);
                    *row_count += 1;
                    if *row_count == 5 {
                        println!("board {} won", n);
                        winning_boards.push(n);
                        last_board_nums = board.clone().into_keys().collect();
                        last_win_call = *num;
                    }
                    let col_count = selections.entry(format!("{}{}{}", n, "c", j)).or_insert(0);
                    *col_count += 1;
                    if *col_count == 5 {
                        println!("board {} won", n);
                        winning_boards.push(n);
                        last_board_nums = board.clone().into_keys().collect();
                        last_win_call = *num;
                    }
                },
                None => continue,
            }
        }
        if winning_boards.len() == boards.len() {
            break;
        }
    }

    sum = 0;
    for num in last_board_nums {
        if called.contains(&num) {
            continue;
        }
        println!("adding num: {}", num);
        sum += num;
    }

    println!("Sum of all uncalled numbers: {}\nLast number called: {}\nProduct: {}", sum, last_win_call, sum*last_win_call);
    */

    let mut max_moves = 0;
    let mut max_board = 0;
    for (n, board) in boards.iter().enumerate() {
        selections.clear();
        called.clear();
        for num in &to_call {
            called.push(*num);
            match board.get(&num) {
                Some(coord) => {
                    let (i, j) = coord;
                    let row_count = selections.entry(format!("{}{}", "r", i)).or_insert(0);
                    *row_count += 1;
                    if *row_count == 5 {
                        if called.len() > max_moves {
                            max_moves = called.len();
                            max_board = n;
                        }
                        break;
                    }
                    let col_count = selections.entry(format!("{}{}", "c", j)).or_insert(0);
                    *col_count += 1;
                    if *col_count == 5 {
                        if called.len() > max_moves {
                            max_moves = called.len();
                            max_board = n;
                        }
                        break;
                    }
                },
                None => continue,
            }
        }
    }

    let replay = &to_call[..max_moves];

    let last_board_nums: Vec<i32> = boards[max_board].clone().into_keys().collect();
    sum = 0;
    for num in last_board_nums {
        if !replay.contains(&num) {
            sum += num;
        }
    }

    println!("Sum of all uncalled numbers: {}\nLast number called: {}\nProduct: {}", sum, replay.last().copied().unwrap(), sum*replay.last().copied().unwrap());


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
