use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn check_adjacency(s: Vec<String>, number: String, i: usize, j: usize) -> bool {
    let digits = number.chars().count();
    if i > 0 {
        // Top left or top right
        if (j > 0 && s[i - 1].chars().nth(j - 1).unwrap() != '.')
            || (j < s[i].chars().count() - 1 - digits
                && s[i - 1].chars().nth(j + digits).unwrap() != '.')
        {
            return true;
        }
        // Top
        for x in 0..digits {
            if s[i - 1].chars().nth(j + x).unwrap() != '.' {
                return true;
            }
        }
    }
    if i < s.len() - 1 {
        // Bottom left or bottom right
        if (j > 0 && s[i + 1].chars().nth(j - 1).unwrap() != '.')
            || (j < s[i].chars().count() - 1 - digits
                && s[i + 1].chars().nth(j + digits).unwrap() != '.')
        {
            return true;
        }
        // Bottom
        for x in 0..digits {
            if s[i + 1].chars().nth(j + x).unwrap() != '.' {
                return true;
            }
        }
    }
    // Left
    if j > 0 {
        if s[i].chars().nth(j - 1).unwrap() != '.' {
            return true;
        }
    }
    // Right
    if j < s[i].chars().count() - 1 - digits {
        if s[i].chars().nth(j + digits).unwrap() != '.' {
            return true;
        }
    }

    false
}

fn parse_schematic(s: Vec<String>) -> u32 {
    let mut sum_valid_numbers = 0;
    for i in 0..s.len() {
        let mut j = 0;
        while j < s[i].chars().count() {
            if !s[i].chars().nth(j).unwrap().is_digit(10) {
                j += 1;
                continue;
            }
            let number = s[i]
                .chars()
                .skip(j)
                .take_while(|&c| c.is_digit(10))
                .collect::<String>();
            if check_adjacency(s.clone(), number.clone(), i, j) {
                sum_valid_numbers += number.parse::<u32>().unwrap();
            }
            j += number.chars().count();
        }
    }
    sum_valid_numbers
}

fn first_part(filename: String) {
    let file = File::open(filename).expect("Error in reading file");
    let reader = io::BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();

    match lines {
        Ok(lines) => {
            let res = parse_schematic(lines);
            println!("The result is: {}", res);
        }
        Err(err) => eprintln!("Error reading files: {}", err),
    }
}


fn get_adjacency(s: Vec<String>, number: String, i: usize, j: usize) -> (usize, usize) {
    let digits = number.chars().count();
    if i > 0 {
        // Top left or top right
        if j > 0 && s[i - 1].chars().nth(j - 1).unwrap() == '*' {
            return (i - 1, j - 1);
        }
        if j < s[i].chars().count() - 1 - digits
            && s[i - 1].chars().nth(j + digits).unwrap() == '*'
        {
            return (i - 1, j + digits);
        }
        // Top
        for x in 0..digits {
            if s[i - 1].chars().nth(j + x).unwrap() == '*' {
                return (i - 1, j + x);
            }
        }
    }
    if i < s.len() - 1 {
        // Bottom left or bottom right
        if j > 0 && s[i + 1].chars().nth(j - 1).unwrap() == '*' {
            return (i + 1, j - 1);
        }
        if j < s[i].chars().count() - 1 - digits
            && s[i + 1].chars().nth(j + digits).unwrap() == '*'
        {
            return (i + 1, j + digits);
        }
        // Bottom
        for x in 0..digits {
            if s[i + 1].chars().nth(j + x).unwrap() == '*' {
                return (i + 1, j + x);
            }
        }
    }
    // Left
    if j > 0 {
        if s[i].chars().nth(j - 1).unwrap() == '*' {
            return (i, j - 1);
        }
    }
    // Right
    if j < s[i].chars().count() - 1 - digits {
        if s[i].chars().nth(j + digits).unwrap() == '*' {
            return (i, j + digits);
        }
    }

    ( s.len(), s[i].chars().count() )
}

fn parse_part_2(s: Vec<String>) -> u32 {
    let mut sum_valid_numbers = 0;
    let mut gear_parts: HashMap<String, String> = HashMap::new();

    for i in 0..s.len() {
        let mut j = 0;
        while j < s[i].chars().count() {
            if !s[i].chars().nth(j).unwrap().is_digit(10) {
                j += 1;
                continue;
            }
            let number = s[i]
                .chars()
                .skip(j)
                .take_while(|&c| c.is_digit(10))
                .collect::<String>();
            let (gear_y, gear_x) = get_adjacency(s.clone(), number.clone(), i, j);
            if gear_y != s.len() {
                let yow = gear_y.to_owned();
                let xow = gear_x.to_owned();
                let key = format!("{yow}-{xow}");
                if gear_parts.contains_key(&key) {
                    sum_valid_numbers += number.parse::<u32>().unwrap() * (*gear_parts.get(&key).unwrap()).parse::<u32>().unwrap();
                } else {
                    gear_parts.insert(key, number.clone());
                }
            }
            j += number.chars().count();
        }
    }
    sum_valid_numbers
}


fn second_part(filename: String) {
    let file = File::open(filename).expect("Error in reading file");
    let reader = io::BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();

    match lines {
        Ok(lines) => {
            let res = parse_part_2(lines);
            println!("The result is: {}", res);
        }
        Err(err) => eprintln!("Error reading files: {}", err),
    }
}
fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}03_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}
