use adventofcode::lib::*;
use std::env;
use std::path::Path;

fn _count_jolt_diff(jolt_ratings: &Vec<u64>) -> std::result::Result<u64, ()> {
    let mut one_jumps: u64 = 0;
    let mut three_jumps: u64 = 0;
    let mut tmp;
    for iter in 0..jolt_ratings.len() {
        if iter != 0 {
            tmp = jolt_ratings[iter] - jolt_ratings[iter - 1];
        } else {
            tmp = 0;
        }
        if tmp == 1 {
            one_jumps += 1;
        } else if tmp == 3 {
            three_jumps += 1;
        } else {
            println!("wtf is happening with this diff:{:?}", tmp);
        }
    }
    if one_jumps != 0{
        one_jumps +=1;
    }
    if three_jumps != 0{
        three_jumps +=1;
    }
    let ret = one_jumps * three_jumps;
    Ok(ret as u64)
}

fn _d10(lines: Vec<String>) -> std::result::Result<u64, ()> {
    let mut entries: Vec<u64> = Vec::new();
    let mut tmp;
    for line in lines {
        if line.len() > 0 {
            tmp = line.parse::<u64>().unwrap();
            entries.push(tmp);
        }
    }
    entries.sort();
    let p1_ret = _count_jolt_diff(&entries).unwrap();
    Ok(p1_ret)
}

pub fn d10() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d10 = _d10(lines);
    println!("the computed: {:?}", d10);
}
