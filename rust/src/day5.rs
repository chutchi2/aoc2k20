use adventofcode::lib::*;
use regex::Regex;
use std::env;
use std::path::Path;
// Rules

// Every seat has a unique seat ID: multiply the row by 8, then add the column.
// FBFBBFFRLR: row 44, column 5, seat ID 357.
// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820.

fn _d5(lines: Vec<String>) -> std::result::Result<isize, ()> {
    let mut pass_no: isize = 0;
    let mut pass_nos = Vec::new();
    let mut pass_nos_list = Vec::new();
    for line in &lines {
        let mut seats = Vec::new();
        let mut row_bins = Vec::new();
        let mut seat_bins = Vec::new();
        let mut row_as_bin: String;
        let mut seat_as_bin: String;
        let mut row_no;
        let mut seat_no;
        seats = line.chars().collect();
        for seat in seats {
            match seat {
                'F' => row_bins.push('0'),
                'B' => row_bins.push('1'),
                'L' => seat_bins.push('0'),
                'R' => seat_bins.push('1'),
                _ => println!("Broken instruction"),
            }
        }
        row_as_bin = row_bins.iter().collect();
        seat_as_bin = seat_bins.iter().collect();
        row_no = isize::from_str_radix(&row_as_bin, 2).unwrap();
        seat_no = isize::from_str_radix(&seat_as_bin, 2).unwrap();
        pass_nos.push(vec![row_no, seat_no])
    }
    let mut curr_max: isize = 0;
    for no in &pass_nos {
        pass_no = no[0] * 8 + no[1];
        pass_nos_list.push(pass_no);
        if pass_no > curr_max {
            curr_max = pass_no;
        }
    }
    pass_nos_list.sort();
    let mut last_seat_ids_index = pass_nos_list.len() - 1;
    let mut current_index = 0;
    let mut my_seat_id;
    while current_index < last_seat_ids_index - 1 {
        println!(
            "{:?} {:?}",
            pass_nos_list[current_index + 1],
            pass_nos_list[current_index]
        );
        if pass_nos_list[current_index + 1] - pass_nos_list[current_index] == 2 {
            my_seat_id = pass_nos_list[current_index] + 1;
            println!("{:?}", my_seat_id);
            break;
        }
        current_index += 1;
    }
    Ok(curr_max)
}

pub fn d5() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d5 = _d5(lines);
    println!("Highest ticket no.: {:?}", d5);
}
