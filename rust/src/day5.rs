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

fn update_coords(coords: &mut Vec<usize>, direction: char) -> std::result::Result<(),()> {
    let mut front: usize = coords[0];
    let mut back: usize = coords[1];
    let mut left: usize = coords[2];
    let mut right: usize = coords[3];
    match direction {
        'F' => coords[1] = (back + 1) / 2 - 1,
        'B' => coords[0] = (front + 1) / 2 - 1,
        'L' => coords[3] = (right + 1) / 2 - 1,
        'R' => coords[2] = (left + 1) / 2 - 1,
        _ => println!("Broken instruction"),
    }

    Ok(())
}

fn something_new(lines: Vec<String>) -> std::result::Result<usize, ()> {
    let front: usize = 0;
    let back: usize = 127;
    let left: usize = 0;
    let right: usize = 7;
    let mut dirs: Vec<char>;
    let mut pass_no = 0;
    let mut pass_nos = Vec::new();
    for line in &lines {
        let mut coords = vec![front, back, left, right];
        dirs = line.chars().collect();
        for dir in dirs {
            update_coords(&mut coords, dir);
        }
        if coords[0] == coords[1] && coords[2] == coords[3] {
            pass_no = coords[0] * 8 + coords[2];
            pass_nos.push(pass_no)
        }
    }
    let mut curr_max = 0;
    for no in pass_nos {
        if no > curr_max {
            curr_max = no;
        }
    }
    Ok(curr_max)
}
// fn _d5(lines: Vec<String>) -> std::result::Result<usize, ()> {
//     let mut count = 0;
//     let mut interim: Vec<char>;
//     let mut tickets = Vec::new();
//     let mut rows = Vec::new();
//     let seat_per_row: usize = 8;
//     let total_rows: usize = 129;
//     let seat_row = vec![1, 2, 3, 4, 5, 6, 7, 8];
//     let passenger_no: usize = lines.len();
//     let coords = vec![0, 0, 0];
//     for i in 1..total_rows {
//         rows.push(i);
//     }
//     for line in &lines {
//         interim = line.chars().collect();
//         tickets.push(interim);
//     }
//     println!("{:?}", seat_row);
//     Ok(total_rows)
// }

pub fn d5() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d5 = something_new(lines);
    println!("Highest ticket no.: {:?}", d5);
}
