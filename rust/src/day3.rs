use adventofcode::lib::*;
use std::env;
use std::path::Path;

// Traverse forest
// . = open space
// # = tree
// Count how many trees are encountered when moving three right and one down across the whole map

fn _d3(jobs: Vec<String>) -> std::result::Result<i64, ()> {
    let mut curr_pos: usize;
    let mut tree_in_path_cnt: i64 = 0;
    let mut curr_row: Vec<char>;
    let width_set: Vec<char> = jobs[0].chars().collect();
    let width = width_set.len();
    let mut set = jobs;
    let mutdiff = set.remove(0);
    let mutdiff = set.remove(0);
    let x_coords: Vec<usize> = vec![1, 3, 5, 7, 1];
    let is_y_equal_2 = false;
    let mut is_odd_line = true;

    // for xcoord in x_coords {
        curr_pos = 1;
    //     tree_in_path_cnt = 0;
    //     is_odd_line = true;

        for tree_row in &set {
            if is_odd_line {
                curr_row = tree_row.chars().collect();
                if curr_row[curr_pos] == '#' {
                    tree_in_path_cnt += 1;
                }
                curr_pos += 1;
                if curr_pos >= width {
                    curr_pos = curr_pos - width;
                }
                is_odd_line = !is_odd_line;
            } else {
                is_odd_line = !is_odd_line;
            }
        }
        println!("{}", tree_in_path_cnt);
    // }
    Ok(tree_in_path_cnt)
}
pub fn d3() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d3 = _d3(lines);
    println!("{:?}", d3);
}
