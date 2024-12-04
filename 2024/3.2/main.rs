extern crate regex;

use regex::Regex;
use std::fs;

fn perform_mul(operation: &str) -> i32 {
    let stripped_of_start: &str = &operation[4..operation.len()];
    let stripped_of_end: &str = &stripped_of_start[0..stripped_of_start.len() - 1];
    let numbers = stripped_of_end
        .split(",")
        .map(|value: &str| value.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    return numbers[0] * numbers[1];
}

fn main() {
    let raw: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = raw.as_str();
    let regex = Regex::new(r"mul\([0-9]*,[0-9]*\)|don't\(\)|do\(\)").unwrap();
    let operations: Vec<&str> = regex.find_iter(&input).map(|m| m.as_str()).collect();

    let mut sum: i32 = 0;
    let mut enabled = true;

    for operation in operations {
        match operation {
            _ if operation.contains("don't") => {
                enabled = false;
            }
            _ if operation.contains("do") => {
                enabled = true;
            }
            _ if operation.contains("mul") => {
                if enabled {
                    sum += perform_mul(&operation);
                }
            }
            _ => {
                panic!(
                    "Operation {} is not of valid type. Check regex for false matches.",
                    operation
                )
            }
        }
    }

    println!("{}", sum);
}
