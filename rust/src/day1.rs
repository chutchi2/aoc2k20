use adventofcode::lib::*;
use std::env;
use std::path::Path;

fn _d1(records: Vec<i64>) -> std::result::Result<i64, ()> {
    for num1 in &records {
        for num2 in &records {
            if num1 > &1000 && num2 > &1000 {
            } else {
                let ret = num1 + num2;
                let retta = num1 * num2;
                if ret == 2020 {
                    println!("{:?} {:?} {:?} {:?}", ret, num1, num2, retta);
                }
            }
        }
    }
    Ok(records[0])
}
fn _d1_p2(records: Vec<i64>) -> std::result::Result<i64, ()> {
    for num1 in &records {
        for num2 in &records {
            for num3 in &records {
                if num1 > &0 && num2 > &0 && num3 > &0 {
                    let ret = num1 + num2 + num3;
                    let retta = num1 * num2 * num3;
                    if ret == 2020 {
                        println!("{:?} {:?} {:?} {:?} {:?}", ret, num1, num2, num3, retta);
                    }
                }
            }
        }
    }
    Ok(records[0])
}
pub fn d1() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let records = record_parser(lines);
    let d1_p1_ret = _d1_p2(records);
}
