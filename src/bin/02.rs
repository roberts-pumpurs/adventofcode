use std::env;
use std::fs;

use AdventOfCode::time_it;

#[derive(Debug, Clone)]
struct Password {
    low: u32,
    high: u32,
    letter: char,
    pw: String,
}

fn read_input() -> Vec<Password> {
    let filename = "src/bin/_02_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<Password> = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|e| {
            let colon: Vec<&str> = e.split(":").collect();
            // Parse left side of colon
            let left: Vec<&str> = colon[0].split(" ").collect();
            let bounds: Vec<&str> = left[0].split("-").map(|x| x.trim()).collect();
            let letter: &str = left[1].trim();

            // parse right side of colon
            let password: &str = colon[1].trim();
            return Password {
                low: bounds[0].parse::<u32>().unwrap(),
                high: bounds[1].parse::<u32>().unwrap(),
                letter: letter.chars().next().unwrap(),
                pw: password.to_owned(),
            };
        })
        .collect();
    contents
}

fn validate_password(pw: &Vec<Password>) -> u32 {
    pw.iter().fold(0, |acc_non_corrupted, x| {
        let repetitions = x.pw.chars().fold(0, |acc_inner, c| {
            if c == x.letter {
                acc_inner + 1
            } else {
                acc_inner
            }
        });
        if (x.low..x.high + 1).contains(&repetitions) {
            return acc_non_corrupted + 1;
        }
        acc_non_corrupted
    })
}
fn validate_password_part2(pw: &Vec<Password>) -> u32 {
    pw.iter().fold(0, |acc_non_corrupted, x| {
        let l = x.pw.chars().nth(x.low as usize - 1).unwrap() == x.letter;
        let r = x.pw.chars().nth(x.high as usize - 1).unwrap() == x.letter;
        if l ^ r {
            return acc_non_corrupted + 1;
        }
        acc_non_corrupted
    })
}

fn main() {
    let input = read_input();
    // let input_c = inp
    time_it! {
        (validate_password(&input), validate_password_part2(&input))
    };
}
