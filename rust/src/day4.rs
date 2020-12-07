use adventofcode::lib::*;
use regex::Regex;
use std::env;
use std::path::Path;
// Rules
// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn _process_job(job: String) -> std::result::Result<bool, ()> {
    let seperator = Regex::new(r"([ :#])+").expect("Invalid regex");

    let splits: Vec<_> = seperator.split(&job).into_iter().collect();
    let mut ecl: String = "".to_string();
    let mut pid: String = "".to_string();
    let mut eyr: u32 = 0;
    let mut hcl: String = "".to_string();
    let mut byr: u32 = 0;
    let mut iyr: u32 = 0;
    let mut hgt: String = "".to_string();
    let mut _cid: u32 = 0;
    let mut count = 0;
    if splits.len() >= 7 {
        for &el in &splits {
            count += 1;
            match el {
                "ecl" => ecl = String::from(splits[count]),
                "pid" => pid = String::from(splits[count]),
                "eyr" => eyr = splits[count].parse::<u32>().unwrap(),
                "hcl" => hcl = String::from(splits[count]), //#fffffd
                "byr" => byr = splits[count].parse::<u32>().unwrap(),
                "iyr" => iyr = splits[count].parse::<u32>().unwrap(),
                "hgt" => hgt = String::from(splits[count]), // 183cm
                "cid" => _cid = splits[count].parse::<u32>().unwrap(),
                _ => println!("not a tag"),
            }
        }
        // check number
        let pid_as_no = pid.parse::<u32>().unwrap_or(0);
        let pid_len = pid.chars().collect::<Vec<char>>().len();
        let hcl_len = hcl.chars().collect::<Vec<char>>().len();
        let height_parse = Regex::new(r"([ci])+").expect("Invalid regex");
        let height: Vec<_> = height_parse.split(&hgt).into_iter().collect();
        let mut is_bad_height = false;
        let height_str = height[0].parse::<u32>().unwrap_or(0);
        if height.len() > 1 {
            match height[1] {
                "m" => is_bad_height = height_str < 150 || height_str > 193,
                "n" => is_bad_height = height_str < 59 || height_str > 76,
                _ => println!("Not a height identifier: {}", height[1]),
            }
        } else {
            is_bad_height = true;
        }
        let mut is_bad_eye = false;
        let ecl2 = &ecl[..];
        match ecl2 {
            "amb" => println!("eye color: {}", ecl),
            "blu" => println!("eye color: {}", ecl),
            "brn" => println!("eye color: {}", ecl),
            "gry" => println!("eye color: {}", ecl),
            "grn" => println!("eye color: {}", ecl),
            "hzl" => println!("eye color: {}", ecl),
            "oth" => println!("eye color: {}", ecl),
            _ => is_bad_eye = true,
        }
        if is_bad_height
            || ecl == ""
            || pid_as_no == 0
            || pid_len != 9
            || iyr < 2010
            || iyr > 2020
            || eyr < 2020
            || eyr > 2030
            || hcl_len != 6
            || byr < 1920
            || byr > 2002
            || iyr == 0
            || hgt == ""
            || is_bad_eye
        {
            Ok(false)
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn _d4(lines: Vec<String>) -> std::result::Result<i64, ()> {
    let mut count = 0;
    let mut correct: bool;
    let mut job: String = String::from("");
    let mut line_val: Vec<char>;
    for line in lines {
        line_val = line.chars().collect();
        if line_val.len() == 0 {
            correct = _process_job(job).unwrap();
            job = String::from("");
            if correct {
                count += 1;
            }
        } else {
            job = job + " " + &line;
        }
    }
    Ok(count)
}

pub fn d4() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d4 = _d4(lines);
    println!("Count of valid passports: {:?}", d4);
}
