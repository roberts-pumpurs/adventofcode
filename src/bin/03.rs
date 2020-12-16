use std::env;
use std::fs;

use AdventOfCode::time_it;

#[derive(Debug, Clone)]
enum Terrain {
    Tree,
    Snow,
}

fn read_input() -> Vec<Vec<Terrain>> {
    let filename = "src/bin/_03_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents: Vec<Vec<Terrain>> = contents
        .split("\n")
        .filter(|x| *x != "")
        .map(|e| {
            e.chars()
                .map(|c| match c {
                    '.' => Terrain::Snow,
                    '#' => Terrain::Tree,
                    _ => Terrain::Snow,
                })
                .collect()
        })
        .collect();
    contents
}

fn puzzle_part(step_x: usize, step_y: usize, data: &Vec<Vec<Terrain>>) -> u32 {
    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;

    while let Some(row) = data.get(y) {
        println!("x {} y {}", x, y);
        let item = row.get(x).unwrap();
        match item {
            Terrain::Tree => tree_count += 1,
            _ => (),
        }
        y += step_y;
        if x + step_x > row.len() - 1 {
            let delta: i32 = row.len() as i32 - (x + step_x) as i32;
            println!("delta {}", delta);
            x = delta.abs() as usize;
        } else {
            x += step_x;
        }
    }

    tree_count
}

fn main() {
    let input = read_input();
    time_it! {
        let p1 = puzzle_part(3, 1, &input);
        let p2_1 = puzzle_part(1, 1, &input);
        let p2_3 = puzzle_part(5, 1, &input);
        let p2_4 = puzzle_part(7, 1, &input);
        let p2_5 = puzzle_part(1, 2, &input);
        (p1, p1 * p2_1 * p2_3 * p2_4 * p2_5)
    };
}
