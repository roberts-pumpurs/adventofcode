use std::env;
use std::fs;

use day_1::{time_it};

pub fn read_input() -> Vec<i32> {
    let filename = "src/bin/_01_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<i32> = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    contents
}

pub fn perform_task(mut input: Vec<i32>) -> Option<i32> {
    // let mut input =
    input.sort();
    for elem in input.iter() {
        for rev_elem in input.iter().rev() {
            if elem + rev_elem == 2020 {
                return Some(elem * rev_elem);
            }
        }
    }
    None
}

fn main() {
    let mut input = read_input();
    time_it! {
        (perform_task(input).unwrap(), "")
    };
}
