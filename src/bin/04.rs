use std::fs::read_to_string;

fn first_part(filename: String) {
    let mut total_points = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (winning_numbers, my_numbers) = line.split_at(line.find('|').unwrap());
        let trimmed_winning_numbers = winning_numbers.split(":").collect::<Vec<_>>()[1]
            .trim()
            .to_owned();
        let my_numbers_vec = my_numbers
            .chars()
            .skip(2)
            .collect::<String>()
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .to_owned();
        let mut local_points = 0;
        for winning_number in trimmed_winning_numbers.split_whitespace() {
            if my_numbers_vec.contains(&winning_number.parse::<u32>().unwrap()) {
                if local_points == 0 {
                    local_points = 1;
                } else {
                    local_points *= 2;
                }
            }
        }
        total_points += local_points;
    }
    println!("The result is : {}", total_points);
}


fn second_part(filename: String) {
    let mut total_cards = 0;
    let mut extra_cards = vec![0; 1];

    for (idx, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let (winning_numbers, my_numbers) = line.split_at(line.find('|').unwrap());
        let trimmed_winning_numbers = winning_numbers.split(":").collect::<Vec<_>>()[1]
            .trim()
            .to_owned();
        let my_numbers_vec = my_numbers
            .chars()
            .skip(2)
            .collect::<String>()
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .to_owned();
        let mut local_points = 0;
        for winning_number in trimmed_winning_numbers.split_whitespace() {
            if my_numbers_vec.contains(&winning_number.parse::<u32>().unwrap()) {
                local_points += 1;
            }
        }
        for i in 1..=local_points {
            if extra_cards.len() <= idx + i {
                extra_cards.push(1 * if extra_cards[idx] > 0 { extra_cards[idx] + 1} else { 1 });
            } else {
                extra_cards[idx + i] += 1 * if extra_cards[idx] > 0 { extra_cards[idx] + 1} else { 1 };
            }
        }
        if local_points == 0 {
            extra_cards.push(0);
        }
        total_cards += (extra_cards[idx] + 1);
    }
    println!("The result is : {}", total_cards);
}

fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}04_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}
