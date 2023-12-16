use aoc_2023::*;

use std::collections::HashMap;
use std::fs::read_to_string;

fn compute_beam(
    grid: Vec<Vec<char>>,
    stack: &mut Vec<((i32, i32), (i8, i8))>,
    (x, y): (i32, i32),
) -> HashMap<(i32, i32), (i8, i8)> {
    if y < 0 || x < 0 || y >= grid.len() as i32 || x >= grid[0].len() as i32 {
        return HashMap::new();
    }

    let mut result_path: HashMap<(i32, i32), (i8, i8)> = HashMap::new();

    while let Some(((x, y), (dx, dy))) = stack.pop() {
        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
            continue;
        }
        if result_path.contains_key(&(x, y)) && result_path[&(x, y)] == (dx, dy) {
            continue;
        }
        let (dx, dy) = (dx, dy);

        result_path.insert((x, y), (dx, dy));

        match grid[x as usize][y as usize] {
            '/' => match (dx, dy) {
                (1, 0)  => stack.push(((x, y - 1), (0, -1))),
                (-1, 0) => stack.push(((x, y + 1), (0, 1))),
                (0, 1)  => stack.push(((x - 1, y), (-1, 0))),
                (0, -1) => stack.push(((x + 1, y), (1, 0))),
                _ => unreachable!(),
            },
            '\\' => match (dx, dy) {
                (1, 0)  => stack.push(((x, y + 1), (0, 1))),
                (-1, 0) => stack.push(((x, y - 1), (0, -1))),
                (0, 1)  => stack.push(((x + 1, y), (1, 0))),
                (0, -1) => stack.push(((x - 1, y), (-1, 0))),
                _ => unreachable!(),
            },
            '|' if dy != 0 => {
                stack.push(((x - 1, y), (-1, 0)));
                stack.push(((x + 1, y), (1, 0)));
            },
            '-' if dx != 0 => {
                stack.push(((x, y - 1), (0, -1)));
                stack.push(((x, y + 1), (0, 1)));
            },
            _ => { stack.push(((x + dx as i32, y + dy as i32), (dx, dy))); },
        }
    }
    result_path
}

fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    // =========== END SETUP =============
    let grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut stack: Vec<((i32, i32), (i8, i8))> = vec![((0, 0), (0, 1))];
    let (x, y) = (0, 0);
    let result = compute_beam(grid, &mut stack, (x, y));

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", result.len());
    result.len() as u64
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut max: u64 = 0;
    // =========== END SETUP =============
    let grid = data
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let top = (0..grid[0].len() as i32).map(|x| ((0, x), (1, 0)));
    let bottom = (0..grid[0].len() as i32).map(|x| ((grid.len() as i32 - 1, x), (-1, 0)));
    let left = (0..grid.len() as i32).map(|x| ((x, 0), (0, 1)));
    let right = (0..grid.len() as i32).map(|x| ((x, grid[0].len() as i32 - 1), (0, -1)));

    let all_starts: Vec<((i32, i32), (i8, i8))> = top.chain(bottom).chain(left).chain(right).collect();

    for ((x,y), direction) in all_starts {
        let mut stack: Vec<((i32, i32), (i8, i8))> = vec![((x, y), direction)];
        let result = compute_beam(grid.clone(), &mut stack, (x, y));
        if result.len() as u64 > max {
            max = result.len() as u64;
        }
    }

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", max);
    max
}

fn main() {
    let cwd = format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/src/bin/"
    );
    let day = 16; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
