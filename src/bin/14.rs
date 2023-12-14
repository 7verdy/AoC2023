use aoc_2023::*;
use num::{Float, PrimInt};

use std::collections::HashMap;
use std::fs::read_to_string;

fn roll_north(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    let (mut row, mut col) = (0, 0);
    while row < rows {
        while col < cols {
            if grid[row][col] == 'O' {
                for i in (0..row).rev() {
                    if grid[i][col] == '.' {
                        grid[i][col] = 'O';
                        grid[i + 1][col] = '.';
                    } else {
                        break;
                    }
                }
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }
}

fn roll_west(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    let (mut row, mut col) = (0, 0);
    while row < rows {
        while col < cols {
            if grid[row][col] == 'O' {
                for i in (0..col).rev() {
                    if grid[row][i] == '.' {
                        grid[row][i] = 'O';
                        grid[row][i + 1] = '.';
                    } else {
                        break;
                    }
                }
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }
}

fn roll_south(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    let (mut row, mut col) = (rows as i64 - 1, 0);
    while row > -1 {
        while col < cols {
            if grid[row as usize][col] == 'O' {
                for i in row as usize + 1..rows {
                    if grid[i][col] == '.' {
                        grid[i][col] = 'O';
                        grid[i - 1][col] = '.';
                    } else {
                        break;
                    }
                }
            }
            col += 1;
        }
        row -= 1;
        col = 0;
    }
}

fn roll_east(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize) {
    let (mut row, mut col) = (0, cols as i64 - 1);
    while row < rows {
        while col > -1 {
            if grid[row][col as usize] == 'O' {
                for i in col as usize + 1..cols {
                    if grid[row][i] == '.' {
                        grid[row][i] = 'O';
                        grid[row][i - 1] = '.';
                    } else {
                        break;
                    }
                }
            }
            col -= 1;
        }
        row += 1;
        col = cols as i64 - 1;
    }
}

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    let mut grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (rows, cols) = (grid.len(), grid[0].len());

    roll_north(&mut grid, rows, cols);

    for (idx, row) in grid.iter().enumerate() {
        total += row.iter().filter(|c| **c == 'O').count() as u64 * (grid.len() - idx) as u64;
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
    let mut grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut repeats: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut loads: Vec<u64> = Vec::new();
    let mut iter = 0;
    while iter < 1000000000 {
        iter += 1;
        roll_north(&mut grid, rows, cols);
        roll_west(&mut grid, rows, cols);
        roll_south(&mut grid, rows, cols);
        roll_east(&mut grid, rows, cols);

        let mut load = 0;
        for (idx, row) in grid.iter().enumerate() {
            load += row.iter().filter(|c| **c == 'O').count() as u64 * (grid.len() - idx) as u64;
        }
        loads.push(load);

        if repeats.contains_key(&load) {
            repeats.get_mut(&load).unwrap().push(iter);
        } else {
            repeats.insert(load, vec![iter]);
        }

        if repeats[&load].len() > 5 {
            let cycle_length =
                repeats[&load][repeats[&load].len() - 1] - repeats[&load][repeats[&load].len() - 2];
            if cycle_length == repeats[&load][repeats[&load].len() - 2]
                - repeats[&load][repeats[&load].len() - 3]
            {
                let amount: f64 = ((1000000000 - iter) as f64 / cycle_length as f64).floor();
                iter += amount as u64 * cycle_length;
            }
        }
    }

    for (idx, row) in grid.iter().enumerate() {
            total += row.iter().filter(|c| **c == 'O').count() as u64 * (grid.len() - idx) as u64;
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
    let day = 14; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
