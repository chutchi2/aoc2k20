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
    let mut index_change = index;
    let mut acc_change: i64 = 0;
    if split_codes.len() > 0 {
        let op = split_codes[0];
        let arg = split_codes[1].parse::<i64>().unwrap();
        println!("{:?}",arg);
        match op {
            "nop" => index_change = 1 + index,
            "acc" => {
                acc_change = arg;
                index_change = 1 + index;
            }
            "jmp" => {
                index_change = arg + index;
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
    let mut curr_index:i64 = 0;
    let mut op_count = 0;
    let mut curr_code: String;
    while op_count < lines.len()-1 {
        curr_code = lines[curr_index as usize].to_string();
        let (index_change, acc_change) = _parse_opcode(curr_code,curr_index).unwrap();
        curr_index += index_change;
        accumulator += acc_change;
        op_count += 1;
    }
    Ok(accumulator)
}

pub fn d8() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d8 = _d8(lines);
    println!("Count of valid passports: {:?}", d8);
}
