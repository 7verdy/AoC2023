use aoc_2023::*;
use num::{Float, PrimInt};

use std::collections::HashMap;
use std::fs::read_to_string;

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    data.split(",").for_each(|x| {
        let mut local = 0;
        x.chars().for_each(|y| {
            local += y as u64;
            local *= 17;
            local %= 256;
        });
        total += local;
    });

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total: u64 = 0;
    // =========== END SETUP =============
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    data.split(",").for_each(|x| {
        let mut local = 0;
        let name = x.chars().take_while(|&y| y != '=' && y != '-').collect::<String>();
        name.chars().for_each(|y| {
            local += y as u64;
            local *= 17;
            local %= 256;
        });

        if x.chars().rev().nth(1).unwrap() == '=' {
            let lens = x.chars().last().unwrap().to_digit(10).unwrap();
            if boxes[local as usize].iter().any(|y| y.0 == name) {
                for box_ in boxes.iter_mut() {
                    box_.iter_mut()
                        .filter(|y| y.0 == name)
                        .for_each(|y| y.1 = lens);
                }
            } else {
                boxes[local as usize].push((name, lens));
            }
        } else if x.chars().last().unwrap() == '-' {
            for box_ in boxes.iter_mut() {
                box_.retain(|y| y.0 != name);
            }
        }
    });

    for (idx, box_) in boxes.iter().enumerate() {
        for (idx2, (_, lens)) in box_.iter().enumerate() {
            total += (idx as u64 + 1) * (idx2 as u64 + 1) * *lens as u64;
        }
    }

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
    total
}

fn main() {
    let cwd = format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/src/bin/"
    );
    let day = 15; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
