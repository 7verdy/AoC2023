use std::fs::read_to_string;

use num::{PrimInt, Float};

fn first_part(filename: String) -> u64 {
    read_to_string(filename)
        .unwrap()
        .split_once("\n")
        .map(|(times, distances)| {
            (
                times
                    .chars()
                    .skip_while(|&c| c != ' ')
                    .collect::<String>()
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap().to_owned())
                    .collect::<Vec<_>>(),
                distances
                    .chars()
                    .skip_while(|&c| c != ' ')
                    .collect::<String>()
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap().to_owned())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(times, distances)| {
            let mut total_ways = 1;
            for (idx, time) in times.iter().enumerate() {
                let mut local_ways = 0;
                for x in 0..*time {
                    let distance = (time - x) * x;
                    if distance > distances[idx] {
                        local_ways += 1;
                    }
                }
                total_ways *= local_ways;
            }
            total_ways
        })
        .unwrap()
}

fn second_part(filename: String) -> u64 {
    read_to_string(filename)
        .unwrap()
        .split_once("\n")
        .map(|(times, distances)| {
            (
                times
                    .chars()
                    .skip_while(|&c| c != ' ')
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<String>(),
                distances
                    .chars()
                    .skip_while(|&c| c != ' ')
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<String>(),
            )
        })
        .map(|(time, distance)| {
            let time = time.parse::<u64>().unwrap();
            let distance = distance.parse::<u64>().unwrap();
            let mut total_ways = 0;
            for x in 0..time {
                let curr_distance = (time - x) * x;
                if curr_distance > distance {
                    total_ways += 1;
                }
            }
            total_ways
        })
        .unwrap()
}
fn main() {
    let cwd = format!(
        "{}{}",
        std::env::current_dir().unwrap().display(),
        "/src/bin/"
    );
    let filename = format!("{}06_{}", cwd, "input.txt");
    let p1_res = first_part(filename.clone());
    println!("The result for step 1 is: {}", p1_res);
    let p2_res = second_part(filename.clone());
    println!("The result for step 2 is: {}", p2_res);
}
