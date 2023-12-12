use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;


fn first_part(filename: String) {
    let mut sum_possible_game_ids = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (game_id, sets) = line.split_once(": ").map(|(a, b)| {
            (a.chars().skip_while(|&c| !c.is_digit(10)).collect::<String>().to_owned(),
            b.split("; ").collect::<Vec<&str>>())
        }).unwrap();

        let cube_limits = HashMap::from([
            ("red", 12), ("green", 13), ("blue", 14),
        ]);
        let mut respect_limit = true;

        sets.iter().for_each(|set| {
            let set_cubes = set.split(", ");
            set_cubes.for_each(|nb_and_color| {
                let (nb, color) = nb_and_color.split_once(' ').unwrap();
                if nb.parse::<u32>().unwrap() > cube_limits[color] {
                    respect_limit = false;
                }
            });
        });

        if respect_limit {
            sum_possible_game_ids += game_id.parse::<u32>().unwrap();
        }
    }
    println!("The result of the first step is: {}", sum_possible_game_ids);
}


fn second_part(filename: String) {
    let mut sum_sets = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let (_, sets) = line.split_once(": ").map(|(a, b)| {
            (a, b.split("; ").collect::<Vec<&str>>())
        }).unwrap();

        let mut curr_power = 1;
        let mut mini = HashMap::from([
            ("red", 0), ("green", 0), ("blue", 0)
        ]);

        sets.iter().for_each(|set| {
            let set_cubes = set.split(", ");
            set_cubes.for_each(|nb_and_color| {
                let (nb, color) = nb_and_color.split_once(' ').unwrap();
                mini.insert(color, cmp::max(mini[color], nb.parse::<u32>().unwrap()));
            });
        });

        mini.values().for_each(|v| curr_power *= v);
        sum_sets += curr_power;
    }
    println!("The result of the second step is: {}", sum_sets);
}

fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}02_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}
