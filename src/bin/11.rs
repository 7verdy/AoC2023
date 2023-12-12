use aoc_2023::*;

use std::fs::read_to_string;

fn part_one(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &value)| value == '#')
                .map(move |(col_i, _)| (row_i as isize, col_i as isize))
        })
        .collect::<Vec<(isize, isize)>>();

    let vr = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&value| value == '.'))
        .map(|(row_i, _)| row_i)
        .collect::<Vec<usize>>();

    let vc = (0..grid[0].len())
        .filter(|&col| grid.iter().all(|row| row[col] == '.'))
        .collect::<Vec<usize>>();

    galaxies.iter().enumerate().for_each(|(i, &(x, y))| {
        galaxies.iter().skip(i + 1).for_each(|&(dx, dy)| {
            let mut manhattan_distance =
                (x as isize - dx as isize).abs() + (y as isize - dy as isize).abs();

            for j in x.min(dx)..x.max(dx) {
                if vr.contains(&(j as usize)) {
                    manhattan_distance += 1;
                }
            }

            for k in y.min(dy)..y.max(dy) {
                if vc.contains(&(k as usize)) {
                    manhattan_distance += 1;
                }
            }

            total += manhattan_distance;
        });
    });

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total as i64
}

fn part_two(filename: String) -> i64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &value)| value == '#')
                .map(move |(col_i, _)| (row_i as isize, col_i as isize))
        })
        .collect::<Vec<(isize, isize)>>();

    let vr = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&value| value == '.'))
        .map(|(row_i, _)| row_i)
        .collect::<Vec<usize>>();

    let vc = (0..grid[0].len())
        .filter(|&col| grid.iter().all(|row| row[col] == '.'))
        .collect::<Vec<usize>>();

    galaxies.iter().enumerate().for_each(|(i, &(x, y))| {
        galaxies.iter().skip(i + 1).for_each(|&(dx, dy)| {
            let mut manhattan_distance =
                (x as isize - dx as isize).abs() + (y as isize - dy as isize).abs();

            for j in x.min(dx)..x.max(dx) {
                if vr.contains(&(j as usize)) {
                    manhattan_distance += 1000000 - 1;
                }
            }

            for k in y.min(dy)..y.max(dy) {
                if vc.contains(&(k as usize)) {
                    manhattan_distance += 1000000 - 1;
                }
            }

            total += manhattan_distance;
        });
    });

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
    total as i64
}


fn main() {
    let cwd = format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/src/bin/"
    );
    let day = 11; // TO MODIFY
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

    // let _ = upload_solution(year, day, 1, p1_res.to_string());
    // let _ = upload_solution(year, day, 2, p2_res.to_string());
}
