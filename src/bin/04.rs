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

fn part_1(input: &Vec<Document>) -> i32 {
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

fn part_2(input: &Vec<Document>) -> i32 {
    let validator = |el: &str, lower: i32, higher: i32| {
        el.parse::<i32>()
            .map_or(false, |nr_val| (lower..higher + 1).contains(&nr_val))
            .into()
    };
    let result = input.iter().fold(0, |current, item| {
        let valid_byr: bool = item
            .byr
            .as_ref()
            .map_or(false, |el| validator(el, 1920, 2002));

        let valid_iyr = item
            .iyr
            .as_ref()
            .map_or(false, |el| validator(el, 2010, 2020));
        let valid_eyr = item
            .eyr
            .as_ref()
            .map_or(false, |el| validator(el, 2020, 2030));
        let valid_hgt = item.hgt.as_ref().map_or(false, |el| {
            if el.ends_with("in") {
                let nr = el.split("in").next().map_or(false, |e| {
                    e.parse::<i32>().map_or(false, |e| (59..76 + 1).contains(&e))
                });
                return nr;
            } else if el.ends_with("cm") {
                let nr = el.split("cm").next().map_or(false, |e| {
                    e.parse::<i32>().map_or(false, |e| (150..193 + 1).contains(&e))
                });
                return nr;
            }
            return false;
        });
        let valid_hcl = item.hcl.as_ref().map_or(false, |e| {
            e.starts_with("#") && e.chars().skip(1).all(|e| e.is_ascii_hexdigit())
        });
        let valid_ecl = item.ecl.as_ref().map_or(false, |e| match e.as_ref() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        });
        let valid_pid = item
            .pid
            .as_ref()
            .map_or(false, |e| e.chars().all(|e| e.is_numeric()) && e.len() == 9);
        let valid_cid = true;

        if valid_byr
            && valid_iyr
            && valid_eyr
            && valid_hgt
            && valid_hcl
            && valid_ecl
            && valid_pid
            && valid_cid
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
        (part_1(&input), part_2(&input))
    };
}
