mod lib;
use std::env;
use std::io::prelude::*;
use std::path::Path;

fn _d1_p1(mut records: Vec<i64>, job_set: Vec<Vec<i64>>) -> std::result::Result<i64, ()> {
    for job in job_set{
        match job[0]{
            1 => records[job[3] as usize] = records[job[1] as usize] + records[job[2] as usize],
            2 => records[job[3] as usize] = records[job[1] as usize] * records[job[2] as usize],
            99 => println!("Jobs done well!"),
            _ => println!("Jobs done..."),
        }
    }
    Ok(records[0])
}

pub fn d1() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    const path = Path::new(&args[1]);
    const lines = lib::lines_from_file(path);
    let records = lib::record_parser(lines);
    let test_records = vec![1,1,1,4,99,5,6,0,99];
    let jobs = lib::job_parser(&test_records);
    let d1_p1_ret = _d1_p1(test_records,jobs);
    println!("{}",d1_p1_ret.unwrap());
}