use adventofcode::lib::*;
use std::env;
use std::path::Path;

fn _d_example(lines: Vec<String>) -> std::result::Result<usize, ()> {
    let mut count = 0;
    let mut line_val: Vec<char>;
    let mut user_answers: Vec<char> = Vec::new();
    let mut users = Vec::new();
    let mut yes_count;
    let mut line_len;
    for line in lines {
        line_val = line.chars().collect();
        line_len = line_val.len();
        for val in line_val {
            user_answers.push(val);
        }
        if line_len == 0 {
            user_answers.sort();
            user_answers.dedup();
            yes_count = user_answers.len();
            users.push(yes_count);
            user_answers = Vec::new();
        }
    }
    for user in users {
        count += user;
    }
    Ok(count)
}
// This list represents answers from five groups:

//     In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
//     In the second group, there is no question to which everyone answered "yes".
//     In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
//     In the fourth group, everyone answered yes to only 1 question, a.
//     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

// In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

// For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
fn _d6_p2(lines: Vec<String>) -> std::result::Result<usize, ()> {
    let mut count = 0;
    let mut line_val: Vec<char>;
    let mut line_len;
    let mut user_group: Vec<Vec<char>> = Vec::new();
    let mut user_groups: Vec<Vec<Vec<char>>> = Vec::new();

    for line in lines {
        line_val = line.chars().collect();
        line_len = line_val.len();
        user_group.push(line_val);

        if line_len == 0 {
            user_group.pop();
            user_groups.push(user_group);
            user_group = Vec::new();
        }
    }

    let mut dup_count = 0;
    let mut user_answers: Vec<char> = Vec::new();

    for user in user_groups {
        let user_len = user.len();
        if user.len() == 1 {
            count += user[0].len();
        } else {
            for mut answers in user {
                answers.sort();
                for answer in answers {
                    user_answers.push(answer);
                }
            }
            user_answers.sort();
            for answer_iter in 0..user_answers.len() {
                if user_answers.len() == answer_iter + 1 {
                    if user_len == (dup_count + 1) {
                        count += 1;
                        dup_count = 0;
                    }
                } else if user_answers[answer_iter] == user_answers[answer_iter + 1] {
                    dup_count += 1;
                } else if user_len == (dup_count + 1) {
                    count += 1;
                    dup_count = 0;
                } else {
                    dup_count = 0;
                }
            }
        }
        user_answers = Vec::new();
    }

    Ok(count)
}

pub fn d6() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let lines = lines_from_file(path);
    let d6 = _d6_p2(lines);
    println!("Yes Count: {:?}", d6);
}
