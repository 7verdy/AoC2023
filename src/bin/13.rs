use aoc_2023::*;
use num::PrimInt;

use std::fs::read_to_string;

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    let patterns = data.split("\n\n").collect::<Vec<_>>();
    for pattern in patterns {
        let rows = pattern.lines().collect::<Vec<_>>();
        let mut cols: Vec<String> = vec![];
        for row in rows.clone() {
            for (i, c) in row.chars().enumerate() {
                if cols.len() <= i {
                    cols.push(String::new());
                }
                cols[i].push(c);
            }
        }
        let mut row_index: i64 = -1;
        for i in 0..rows.len() - 1 {
            if rows[i] == rows[i + 1] {
                row_index = i as i64;
                let mut left = i;
                let mut right = i + 1;
                while left > 0 && right < rows.len() - 1 {
                    left -= 1;
                    right += 1;
                    if rows[left] != rows[right] {
                        row_index = -1;
                        break;
                    }
                }
                if row_index != -1 {
                    break;
                }
            }
        }
        if row_index != -1 {
            total += 100 * (row_index as u64 + 1);
        }

        let mut col_index: i64 = -1;
        for i in 0..cols.len() - 1 {
            if cols[i] == cols[i + 1] {
                col_index = i as i64;
                let mut left = i;
                let mut right = i + 1;
                while left > 0 && right < cols.len() - 1 {
                    left -= 1;
                    right += 1;
                    if cols[left] != cols[right] {
                        col_index = -1;
                        break;
                    }
                }
                if col_index != -1 {
                    break;
                }
            }
        }
        if col_index != -1 {
            total += col_index as u64 + 1;
        }
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total: u64 = 0;
    // =========== END SETUP =============
    let patterns = data.split("\n\n").collect::<Vec<_>>();
    for pattern in patterns {
        let grid = pattern
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (rows, cols) = (grid.len(), grid[0].len());

        // Checking for vertical symmetry
        for c in 0..(cols - 1) {
            let mut smudges = 0;
            for cn in 0..=c {
                let (left, right) = ((c - cn) as i64, (c + 1 + cn) as i64);
                if 0 <= left && left < right && right < cols as i64 {
                    for r in 0..rows {
                        if grid[r][left as usize] != grid[r][right as usize] {
                            smudges += 1;
                        }
                    }
                }
            }
            if smudges == 1 {
                total += c as u64 + 1;
            }
        }
        // Checking for horizontal symmetry
        for r in 0..rows - 1 {
            let mut smudges = 0;
            for rn in 0..=r {
                let (top, bottom) = ((r - rn) as i64, (r + 1 + rn) as i64);
                if 0 <= top && top < bottom && bottom < rows as i64 {
                    for c in 0..cols {
                        if grid[top as usize][c] != grid[bottom as usize][c] {
                            smudges += 1;
                        }
                    }
                }
            }
            if smudges == 1 {
                total += 100 * (r as u64 + 1);
            }
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
    let day = 13; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    // let _ = upload_solution(year, day, 1, p1_res.to_string());
    // let _ = upload_solution(year, day, 2, p2_res.to_string());
}
