use std::fs::read_to_string;

fn first_part(filename: String) {
    let data = read_to_string(filename).unwrap();
    let seeds = data
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut locations = seeds;

    let mut n = 0;
    while n < data.lines().count() {
        let mut section: String = String::new();
        if data.lines().nth(n).unwrap().is_empty() {
            n += 1;
            continue;
        }
        while n < data.lines().count() && !data.lines().nth(n).unwrap().is_empty() {
            section.push_str(&(data.lines().nth(n).unwrap().to_owned() + "\n"));
            n += 1;
        }

        for seed in locations.clone().into_iter() {
            for line in section.lines() {
                if line.chars().nth(0).unwrap().is_digit(10) {
                    let tmp = line
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    let dest_start = tmp[0];
                    let source_start = tmp[1];
                    let range = tmp[2];
                    let idx = locations.clone().iter().position(|&x| x == seed);
                    if idx.is_none() {
                        continue;
                    }
                    let idx = idx.unwrap();
                    if seed >= source_start && seed <= source_start + range {
                        locations[idx] = dest_start + (seed - source_start);
                    } else {
                        locations[idx] = seed;
                    }
                }
            }
        }
    }
    println!("The result is: {}", locations.iter().min().unwrap());
}

fn process_map(mut seeds: Vec<(i64, i64)>, full_lines: Vec<String>, idx: u32) -> (Vec<(i64, i64)>, u32) {

    let mut result: Vec<(i64, i64)> = vec![];
    let mut remain = seeds.clone();
    let lines = full_lines.clone().into_iter().skip(idx as usize).collect::<Vec<String>>();
    let mut current_idx = idx;

    for line in lines {
        current_idx += 1;
        if line == "" || !line.chars().nth(0).unwrap().is_digit(10) {
            break;
        }
        let tmp = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let (dest, src, range) = (tmp[0], tmp[1], tmp[2]);
        let delta = dest - src;
        let end = src + range;

        remain = vec![];

        for (start, stop) in seeds.clone() {
            if src <= start && start < stop && stop <= end {
                result.push((start + delta, stop + delta));
            } else if start < src && src < stop && stop <= end {
                remain.push((start, src));
                result.push((src + delta, stop + delta));
            } else if src <= start && start < end && end < stop {
                remain.push((end, stop));
                result.push((start + delta, end + delta));
            } else if start < src && src <= end && end < stop {
                remain.push((start, src));
                remain.push((end, stop));
                result.push((src + delta, end + delta));
            } else {
                remain.push((start, stop));
            }
        }
        seeds = remain.clone();
    }
    result.append(&mut remain);
    (result, current_idx)
}

fn second_part(filename: String) {
    let data = read_to_string(filename).unwrap();
    let mut seeds: Vec<(i64, i64)> = vec![];
    let mut idx = 3;

    let first_line_seeds = data
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    for i in (0..first_line_seeds.len() - 1).step_by(2) {
        seeds.push((first_line_seeds[i], first_line_seeds[i] + first_line_seeds[i + 1]));
    }

    let lines = data.lines().skip(1).map(|x| x.to_owned()).collect::<Vec<String>>();

    for _ in lines {
        (seeds, idx) = process_map(seeds, data.lines().map(|x| x.to_owned()).collect::<Vec<String>>(), idx);
    }
    let min = seeds.iter().min_by_key(|x| x.0).unwrap();
    println!("The result is: {}", min.0);
}

fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}05_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}
