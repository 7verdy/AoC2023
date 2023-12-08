use std::fs::read_to_string;

fn first_part(filename: String) {
    let data = read_to_string(filename).unwrap();

    let times = data
        .lines().nth(0).unwrap().split(": ").nth(1).unwrap()
        .split_whitespace().map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let distances = data
        .lines().nth(1).unwrap().split(": ").nth(1).unwrap()
        .split_whitespace().map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

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
    println!("The result is: {}", total_ways);
}

fn second_part(filename: String) {
    let data = read_to_string(filename).unwrap();
    let time = data
        .lines().nth(0).unwrap().split(": ").nth(1).unwrap()
        .split_whitespace().collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = data
        .lines().nth(1).unwrap().split(": ").nth(1).unwrap()
        .split_whitespace().collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut total_ways = 0;
    for x in 0..time {
        let local_distance: u64 = (time - x) * x;
        if local_distance > distance {
            total_ways += 1;
        }
    }
    println!("The result is: {}", total_ways);
}
fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}06_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}