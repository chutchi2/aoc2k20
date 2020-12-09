use adventofcode::lib::*;
use std::env;
use std::path::Path;

fn _slide_parse(entries: Vec<u64>) -> std::result::Result<u64, ()> {
    let mut modified_entries = entries.to_vec();
    let mut sums:Vec<u64> = Vec::new();
    let mut bad_num: u64 = 0;
    let mut tmp;
    let slide_window:Vec<u64> = modified_entries[0..25].to_vec();

    for slide_one in &slide_window {
        for slide_two in &slide_window {
            tmp = slide_one+slide_two;
            sums.push(tmp);
        }
    }
    sums.sort();
    sums.dedup();
    if !sums.contains(&modified_entries[25]) {
        bad_num = modified_entries[25];
        Ok(bad_num)
    } else {
        modified_entries.remove(0);
        println!("About to recurse!!!");
        _slide_parse(modified_entries)
    }
}

fn _d9(lines: Vec<String>) -> std::result::Result<u64, ()> {
    let mut entries: Vec<u64> = Vec::new();
    let mut tmp;
    for line in lines {
        if line.len() > 0 {
            tmp = line.parse::<u64>().unwrap();
            entries.push(tmp);
        }
    }
    let mut ret = _slide_parse(entries).unwrap();
    Ok(ret)
}

pub fn d9() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d9 = _d9(lines);
    println!("the bad number: {:?}", d9);
}
