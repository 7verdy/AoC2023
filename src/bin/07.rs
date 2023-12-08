use aoc_2023::*;

use std::fs::read_to_string;

fn parse_input(data: String) -> Vec<(String, u64)> {
    let mut res = Vec::new();
    for line in data.lines() {
        let mut line = line.split_whitespace();
        let mut hands = line.next().unwrap().to_string();
        let bid = line.next().unwrap().to_string();
        res.push((hands, bid.parse::<u64>().unwrap()));
    }
    res
}

fn get_score_s1(hand: String) -> (u64, Vec<usize>) {
    let order = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T',
                    'J', 'Q', 'K', 'A'];
    let mut strengths = vec![];
    strengths.push(vec![1, 1, 1, 1, 1]);
    strengths.push(vec![1, 1, 1, 2]);
    strengths.push(vec![1, 2, 2]);
    strengths.push(vec![1, 1, 3]);
    strengths.push(vec![2, 3]);
    strengths.push(vec![1, 4]);
    strengths.push(vec![5]);

    let orders = hand.chars().map(|x| order.iter().position(|&r| r == x).unwrap()).collect::<Vec<_>>();
    let mut occ = orders.iter().fold(vec![0; 13], |mut acc, &x| {acc[x] += 1; acc});
    occ = occ.iter().filter(|&x| *x != 0).map(|&x| x).collect::<Vec<_>>();
    occ.sort();
    (strengths.iter().position(|x| *x == occ).unwrap() as u64, orders)
}

fn get_score_s2(hand: String) -> (u64, Vec<usize>) {
    let order = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T',
                    'Q', 'K', 'A'];
    let mut strengths = vec![];
    strengths.push(vec![1, 1, 1, 1, 1]);
    strengths.push(vec![1, 1, 1, 2]);
    strengths.push(vec![1, 2, 2]);
    strengths.push(vec![1, 1, 3]);
    strengths.push(vec![2, 3]);
    strengths.push(vec![1, 4]);
    strengths.push(vec![5]);
    let mut best_alternative = 0;

    for card in order[1..].iter() {
        let new_hand = hand.replace('J', card.to_string().as_str());

        let cur_orders = new_hand.chars().map(|x| order.iter().position(|&r| r == x).unwrap()).collect::<Vec<_>>();
        let mut occ = cur_orders.iter().fold(vec![0; 13], |mut acc, &x| {acc[x] += 1; acc});
        occ = occ.iter().filter(|&x| *x != 0).map(|&x| x).collect::<Vec<_>>();
        occ.sort();
        let strength = strengths.iter().position(|x| *x == occ).unwrap() as u64;
        if strength > best_alternative {
            best_alternative = strength;
        }
    }
    let orders = hand.chars().map(|x| order.iter().position(|&r| r == x).unwrap()).collect::<Vec<_>>();
    (best_alternative, orders)
}


fn part_one(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;

    // =========== END SETUP =============
    let mut hands_bids = parse_input(data);
    hands_bids.sort_by(|a, b| get_score_s1(a.0.clone()).cmp(&get_score_s1(b.0.clone())));
    for (i, (_, bid)) in hands_bids.iter().enumerate() {
        total += bid * (i as u64 + 1);
    }

    // =========== END STEP 1 ============
    println!("The result for step 1 is: {}", total);
    total
}

fn part_two(filename: String) -> u64 {
    // ============= SETUP ===============
    let data = read_to_string(filename).expect("Something went wrong reading the file");
    let mut total = 0;
    // =========== END SETUP =============
    let mut hands_bids = parse_input(data);
    hands_bids.sort_by(|a, b| get_score_s2(a.0.clone()).cmp(&get_score_s2(b.0.clone())));
    for (i, (_, bid)) in hands_bids.iter().enumerate() {
        total += bid * (i as u64 + 1);
    }

    // =========== END STEP 2 ============
    println!("The result for step 2 is: {}", total);
    total
}



fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let day = 7; // TO MODIFY
    let filename = format!("{}{:02}_{}", cwd, day, "input.txt");
    let year = 2023; // TO MODIFY
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let _ = dl_file_from_url(input_url, filename.clone());

    let p1_res = part_one(filename.clone());
    let p2_res = part_two(filename.clone());

    let _ = upload_solution(year, day, 1, p1_res.to_string());
    let _ = upload_solution(year, day, 2, p2_res.to_string());
}
