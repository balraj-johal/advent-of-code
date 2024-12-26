extern crate regex;

use regex::Regex;
use std::fs;

fn main() {
    let raw: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = raw.as_str();
    let regex = Regex::new(r"mul\([0-9]*,[0-9]*\)").unwrap();
    let operations: Vec<&str> = regex.find_iter(&input).map(|m| m.as_str()).collect();

    let mut sum: i32 = 0;

    for operation in operations {
        let stripped_of_start: &str = &operation[4..operation.len()];
        let stripped_of_end: &str = &stripped_of_start[0..stripped_of_start.len() - 1];
        let numbers = stripped_of_end
            .split(",")
            .map(|value: &str| value.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        sum += numbers[0] * numbers[1];
    }

    println!("{}", sum);
}