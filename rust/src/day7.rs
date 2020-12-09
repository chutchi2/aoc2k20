// #[derive(Debug)]
use adventofcode::lib::*;
use regex::Regex;
use std::env;
use std::path::Path;

// Rule parsed ex:
// ["light", "red", "1", "bright", "white", "2", "muted", "yellow"]
// ["dark", "orange", "3", "bright", "white", "4", "muted", "yellow"]
// ["bright", "white", "1", "shiny", "gold"]
// ["muted", "yellow", "2", "shiny", "gold", "9", "faded", "blue"]
// ["shiny", "gold", "1", "dark", "olive", "2", "vibrant", "plum"]
// ["dark", "olive", "3", "faded", "blue", "4", "dotted", "black"]
// ["vibrant", "plum", "5", "faded", "blue", "6", "dotted", "black"]
// ["faded", "blue", "no", "other"]
// ["dotted", "black", "no", "other"]

// Need to find if bag can hold gold
struct Bag {
    color: String,
    child_counts: Vec<i32>,
    child_bags: Vec<String>,
}

fn _process_rules(job: String) -> std::result::Result<usize, ()> {
    let seperator = Regex::new(r"([ ,.])|(?:bags|contain|bag)+").expect("Invalid regex");
    let mut splits: Vec<_> = seperator.split(&job).into_iter().collect();
    let mut bag_color;
    let mut bag: Bag;
    let mut ticker = 0;
    let mut count = 0;
    let mut counts: Vec<i32> = Vec::new();
    let mut bag_string: Vec<char> = Vec::new();
    let mut bags: Vec<String> = Vec::new();

    splits.dedup();
    splits.retain(|&x| x != "");
    if splits.len() > 0 {
        bag_color = vec![splits[0], splits[1]].into_iter().collect();
        splits.remove(0);
        splits.remove(0);
        if splits.len() == 2 {
            bag = Bag {
                color: bag_color,
                child_counts: vec![0],
                child_bags: vec!["".to_string()],
            };
        } else {
            for el in splits {
                match ticker {
                    0 => {
                        counts.push(el.parse::<i32>().unwrap());
                        ticker += 1;
                    },
                    1 => {
                        bag_string.append(&mut el.chars().collect());
                        ticker += 1;
                    },
                    2 => {
                        bag_string.append(&mut el.chars().collect());
                        let string = bag_string.iter().collect();
                        if string == "shinygold" {
                            count += 1;
                        }
                        bags.push(string);
                        ticker = 0;
                        bag_string = Vec::new();
                    },
                    _ => println!("Out of bounds"),
                }
                // println!("{:?} {:?} {:?}", bag_color,counts,bags);
                bag = Bag {
                    color: (*bag_color).to_string(),
                    child_counts: (*counts).to_vec(),
                    child_bags: (*bags).to_vec(),
                };
            }
        }
    }
    println!("got gold {}",count);
    Ok(count)
}

fn _d7(lines: Vec<String>) -> std::result::Result<usize, ()> {
    let mut count = 0;
    let mut correct: bool;
    let mut job: String = String::from("");
    let mut line_val: Vec<char>;
    let mut val = 0;
    let mut max = 0;
    for line in lines {
        val += _process_rules(line).unwrap();
        // if val > max {
        //     max = val;
        // }
        // line_val = line.chars().collect();
        // if line_val.len() == 0 {
        //     correct = _process_job(job).unwrap();
        //     job = String::from("");
        //     if correct {
        //         count += 1;
        //     }
        // } else {
        //     job = job + " " + &line;
        // }
    }
    Ok(val)
}

pub fn d7() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d7 = _d7(lines);
    println!("Shiny gold capable bags: {:?}", d7);
}
