use std::env;
use std::fs;

use AdventOfCode::time_it;

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

pub fn perform_task_one(mut input: Vec<i32>) -> Option<i32> {
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

pub fn perform_task_two(mut input: Vec<i32>) -> Option<i32> {
    input.sort();
    for elem in input.iter() {
        for rev_elem in input.iter().rev() {
            let filtered: Vec<&i32> = input
                .iter()
                .filter(|e| *e != elem && *e != rev_elem)
                .collect();
            for middle in filtered {
                if elem + rev_elem + middle == 2020 {
                    return Some(elem * rev_elem * middle);
                }
            }
        }
    }
    None
}

fn main() {
    let input = read_input();
    time_it! {
        (perform_task_one(input.clone()).unwrap(), perform_task_two(input.clone()).unwrap())
    };
}
