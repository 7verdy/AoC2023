use aoc_2023::*;

use std::fs::read_to_string;
use std::collections::HashMap;
use once_cell::sync::Lazy;


static mut DP: Lazy<HashMap<(usize, usize, usize), u64>> = Lazy::new(|| {
    let map = HashMap::new();
    map
});

fn process_marks(row: &str, blocks: Vec<u32>, i: usize, bi: usize, idx: usize) -> u64 {
    let key = (i, bi, idx);
    unsafe {
        if DP.contains_key(&key) {
            return DP.get(&key).unwrap().clone();
        }
    }
    if i == row.chars().count() {
        if bi == blocks.len() && idx == 0 {
            return 1;
        } else if bi == blocks.len() - 1 && idx == blocks[bi] as usize {
            return 1;
        } else {
            return 0;
        }
    }
    let mut answer = 0;
    for c in ['.', '#'].iter() {
        if row.chars().nth(i).unwrap() == *c || row.chars().nth(i).unwrap() == '?' {
            if *c == '.' && idx == 0 {
                answer += process_marks(row, blocks.clone(), i + 1, bi, 0);
            } else if *c == '.' && idx > 0 && bi < blocks.len() && blocks[bi] == idx as u32 {
                answer += process_marks(row, blocks.clone(), i + 1, bi + 1, 0);
            } else if *c == '#' {
                answer += process_marks(row, blocks.clone(), i + 1, bi, idx + 1);
            }
        }
    }
    unsafe {
        DP.insert(key, answer);
    }
    answer
}

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    let rows = data
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for row in &rows {
        let blocks = row[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        unsafe {
            DP.clear();
        }
        total += process_marks((*row)[0], blocks, 0, 0, 0);
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total as u64
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total: u64 = 0;
    // =========== END SETUP =============
    let rows = data
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for row in &rows {
        let fd_row = format!("{}?{}?{}?{}?{}", row[0], row[0], row[0], row[0], row[0]);

        let blocks = row[1]
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let fd_blocks = blocks.repeat(5);
        unsafe {
            DP.clear();
        }
        total += process_marks(fd_row.as_str(), fd_blocks, 0, 0, 0);
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
    let day = 12; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
