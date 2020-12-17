use std::env;
use std::fs;

use AdventOfCode::time_it;

#[derive(Debug, PartialEq)]
struct Document {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn read_input() -> Vec<Document> {
    let filename = "src/bin/_04_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<Document> = contents
        .split("\n\n")
        .filter(|x| *x != "")
        .map(|e| {
            let mut doc = Document {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };
            e.split_ascii_whitespace().for_each(|item| {
                let a: Vec<&str> = item.split(":").take(2).collect();
                let key = a.get(0).unwrap();
                let val = a.get(1).unwrap();

                match *key {
                    "byr" => doc.byr = Some((*val).to_owned()),
                    "iyr" => doc.iyr = Some((*val).to_string()),
                    "eyr" => doc.eyr = Some((*val).to_string()),
                    "hgt" => doc.hgt = Some((*val).to_string()),
                    "hcl" => doc.hcl = Some((*val).to_string()),
                    "ecl" => doc.ecl = Some((*val).to_string()),
                    "pid" => doc.pid = Some((*val).to_string()),
                    "cid" => doc.cid = Some((*val).to_string()),
                    _ => (),
                }
            });
            return doc;
        })
        .collect();
    contents
}

fn part_1(input: Vec<Document>) -> i32 {
    let result = input.iter().fold(0, |current, item| {
        if item.byr.is_some()
            && item.iyr.is_some()
            && item.eyr.is_some()
            && item.hgt.is_some()
            && item.hcl.is_some()
            && item.ecl.is_some()
            && item.pid.is_some()
            && (item.cid.is_some() || item.cid.is_none())
        {
            return current + 1;
        } else {
            return current;
        }
    });
    return result;
}

fn main() {
    let input = read_input();
    time_it! {
        (part_1(input), "")
    };
}
