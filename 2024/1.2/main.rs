extern crate regex;

use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

fn get_input_as_array(input: &str) -> Vec<String> {
    let newline_regex: Regex = Regex::new(r"\n").unwrap();
    let stripped_of_newlines = newline_regex.replace_all(input, ",");
    let triple_space_regex: Regex = Regex::new(r"   ").unwrap();
    let stripped_of_spaces: String = triple_space_regex
        .replace_all(stripped_of_newlines.as_ref(), ",")
        .into_owned();

    let split_input: Vec<String> = stripped_of_spaces.split(",").map(str::to_string).collect();
    return split_input;
}

fn main() {
    let mut first_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut second_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let mut second_heap_frequency_map: HashMap<i32, i32> = HashMap::new();

    let loaded_input: String =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let values: Vec<String> = get_input_as_array(&loaded_input);

    for (index, value) in values.into_iter().enumerate() {
        let parsed_value: i32 = value.parse().unwrap();

        if index % 2 == 0 {
            first_heap.push(Reverse(parsed_value));
        } else {
            second_heap.push(Reverse(parsed_value));
        }
    }

    if first_heap.len() != second_heap.len() {
        panic!("Heaps are not of equal length.")
    }

    for _ in 0..second_heap.len() {
        let second_heap_value: Option<Reverse<i32>> = second_heap.pop();

        match second_heap_value {
            Some(x) => {
                let Reverse(value) = x;
                let frequency = second_heap_frequency_map.get(&value).unwrap_or(&0);

                let new_value = frequency + 1;
                second_heap_frequency_map.insert(value, new_value);
            }
            _ => {
                panic!("Second heap value is empty.");
            }
        }
    }

    let mut similarity_score: i32 = 0;

    for _ in 0..first_heap.len() {
        let first_heap_value: Option<Reverse<i32>> = first_heap.pop();

        match first_heap_value {
            Some(x) => {
                let Reverse(value) = x;
                let frequency = second_heap_frequency_map.get(&value).unwrap_or(&0);

                let similarity = frequency * value;
                similarity_score += similarity;
            }
            _ => {
                panic!("First heap value is empty.");
            }
        }
    }

    println!("{}", similarity_score);
}
