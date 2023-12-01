use std::fs::read_to_string;

fn main() {
    let mut sum_calibration_values = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
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
