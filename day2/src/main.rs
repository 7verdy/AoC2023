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
                if (nb.parse::<u32>().unwrap() > nbs_and_colors[color]) {
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
        let mut rest = line.chars().skip_while(|&c| c != ':').skip(2).collect::<String>();
        let mut mini = [ 0, 0, 0 ];


        while rest.chars().count() > 1 {
            let current_set = rest.chars().take_while(|&c| c != ';').collect::<String>();
            let mut nbs_and_colors = [ 0, 0, 0 ];
            rest = (&rest[0 + current_set.chars().count()..]).to_string().chars().skip(2).collect();
            let set_cubes = current_set.split(", ");
            for nb_and_color in set_cubes {
                let nb = nb_and_color.chars().take_while(|&c| c != ' ').collect::<String>().parse::<u32>().unwrap();
                let color = nb_and_color.chars().skip_while(|&c| c != ' ').skip(1).collect::<String>();
                match color.as_str() {
                    "red"   => nbs_and_colors[0] += nb,
                    "green" => nbs_and_colors[1] += nb,
                    "blue"  => nbs_and_colors[2] += nb,
                    _ => println!("Error!"),
                }
            }
            for i in 0..3 {
                if mini[i] < nbs_and_colors[i] || mini[i] == 0 {
                    mini[i] = nbs_and_colors[i];
                }
            }
        }
        sum_sets += mini[0] * mini[1] * mini[2];
    }
    println!("Result: {}", sum_sets);
}

fn main() {
    first_part();
    second_part();
}
