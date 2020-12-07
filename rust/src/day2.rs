use adventofcode::lib::*;
use regex::Regex;
use std::env;
use std::path::Path;

fn _letter_checker(
    first: i64,
    second: i64,
    key_letter: char,
    password: String,
) -> std::result::Result<bool, ()> {
    let mut isAtFirstPos = false;
    let mut isAtSecondPos = false;
    let mut count = 0;
    for letter in password.chars() {
        count += 1;
        if letter == key_letter {
            if count == first {
                isAtFirstPos = true;
            } else if count == second {
                isAtSecondPos = true;
            }
        }
    }

    println!("{} {} {} {} {} {}", first, second, count, key_letter, isAtFirstPos, !isAtSecondPos);
    let ret = isAtFirstPos != isAtSecondPos;
    Ok(ret)
}
fn _process_jobs(job: String) -> std::result::Result<(i64, i64, char, String), ()> {
    let low: i64;
    let high: i64;
    let key_letter: char;
    let password: String;

    let seperator = Regex::new(r"([ :-])+").expect("Invalid regex");
    let splits: Vec<_> = seperator.split(&job).into_iter().collect();

    low = splits[0].parse::<i64>().unwrap();
    high = splits[1].parse::<i64>().unwrap();
    key_letter = splits[2].parse::<char>().unwrap();
    password = splits[3].to_string();
    let ret = (low, high, key_letter, password);
    Ok(ret)
}
fn _d2(jobs: Vec<String>) -> std::result::Result<i64, ()> {
    let low: i64;
    let high: i64;
    let key_letter: char;
    let password: String;
    let mut count = 0;
    let mut correct = false;
    for job in jobs {
        let (low, high, key_letter, password) = _process_jobs(job).unwrap();
        correct = _letter_checker(low, high, key_letter, password).unwrap();
        if (correct) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn d2() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d2 = _d2(lines);
    println!("{:?}", d2);
}
