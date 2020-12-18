use core::ops::Range;
use std::fs;
use std::{cmp, env};

use AdventOfCode::time_it;

#[derive(Debug, Clone)]
struct Seat(String);

fn read_input() -> Vec<Seat> {
    let filename = "src/bin/_05_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<Seat> = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|e| Seat(e.to_owned()))
        .collect();
    contents
}

fn get_position(input: &str) -> (i32, i32) {
    let (mut lrow, mut hrow) = (0, 128);
    let (mut lcol, mut hcol) = (0, 8);

    for c in input.chars() {
        match c {
            'F' => hrow = (hrow + lrow) / 2,
            'B' => lrow = (hrow + lrow) / 2,
            'R' => lcol = (hcol + lcol) / 2,
            'L' => hcol = (hcol + lcol) / 2,
            _ => unreachable!(),
        }
    }
    (lrow, lcol)
}

fn part_1(input: &Vec<Seat>) -> i32 {
    let res = input
        .iter()
        .map(|s| get_position(&s.0))
        .map(|(lrow, lcol)| lrow * 8 + lcol)
        .max()
        .unwrap_or(0);
    res
}

fn main() {
    let input = read_input();
    // println!("{:#?}", input);
    time_it! {
        (part_1(&input), "")
    };
}
