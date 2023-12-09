use aoc_2023::*;

use std::fs::read_to_string;

fn part_one(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    for line in data.lines() {
        let mut values = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        while !values.iter().all(|&x| x == 0) {
            for i in 0..values.len() - 1 {
                values[i] = values[i + 1] - values[i];
            }
            total += values.pop().unwrap();
        }
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============
    for line in data.lines() {
        let mut values = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        values.reverse();
        while !values.iter().all(|&x| x == 0) {
            for i in 0..values.len() - 1 {
                values[i] = values[i + 1] - values[i];
            }
            total += values.pop().unwrap();
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
    let day = 9; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    match dl_file_from_url(input_url, filename.clone()) {
        Ok(_) => {
            println!("Input file downloaded");
        }
        Err(_) => {
            println!("Input file already exists");
        }
    }

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
