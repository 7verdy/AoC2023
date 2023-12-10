use aoc_2023::*;

use std::fs::read_to_string;

fn check_neighbours(grid: &Vec<Vec<char>>, distances: &mut Vec<Vec<i64>>, i: usize, j: usize) {
    // Right: '7' or '-' or 'J'
    if j + 1 < grid[i].len()
        && (grid[i][j + 1] == '7' || grid[i][j + 1] == '-' || grid[i][j + 1] == 'J')
    {
        if grid[i][j] == '-' || grid[i][j] == 'F' || grid[i][j] == 'L' || grid[i][j] == 'S' {
            if distances[i][j + 1] == 0 || distances[i][j + 1] > distances[i][j] + 1 {
                distances[i][j + 1] = distances[i][j] + 1;
                check_neighbours(grid, distances, i, j + 1);
            }
        }
    }
    // Left: 'F' or '-' or 'L'
    if j > 0 && (grid[i][j - 1] == 'F' || grid[i][j - 1] == '-' || grid[i][j - 1] == 'L') {
        if grid[i][j] == '-' || grid[i][j] == '7' || grid[i][j] == 'J' || grid[i][j] == 'S' {
            if distances[i][j - 1] == 0 || distances[i][j - 1] > distances[i][j] + 1 {
                distances[i][j - 1] = distances[i][j] + 1;
                check_neighbours(grid, distances, i, j - 1);
            }
        }
    }
    // North: '7' or '|' or 'F'
    if i > 0 && (grid[i - 1][j] == '7' || grid[i - 1][j] == '|' || grid[i - 1][j] == 'F') {
        if grid[i][j] == '|' || grid[i][j] == 'L' || grid[i][j] == 'J' || grid[i][j] == 'S' {
            if distances[i - 1][j] == 0 || distances[i - 1][j] > distances[i][j] + 1 {
                distances[i - 1][j] = distances[i][j] + 1;
                check_neighbours(grid, distances, i - 1, j);
            }
        }
    }
    // South: '|' or 'J' or 'L'
    if i + 1 < grid.len()
        && (grid[i + 1][j] == '|' || grid[i + 1][j] == 'J' || grid[i + 1][j] == 'L')
    {
        if grid[i][j] == '|' || grid[i][j] == 'F' || grid[i][j] == '7' || grid[i][j] == 'S' {
            if distances[i + 1][j] == 0 || distances[i + 1][j] > distances[i][j] + 1 {
                distances[i + 1][j] = distances[i][j] + 1;
                check_neighbours(grid, distances, i + 1, j);
            }
        }
    }
}

fn part_one(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============

    let grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut distances = vec![vec![0; grid[0].len()]; grid.len()];
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
        }
    }

    check_neighbours(&grid, &mut distances, start.0, start.1);

    for i in 0..distances.len() {
        for j in 0..distances[i].len() {
            if distances[i][j] > total {
                total = distances[i][j];
            }
        }
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn mark_enclosed(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let mut count = 0;
    if grid[i][j] == 'S' || grid[i][j] != '.' {
        return;
    }
    for k in (0..j).rev() {
        if grid[i][k] == '|' || grid[i][k] == 'L' || grid[i][k] == 'J' {
            count += 1;
        }
    }
    if count % 2 == 1 {
        grid[i][j] = 'I';
    } else {
        grid[i][j] = '.';
    }
}

fn part_two(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============
    let mut grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut distances = vec![vec![0; grid[0].len()]; grid.len()];
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
            }
        }
    }
    check_neighbours(&grid, &mut distances, start.0, start.1);

    // get indexes of max
    let (max_i, max_j) = distances
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.iter().enumerate().max_by_key(|&(_, y)| y).unwrap()))
        .max_by_key(|&(_, (_, y))| y)
        .unwrap();
    // Remove walls not in the loop
    for i in 0..distances.len() {
        for j in 0..distances[i].len() {
            if (i, j) == start || grid[i][j] == '.' {
                continue;
            }
            let (mut has_prev, mut has_next) = (false, false);
            if i > 0 {
                if distances[i - 1][j] == distances[i][j] - 1 {
                    has_prev = true;
                } else if distances[i - 1][j] == distances[i][j] + 1 {
                    has_next = true;
                }
            }
            if i + 1 < distances.len() {
                if distances[i + 1][j] == distances[i][j] - 1 {
                    has_prev = true;
                } else if distances[i + 1][j] == distances[i][j] + 1 {
                    has_next = true;
                }
            }
            if j > 0 {
                if distances[i][j - 1] == distances[i][j] - 1 {
                    has_prev = true;
                } else if distances[i][j - 1] == distances[i][j] + 1 {
                    has_next = true;
                }
            }
            if j + 1 < distances[i].len() {
                if distances[i][j + 1] == distances[i][j] - 1 {
                    has_prev = true;
                } else if distances[i][j + 1] == distances[i][j] + 1 {
                    has_next = true;
                }
            }
            // if not either and not max
            if (!has_prev || !has_next) && (i, j) != (max_i, max_j.0) {
                grid[i][j] = '.';
            }
        }
    }

    // Mark enclosed areas
    for i in 0..distances.len() {
        for j in 0..distances[i].len() {
            mark_enclosed(&mut grid, i, j);
            if grid[i][j] == 'I' {
                total += 1;
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
    let day = 10; // TO MODIFY
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
