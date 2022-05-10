use std::env;
use std::fs;

fn read_file(filename: &str) -> Vec<i128> {
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i128)
        .collect();
    return content;
}

fn adjacents(digits: &Vec<i128>) -> i128 {
    let mut max_sum: i128 = 0;
    for i in 0..digits.len() - 13 {
        let sum: i128 = digits[i..i + 13].iter().product();
        if sum > max_sum {
            max_sum = sum
        }
    }
    max_sum
}

pub fn solve_8_problem(filename: &str) -> i128 {
    let digits = read_file(filename);
    let result = adjacents(&digits);
    result
}
