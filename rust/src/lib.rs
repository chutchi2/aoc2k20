use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Iterator;
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn record_parser(lines: Vec<String>) -> Vec<i64> {
    let mut records = Vec::new();
    for line in lines {
        records.push(line.parse::<i64>().unwrap());
    }
    records
}

pub fn job_parser(records: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut job_set: Vec<Vec<i64>> = Vec::new();
    for slice in records.chunks(4){
            job_set.push(slice.to_vec());
            println!("{:?}",slice.to_vec());
    }
    job_set
}
pub fn d2_p1(mut records: Vec<i64>, job_set: Vec<Vec<i64>>) -> std::result::Result<i64, ()> {
    for job in job_set{
        match job[0]{
            1 => records[job[3] as usize] = records[job[1] as usize] + records[job[2] as usize],
            2 => records[job[3] as usize] = records[job[1] as usize]*records[job[2] as usize],
            99 => println!("Jobs done well!"),
            _ => println!("Jobs done..."),
        }
    }
    Ok(records[0])
}
pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let records = record_parser(lines);
    let test_records = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    let test_records = vec![1,1,1,4,99,5,6,0,99];
    let jobs = job_parser(&test_records);
    let d2_p1_ret = d2_p1(test_records,jobs);
    println!("{}",d2_p1_ret.unwrap());
}