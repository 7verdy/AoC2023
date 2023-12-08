mod aoc_utils;

use std::fs::read_to_string;
use num::integer::lcm;


fn parse_input(data: String) -> Vec<(String, (String, String))> {
    let mut nodes = Vec::new();
    for line in data.lines().skip(2) {
        // AAA = (BBB, CCC)
        let (names, values) = line.split_once(" = ").unwrap();
        let (mut lvalue, mut rvalue) = values.split_once(", ").unwrap();
        lvalue = lvalue.trim_start_matches('(');
        rvalue = rvalue.trim_end_matches(')');
        nodes.push((names.to_string(), (lvalue.to_string(), rvalue.to_string())));
    }
    nodes
}


fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    let nodes = parse_input(data.clone());
    let start = nodes.iter().find(|(name, _)| name == "AAA").unwrap().0.clone();
    let end = nodes.iter().find(|(name, _)| name == "ZZZ").unwrap().0.clone();
    let mut current = start.clone();
    let mut done = false;
    let commands = data.lines().nth(0).unwrap();
    while true {
        for c in commands.chars() {
            if current == end {
                done = true;
                break;
            }
            match c {
                'L' => {
                    current = nodes.iter().find(|(name, _)| *name == current).unwrap().1.0.clone();
                    total += 1;
                },
                'R' => {
                    current = nodes.iter().find(|(name, _)| *name == current).unwrap().1.1.clone();
                    total += 1;
                },
                _ => {
                    println!("Unknown command: {}", c);
                }
            }
        }
        if done {
            break;
        }
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 1;
    // =========== END SETUP =============
    let nodes = parse_input(data.clone());
    let starts = nodes.iter().filter(|(name, _)| name.ends_with("A")).map(|(name, _)| name.clone()).collect::<Vec<String>>();
    let ends = nodes.iter().filter(|(name, _)| name.ends_with("Z")).map(|(name, _)| name.clone()).collect::<Vec<String>>();
    let mut currents = starts.clone();
    let commands = data.lines().nth(0).unwrap();

    for mut node in starts {
        let mut steps = 0;
        let mut idx = 0;
        while true {
            if node.chars().last().unwrap() == 'Z' {
                break;
            }
            steps += 1;
            match commands.chars().nth(idx).unwrap() {
                'L' => {
                    node = nodes.iter().find(|(name, _)| *name == node).unwrap().1.0.clone();
                },
                'R' => {
                    node = nodes.iter().find(|(name, _)| *name == node).unwrap().1.1.clone();
                },
                _ => {
                    println!("Unknown command: {}", commands.chars().nth(idx).unwrap());
                }
            }
            idx = (idx + 1) % commands.len();
        }
        // total is the least common multiple of all steps
        total = lcm(total, steps);
    }


    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
    total
}



fn main() {
    let filename = "input.txt".to_string();
    let day = 8; // TO MODIFY
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = aoc_utils::dl_file_from_url(input_url);

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = aoc_utils::upload_solution(year, day, 1, p1_res.to_string());
    let _ = aoc_utils::upload_solution(year, day, 2, p2_res.to_string());
}
