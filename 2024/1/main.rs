extern crate regex;

use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn get_input_as_array(input: &str) -> Vec<String> {
    let newline_regex = Regex::new(r"\n").unwrap();
    let stripped_of_newlines = newline_regex.replace_all(input, ",");
    let triple_space_regex = Regex::new(r"   ").unwrap();
    let stripped_of_spaces = triple_space_regex
        .replace_all(stripped_of_newlines.as_ref(), ",")
        .into_owned();

    let split_input: Vec<String> = stripped_of_spaces.split(",").map(str::to_string).collect();
    return split_input;
}

fn main() {
    let mut first_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut second_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

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

    let mut total_difference: i32 = 0;

    for index in 0..first_heap.len() {
        let first_heap_value = first_heap.pop();
        let second_heap_value = second_heap.pop();

        match (first_heap_value, second_heap_value) {
            (Some(x), Some(y)) => {
                let Reverse(first_value) = x;
                let Reverse(second_value) = y;
                let difference = first_value - second_value;
                println!("first {:?}", first_value);
                println!("second {:?}", second_value);

                if difference < 0 {
                    total_difference += difference * -1;
                } else {
                    total_difference += difference;
                }

                println!("total  {}", total_difference);
            }
            _ => {
                println!("index {}", index);
                println!("first {:?}", first_heap_value);
                println!("second {:#?}", second_heap_value);
                panic!("One of the heaps is empty.");
            }
        }
    }
}
