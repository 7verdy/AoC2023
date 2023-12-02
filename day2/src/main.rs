use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;

fn first_part() {
    let mut sum_possible_game_ids = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let (mut game_id, mut rest) = line.split_at(line.find(':').unwrap());
        rest = &rest[2..];
        game_id = game_id.split(' ').nth(1).unwrap();

        let nbs_and_colors = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        let mut respect_limit = true;

        while rest.chars().count() > 0 {
            let current_set;
            (current_set, rest) = rest.split_at(rest.find(';').unwrap_or(rest.chars().count()));
            if !rest.is_empty() {
                rest = &rest[2..];
            }
            let set_cubes = current_set.split(", ");
            for nb_and_color in set_cubes {
                let (nb, color) = nb_and_color.split_once(' ').unwrap();
                if nb.parse::<u32>().unwrap() > nbs_and_colors[color] {
                    respect_limit = false;
                }
            }
        }
        if respect_limit {
            sum_possible_game_ids += game_id.parse::<u32>().unwrap();
        }
    }
    println!("Result: {}", sum_possible_game_ids);
}

fn second_part() {
    let mut sum_sets = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let mut curr_power = 1;
        let mut rest: &str = &line.chars().skip_while(|&c| c != ':').skip(2).collect::<String>();
        let mut mini = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        while rest.chars().count() > 1 {
            let current_set;
            (current_set, rest) = rest.split_at(rest.find(';').unwrap_or(rest.chars().count()));
            if !rest.is_empty() {
                rest = &rest[2..];
            }
            let set_cubes = current_set.split(", ");
            for nb_and_color in set_cubes {
                let (nb, color) = nb_and_color.split_once(' ').unwrap();
                *mini.get_mut(color).unwrap() = cmp::max(*mini.get_mut(color).unwrap(),
                    nb.parse::<u32>().unwrap());
            }
        }
        mini.values().for_each(|v| curr_power *= v);
        sum_sets += curr_power;
    }
    println!("Result: {}", sum_sets);
}

fn main() {
    first_part();
    second_part();
}
