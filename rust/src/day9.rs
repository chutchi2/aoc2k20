use adventofcode::lib::*;
use std::env;
use std::path::Path;

fn _slide_parse(entries: &Vec<u64>) -> std::result::Result<u64, ()> {
    let mut modified_entries = entries.to_vec();
    let mut sums:Vec<u64> = Vec::new();
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
        Ok(modified_entries[25])
    } else {
        modified_entries.remove(0);
        _slide_parse(&modified_entries)
    }
}
fn _slide_parse_2(entries: &Vec<u64>, target_num: u64) -> std::result::Result<u64, ()> {
    let mut special_sums: Vec<u64> = Vec::new();
    let mut running_count: u64 = 0;
    let mut modded_entries: Vec<u64> = entries.to_vec();
    let mut tmp = 0;
    let mut is_bad_set:bool = false;
    for entry in entries {
        running_count += entry;
        if running_count > target_num {
            is_bad_set = true;
            modded_entries.remove(0);
            break;
        } else if running_count < target_num {
            special_sums.push(*entry);
        } else if running_count == target_num {
            special_sums.push(*entry);
            special_sums.sort();
            tmp=special_sums[0] + special_sums.pop().unwrap();
            break;
        }
    }
    if is_bad_set {
        _slide_parse_2(&modded_entries, target_num)
    } else {
        Ok(tmp)
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
    let p1_ret = _slide_parse(&entries).unwrap();
    let p2_ret = _slide_parse_2(&entries,p1_ret).unwrap();
    Ok(p2_ret)
}

pub fn d9() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d9 = _d9(lines);
    println!("the bad number: {:?}", d9);
}
