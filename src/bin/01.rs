use std::fs::read_to_string;

fn first_part(filename: String) {
    let mut sum_calibration_values = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let first_digit = line
            .chars()
            .by_ref()
            .skip_while(|&c| !c.is_digit(10))
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit_ref = line
            .chars()
            .rev()
            .by_ref()
            .skip_while(|&c| !c.is_digit(10))
            .next();
        let last_digit = match last_digit_ref {
            Some(value) => value.to_digit(10).unwrap(),
            None => first_digit,
        };
        sum_calibration_values += first_digit * 10 + last_digit;
    }
    println!("Result: {}", sum_calibration_values);
}


fn get_spelled_out_digit_or_value(s: &str, start_index: usize) -> (i32, usize) {
    let (mut value, mut index) = (-1, start_index);
    for i in index..s.chars().count() {
        let splice = &s[i..];
        if splice.starts_with("one")    { value = 1; }
        if splice.starts_with("two")    { value = 2; }
        if splice.starts_with("three")  { value = 3; }
        if splice.starts_with("four")   { value = 4; }
        if splice.starts_with("five")   { value = 5; }
        if splice.starts_with("six")    { value = 6; }
        if splice.starts_with("seven")  { value = 7; }
        if splice.starts_with("eight")  { value = 8; }
        if splice.starts_with("nine")   { value = 9; }
        else if s.chars().nth(i).unwrap().is_digit(10) {
            value = s.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        }
        if value != -1 {
            index = i;
            if start_index == 0 { break; }
        }
    }
    return (value, index);
}

fn second_part(filename: String) {
    let mut sum_calibration_values = 0;
    for line in read_to_string(filename).unwrap().lines() {

        let (first_digit, index) = get_spelled_out_digit_or_value(line, 0);
        let (mut last_digit, _) = get_spelled_out_digit_or_value(line, index + 1);
        if last_digit == -1 { last_digit = first_digit; }

        sum_calibration_values += first_digit * 10 + last_digit;

    }
    println!("Result: {}", sum_calibration_values);
}

fn main() {
    let cwd = format!("{}{}", std::env::current_dir().unwrap().display(), "/src/bin/");
    let filename = format!("{}01_{}", cwd, "input.txt");
    first_part(filename.clone());
    second_part(filename.clone());
}