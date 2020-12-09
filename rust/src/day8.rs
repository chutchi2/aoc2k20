use adventofcode::lib::*;
use regex::Regex;
use std::env;
use std::path::Path;
// Visit order
// nop +0  | 1
// acc +1  | 2, 8(!)
// jmp +4  | 3
// acc +3  | 6
// jmp -3  | 7
// acc -99 |
// acc +1  | 4
// jmp -4  | 5
// acc +6  |
// Goal: get accumulator value before code loops

fn _parse_opcode(job: String, index: i64) -> std::result::Result<(i64, i64), ()> {
    let seperator = Regex::new(r"([ +])+").expect("Invalid regex");
    let split_codes: Vec<_> = seperator.split(&job).into_iter().collect();
    let mut index_change = 0;
    let mut acc_change: i64 = 0;
    let mut op: &str = "";
    let mut arg: i64 = 0;
    if split_codes.len() > 0 {
        op = split_codes[0];
        arg = split_codes[1].parse::<i64>().unwrap();
        match op {
            "nop" => {
                acc_change = 0;
                index_change = 1;
            }
            "acc" => {
                acc_change = arg;
                index_change = 1;
            }
            "jmp" => {
                acc_change = 0;
                index_change = arg;
            }
            _ => println!("Invalid Opcode"),
        }
    }
    let ret = (index_change, acc_change);
    Ok(ret)
}

fn _d8(lines: Vec<String>) -> std::result::Result<i64, ()> {
    let mut count = 0;
    let mut line_val: Vec<char>;
    let mut accumulator: i64 = 0;
    let mut curr_index: i64 = 0;
    let mut op_count = 0;
    let mut curr_code: String;
    let mut lines_visited: Vec<i64> = Vec::new();
    let mut modified_lines = lines.to_vec();
    while op_count < lines.len() - 1 {
        if lines_visited.contains(&curr_index) {
            modified_lines[curr_index as usize - 2] = "nop +0".to_string();
            println!("About to recurse");
            accumulator = _d8(modified_lines).unwrap();
            break;
        } else if curr_index as usize >= lines.len() {
            break;
        } else if lines[curr_index as usize].len() == 0 {
            println!("The end i guess");
            break;
        } else {
            lines_visited.push(curr_index);
            curr_code = lines[curr_index as usize].to_string();
            let (index_change, acc_change) = _parse_opcode(curr_code, curr_index).unwrap();
            curr_index += index_change;
            accumulator += acc_change;
            op_count += 1;
        }
    }
    Ok(accumulator)
}

pub fn d8() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d8 = _d8(lines);
    println!("accumulator: {:?}", d8);
}
